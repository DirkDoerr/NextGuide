//! Private Preserve-through bags and v2 load/save codec.
//!
//! Layout follows The Guide 2.0 `guide.c` (`guide_store` / `guide_load_v2`).

use std::collections::HashMap;
use std::io::Write;

use crate::error::Error;
use crate::model::{Guide, Node};
use crate::Result;

const MAGIC: &[u8; 3] = b"GDE";
const FORMAT_V2: u32 = 2;
const V1_DUMMY_MARKER: &[u8] = b"\0\0\0\0dummy\0dummy\0";

/// Opaque file-level state kept across load/save.
#[derive(Debug, Clone, Default)]
pub(crate) struct PreserveBag {
    pub(crate) file_attrs: Vec<Attr>,
    /// Selected node fake id after remapping (0 = none).
    pub(crate) sel_node_id: u32,
}

/// Opaque node-level attributes kept across load/save.
#[derive(Debug, Clone, Default)]
pub(crate) struct NodePreserve {
    pub(crate) attrs: Vec<Attr>,
    pub(crate) node_id: u32,
}

/// One length-prefixed attribute (file or node).
#[derive(Debug, Clone)]
pub(crate) struct Attr {
    pub(crate) id: u32,
    pub(crate) value: Vec<u8>,
}

struct Cursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        if self.remaining() < n {
            return Err(Error::Corrupt(format!(
                "truncated: need {n} bytes at offset {}, have {}",
                self.pos,
                self.remaining()
            )));
        }
        let slice = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Ok(slice)
    }

    fn u32_le(&mut self) -> Result<u32> {
        let bytes = self.take(4)?;
        Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
    }
}

struct RawNode {
    node_id: u32,
    parent_id: u32,
    attrs: Vec<Attr>,
    title: String,
    rtf_body: Vec<u8>,
}

/// Load a v2 `.gde` buffer into a Core-model Guide.
pub(crate) fn load(bytes: &[u8]) -> Result<Guide> {
    if bytes.len() < 20 {
        return Err(Error::Corrupt(
            "file too short to be a valid .gde".into(),
        ));
    }

    if bytes.len() >= 20 && &bytes[4..20] == V1_DUMMY_MARKER {
        return Err(Error::Unsupported(
            "The Guide v1 .gde is not supported by Format v1".into(),
        ));
    }

    if bytes.len() < 7 || &bytes[0..3] != MAGIC {
        return Err(Error::Corrupt("missing GDE magic".into()));
    }

    let version = u32::from_le_bytes(bytes[3..7].try_into().unwrap());
    if version != FORMAT_V2 {
        return Err(Error::Unsupported(format!(
            "format version {version} is not supported (need v2)"
        )));
    }

    let mut cur = Cursor::new(bytes);
    cur.take(3)?; // magic
    let _version = cur.u32_le()?;

    let file_attr_count = cur.u32_le()?;
    let mut file_attrs = Vec::with_capacity(file_attr_count as usize);
    let mut sel_raw: u32 = 0;
    for _ in 0..file_attr_count {
        let id = cur.u32_le()?;
        let len = cur.u32_le()? as usize;
        let value = cur.take(len)?.to_vec();
        if id == 2 && value.len() >= 4 {
            sel_raw = u32::from_le_bytes(value[..4].try_into().unwrap());
        }
        file_attrs.push(Attr { id, value });
    }

    let mut raw_nodes = Vec::new();
    while cur.remaining() > 0 {
        raw_nodes.push(read_node(&mut cur)?);
    }

    if raw_nodes.is_empty() {
        return Err(Error::Corrupt("no nodes in guide".into()));
    }

    // Remap fake pointer ids to dense preorder ids starting at 1.
    let mut id_map: HashMap<u32, u32> = HashMap::new();
    for (i, raw) in raw_nodes.iter().enumerate() {
        id_map.insert(raw.node_id, (i as u32) + 1);
    }

    let root_raw = &raw_nodes[0];
    if root_raw.parent_id != 0 {
        return Err(Error::Corrupt(
            "first node is not a Dummy root (parent != 0)".into(),
        ));
    }

    let mut children_of: HashMap<u32, Vec<usize>> = HashMap::new();
    for (i, raw) in raw_nodes.iter().enumerate().skip(1) {
        if !id_map.contains_key(&raw.parent_id) {
            return Err(Error::Corrupt(format!(
                "node {:?} references unknown parent id {}",
                raw.title, raw.parent_id
            )));
        }
        children_of.entry(raw.parent_id).or_default().push(i);
    }

    fn build_node(
        index: usize,
        raw_nodes: &[RawNode],
        children_of: &HashMap<u32, Vec<usize>>,
        id_map: &HashMap<u32, u32>,
    ) -> Node {
        let raw = &raw_nodes[index];
        let child_idxs = children_of.get(&raw.node_id).into_iter().flatten();
        let children = child_idxs
            .map(|&ci| build_node(ci, raw_nodes, children_of, id_map))
            .collect();
        Node {
            title: raw.title.clone(),
            rtf_body: raw.rtf_body.clone(),
            children,
            preserve: NodePreserve {
                attrs: raw.attrs.clone(),
                node_id: id_map[&raw.node_id],
            },
        }
    }

    let root = build_node(0, &raw_nodes, &children_of, &id_map);
    let sel_node_id = if sel_raw == 0 {
        0
    } else {
        id_map.get(&sel_raw).copied().unwrap_or(0)
    };

    // Rewrite file attr 2 (sel_node) to remapped id for Preserve-through writes.
    let mut rewritten_file_attrs = file_attrs;
    for attr in &mut rewritten_file_attrs {
        if attr.id == 2 {
            attr.value = sel_node_id.to_le_bytes().to_vec();
        }
    }

    Ok(Guide {
        root,
        preserve: PreserveBag {
            file_attrs: rewritten_file_attrs,
            sel_node_id,
        },
    })
}

