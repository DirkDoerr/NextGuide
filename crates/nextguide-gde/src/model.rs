//! Core-model Guide and Node types, including mutation ops.

use std::fmt;

use crate::codec::{NodePreserve, PreserveBag};
use crate::error::Error;
use crate::path::NodePath;
use crate::Result;

/// In-memory Guide: ordered tree rooted at the Dummy root.
#[derive(Clone)]
pub struct Guide {
    pub(crate) root: Node,
    pub(crate) preserve: PreserveBag,
}

impl Guide {
    /// New Guide with only a Dummy root (no user Nodes).
    pub fn new_empty() -> Self {
        Self {
            root: Node {
                title: "dummy".into(),
                rtf_body: b"dummy".to_vec(),
                children: Vec::new(),
                preserve: NodePreserve::default(),
            },
            preserve: PreserveBag::default(),
        }
    }

    /// Dummy root of the on-disk tree.
    pub fn root(&self) -> &Node {
        &self.root
    }

    /// Look up a Node by path (Dummy root at `[]`).
    pub fn node(&self, path: &NodePath) -> Result<&Node> {
        Self::resolve(&self.root, path.indices())
    }

    /// Set the title of the Node at `path`.
    pub fn set_title(&mut self, path: &NodePath, title: impl Into<String>) -> Result<()> {
        let node = Self::resolve_mut(&mut self.root, path.indices())?;
        node.title = title.into();
        Ok(())
    }

    /// Set the opaque RTF body of the Node at `path`.
    pub fn set_rtf_body(&mut self, path: &NodePath, rtf_body: impl Into<Vec<u8>>) -> Result<()> {
        let node = Self::resolve_mut(&mut self.root, path.indices())?;
        node.rtf_body = rtf_body.into();
        Ok(())
    }

    /// Insert a new empty child under `parent` at `index` (`len` = append).
    ///
    /// Returns the path of the inserted Node. The new Node has an empty title,
    /// empty RTF body, no children, and an empty Preserve-through bag.
    pub fn insert_child(&mut self, parent: &NodePath, index: usize) -> Result<NodePath> {
        let parent_node = Self::resolve_mut(&mut self.root, parent.indices())?;
        let len = parent_node.children.len();
        if index > len {
            return Err(Error::InvalidEdit(format!(
                "insert index {index} out of range 0..={len} under {parent}"
            )));
        }
        parent_node.children.insert(index, Node::new_empty_child());
        Ok(parent.child(index))
    }

    /// Remove the Node at `path` and its subtree. Refuses the Dummy root.
    pub fn remove_node(&mut self, path: &NodePath) -> Result<()> {
        if path.is_root() {
            return Err(Error::InvalidEdit(
                "cannot remove the Dummy root".into(),
            ));
        }
        // Validate the target exists before mutating.
        let _ = Self::resolve(&self.root, path.indices())?;
        let parent_path = path.parent().expect("non-root has parent");
        let index = path.last_index().expect("non-root has index");
        let parent = Self::resolve_mut(&mut self.root, parent_path.indices())?;
        if index >= parent.children.len() {
            return Err(Error::InvalidPath(format!(
                "no Node at {path}"
            )));
        }
        parent.children.remove(index);
        Ok(())
    }

    /// Move the Node at `from` under `to_parent` at `index` (`len` = append).
    ///
    /// Same-parent moves adjust `index` after removal when the source index is
    /// less than the destination. Refuses moving the Dummy root or into its
    /// own descendant subtree.
    pub fn move_node(
        &mut self,
        from: &NodePath,
        to_parent: &NodePath,
        index: usize,
    ) -> Result<NodePath> {
        if from.is_root() {
            return Err(Error::InvalidEdit(
                "cannot reparent the Dummy root".into(),
            ));
        }
        if Self::is_prefix(from.indices(), to_parent.indices()) {
            return Err(Error::InvalidEdit(format!(
                "cannot move {from} under its descendant {to_parent}"
            )));
        }

        // Validate both endpoints exist and index is in range *before* extract.
        let _ = Self::resolve(&self.root, from.indices())?;
        let dest_parent = Self::resolve(&self.root, to_parent.indices())?;
        let dest_len = dest_parent.children.len();
        if index > dest_len {
            return Err(Error::InvalidEdit(format!(
                "move index {index} out of range 0..={dest_len} under {to_parent}"
            )));
        }

        let from_parent = from.parent().expect("non-root has parent");
        let from_index = from.last_index().expect("non-root has index");

        let same_parent = from_parent == *to_parent;
        let mut insert_at = index;
        if same_parent && from_index < insert_at {
            insert_at -= 1;
        }

        // Mutate a clone so a failed second step cannot leave a half-updated Guide.
        let mut next = self.clone();
        let node = {
            let parent = Self::resolve_mut(&mut next.root, from_parent.indices())?;
            parent.children.remove(from_index)
        };
        let parent = Self::resolve_mut(&mut next.root, to_parent.indices())?;
        if insert_at > parent.children.len() {
            return Err(Error::InvalidEdit(format!(
                "move index {insert_at} out of range after extract under {to_parent}"
            )));
        }
        parent.children.insert(insert_at, node);
        *self = next;
        Ok(to_parent.child(insert_at))
    }

    fn resolve<'a>(root: &'a Node, indices: &[usize]) -> Result<&'a Node> {
        let mut cur = root;
        for (depth, &idx) in indices.iter().enumerate() {
            if idx >= cur.children.len() {
                return Err(Error::InvalidPath(format!(
                    "no Node at [{}] (index {idx} out of range at depth {depth})",
                    indices
                        .iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )));
            }
            cur = &cur.children[idx];
        }
        Ok(cur)
    }

    fn resolve_mut<'a>(root: &'a mut Node, indices: &[usize]) -> Result<&'a mut Node> {
        let mut cur = root;
        for (depth, &idx) in indices.iter().enumerate() {
            if idx >= cur.children.len() {
                return Err(Error::InvalidPath(format!(
                    "no Node at [{}] (index {idx} out of range at depth {depth})",
                    indices
                        .iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )));
            }
            cur = &mut cur.children[idx];
        }
        Ok(cur)
    }

    /// True when `prefix` equals `path` or is a strict ancestor path of `path`.
    fn is_prefix(prefix: &[usize], path: &[usize]) -> bool {
        path.len() >= prefix.len() && path[..prefix.len()] == *prefix
    }
}

impl fmt::Debug for Guide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Omit Preserve-through bags from Debug (not part of the Core-model API).
        f.debug_struct("Guide").field("root", &self.root).finish()
    }
}

/// One Node in the Core-model tree.
#[derive(Clone)]
pub struct Node {
    pub(crate) title: String,
    pub(crate) rtf_body: Vec<u8>,
    pub(crate) children: Vec<Node>,
    pub(crate) preserve: NodePreserve,
}

impl Node {
    fn new_empty_child() -> Self {
        Self {
            title: String::new(),
            rtf_body: Vec::new(),
            children: Vec::new(),
            preserve: NodePreserve::default(),
        }
    }

    /// Node title (UTF-8).
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Opaque RTF body bytes.
    pub fn rtf_body(&self) -> &[u8] {
        &self.rtf_body
    }

    /// Ordered children.
    pub fn children(&self) -> &[Node] {
        &self.children
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("title", &self.title)
            .field("rtf_body_len", &self.rtf_body.len())
            .field("children", &self.children)
            .finish()
    }
}
