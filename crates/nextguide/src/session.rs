//! Opened Guide session helpers (Dummy-root-hidden tree + body view + save).

use std::path::{Path, PathBuf};

use nextguide_gde::{Guide, Node, NodePath, Result};
use nextguide_rtf::{load as load_rtf, LoadBody};

/// One visible tree row (Dummy root never appears).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeRow {
    /// Library Node path (includes Dummy root index, e.g. first top row is `[0]`).
    pub path: NodePath,
    /// Node title shown in the tree.
    pub title: String,
    /// Nested visible children.
    pub children: Vec<TreeRow>,
}

/// Body pane contents for the current selection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BodyView {
    /// Text to show in the body pane (subset plain rendering or soft-fail fallback).
    pub display_text: String,
    /// True when RTF could not be parsed into the edit subset.
    pub soft_fail: bool,
}

/// An opened Guide plus its on-disk path (if any).
#[derive(Clone, Debug)]
pub struct EditorDoc {
    guide: Guide,
    path: Option<PathBuf>,
}

impl EditorDoc {
    /// Open a `.gde` from disk.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let guide = nextguide_gde::load_path(path)?;
        Ok(Self {
            guide,
            path: Some(path.to_path_buf()),
        })
    }

    /// In-memory Guide.
    pub fn guide(&self) -> &Guide {
        &self.guide
    }

    /// Path the Guide was opened from (`None` if untitled).
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    /// Visible tree: Dummy root’s children as top-level rows.
    pub fn visible_tree(&self) -> Vec<TreeRow> {
        visible_tree(&self.guide)
    }

    /// Body view for a selected Node path (original RTF bytes stay on the Guide).
    pub fn body_view(&self, path: &NodePath) -> Result<BodyView> {
        body_view(&self.guide, path)
    }

    /// Save the Guide to its path without rewriting any RTF bodies.
    ///
    /// Untouched bodies keep their original opaque bytes; Structural Round-trip
    /// honesty is preserved for an unmodified open→save cycle.
    pub fn save_unchanged(&self) -> Result<()> {
        let path = self.path.as_ref().ok_or_else(|| {
            nextguide_gde::Error::InvalidEdit("save unchanged requires an opened path".into())
        })?;
        nextguide_gde::save_path(&self.guide, path)
    }

    /// Save to an explicit path without rewriting RTF bodies (used by tests / Save As later).
    pub fn save_unchanged_to(&self, path: impl AsRef<Path>) -> Result<()> {
        nextguide_gde::save_path(&self.guide, path)
    }
}

/// Build the Dummy-root-hidden tree for a Guide.
pub fn visible_tree(guide: &Guide) -> Vec<TreeRow> {
    guide
        .root()
        .children()
        .iter()
        .enumerate()
        .map(|(i, node)| row_from_node(NodePath::from_indices(vec![i]), node))
        .collect()
}

fn row_from_node(path: NodePath, node: &Node) -> TreeRow {
    let children = node
        .children()
        .iter()
        .enumerate()
        .map(|(i, child)| row_from_node(path.child(i), child))
        .collect();
    TreeRow {
        path,
        title: node.title().to_string(),
        children,
    }
}

/// Display text for a Node body via `nextguide-rtf` (does not mutate the Guide).
pub fn body_view(guide: &Guide, path: &NodePath) -> Result<BodyView> {
    let node = guide.node(path)?;
    Ok(body_view_from_rtf(node.rtf_body()))
}

/// Map opaque RTF bytes to body-pane display (soft-fail keeps bytes with the Guide).
pub fn body_view_from_rtf(rtf: &[u8]) -> BodyView {
    match load_rtf(rtf) {
        LoadBody::Editable(doc) => BodyView {
            display_text: document_to_display_text(&doc),
            soft_fail: false,
        },
        LoadBody::Unparseable { plain_fallback } => BodyView {
            display_text: plain_fallback,
            soft_fail: true,
        },
    }
}

fn document_to_display_text(doc: &nextguide_rtf::Document) -> String {
    let mut out = String::new();
    let mut number = 1u32;
    for (i, block) in doc.blocks.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        match block {
            nextguide_rtf::Block::Paragraph(runs) => {
                append_runs(&mut out, runs);
            }
            nextguide_rtf::Block::BulletItem(runs) => {
                out.push_str("• ");
                append_runs(&mut out, runs);
            }
            nextguide_rtf::Block::NumberedItem(runs) => {
                out.push_str(&format!("{number}. "));
                append_runs(&mut out, runs);
                number += 1;
            }
        }
    }
    out
}

fn append_runs(out: &mut String, runs: &[nextguide_rtf::Run]) {
    for run in runs {
        out.push_str(&run.text);
    }
}