fn read_node(cur: &mut Cursor<'_>) -> Result<RawNode> {
    let node_id = cur.u32_le()?;
    let parent_id = cur.u32_le()?;
    let n_attrs = cur.u32_le()?;
    let mut attrs = Vec::with_capacity(n_attrs as usize);
    for _ in 0..n_attrs {
        let id = cur.u32_le()?;
        let len = cur.u32_le()? as usize;
        let value = cur.take(len)?.to_vec();
        attrs.push(Attr { id, value });
    }
    let title_len = cur.u32_le()? as usize;
    let title_bytes = cur.take(title_len)?;
    let title = match std::str::from_utf8(title_bytes) {
        Ok(s) => s.to_string(),
        Err(_) => {
            // The Guide stores titles as UTF-8; treat invalid sequences as corrupt.
            return Err(Error::Corrupt(format!(
                "node title is not valid UTF-8 (id {node_id})"
            )));
        }
    };
    let text_len = cur.u32_le()? as usize;
    let rtf_body = cur.take(text_len)?.to_vec();
    Ok(RawNode {
        node_id,
        parent_id,
        attrs,
        title,
        rtf_body,
    })
}

/// Save a Guide to a v2 `.gde` buffer (canonical writer).
pub(crate) fn save(guide: &Guide) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    out.write_all(MAGIC).unwrap();
    out.extend_from_slice(&FORMAT_V2.to_le_bytes());

    let file_attrs = if guide.preserve.file_attrs.is_empty() {
        default_file_attrs(guide.preserve.sel_node_id)
    } else {
        guide.preserve.file_attrs.clone()
    };

    write_u32(&mut out, file_attrs.len() as u32);
    for attr in &file_attrs {
        write_attr(&mut out, attr);
    }

    // Assign dense preorder ids starting at 1, preserving relative order.
    let mut next_id = 1u32;
    write_node_preorder(&mut out, &guide.root, 0, &mut next_id)?;

    Ok(out)
}

fn default_file_attrs(sel_node_id: u32) -> Vec<Attr> {
    vec![
        Attr {
            id: 1,
            value: 0u32.to_le_bytes().to_vec(),
        },
        Attr {
            id: 2,
            value: sel_node_id.to_le_bytes().to_vec(),
        },
    ]
}

fn write_node_preorder(
    out: &mut Vec<u8>,
    node: &Node,
    parent_id: u32,
    next_id: &mut u32,
) -> Result<()> {
    let node_id = if node.preserve.node_id != 0 {
        node.preserve.node_id
    } else {
        let id = *next_id;
        *next_id += 1;
        id
    };
    // Keep next_id ahead of any preserved ids we encounter.
    if node_id >= *next_id {
        *next_id = node_id + 1;
    }

    write_u32(out, node_id);
    write_u32(out, parent_id);

    let attrs = if node.preserve.attrs.is_empty() {
        default_node_attrs()
    } else {
        node.preserve.attrs.clone()
    };
    write_u32(out, attrs.len() as u32);
    for attr in &attrs {
        write_attr(out, attr);
    }

    let title_bytes = node.title.as_bytes();
    write_u32(out, title_bytes.len() as u32);
    out.extend_from_slice(title_bytes);

    write_u32(out, node.rtf_body.len() as u32);
    out.extend_from_slice(&node.rtf_body);

    for child in &node.children {
        write_node_preorder(out, child, node_id, next_id)?;
    }
    Ok(())
}

fn default_node_attrs() -> Vec<Attr> {
    // Match The Guide writer defaults for a blank node (7 uint32 attrs).
    (1..=7)
        .map(|id| {
            let value = match id {
                4 | 5 => 0xffff_ffffu32, // color / bgcolor default in samples
                _ => 0u32,
            };
            Attr {
                id,
                value: value.to_le_bytes().to_vec(),
            }
        })
        .collect()
}

fn write_attr(out: &mut Vec<u8>, attr: &Attr) {
    write_u32(out, attr.id);
    write_u32(out, attr.value.len() as u32);
    out.extend_from_slice(&attr.value);
}

fn write_u32(out: &mut Vec<u8>, v: u32) {
    out.extend_from_slice(&v.to_le_bytes());
}
