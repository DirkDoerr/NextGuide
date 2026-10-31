//! Opened Guide session helpers (Dummy-root-hidden tree + body view + edit/restructure/save).

use std::path::{Path, PathBuf};

use nextguide_gde::{Guide, Node, NodePath, Result};
use nextguide_rtf::{bytes_for_save, empty_rtf, load as load_rtf, Document, LoadBody};

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

/// An opened Guide plus its on-disk path (if any) and dirty state.
#[derive(Clone, Debug)]
pub struct EditorDoc {
    guide: Guide,
    path: Option<PathBuf>,
    dirty: bool,
}

impl EditorDoc {
    /// Untitled New Guide (Dummy root only; empty visible tree).
    pub fn new() -> Self {
        Self {
            guide: Guide::new_empty(),
            path: None,
            dirty: false,
        }
    }

    /// Open a `.gde` from disk.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let guide = nextguide_gde::load_path(path)?;
        Ok(Self {
            guide,
            path: Some(path.to_path_buf()),
            dirty: false,
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

    /// Whether the in-memory Guide has unsaved Core-model or body edits.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Visible tree: Dummy root’s children as top-level rows.
    pub fn visible_tree(&self) -> Vec<TreeRow> {
        visible_tree(&self.guide)
    }

    /// Body view for a selected Node path (original RTF bytes stay on the Guide).
    pub fn body_view(&self, path: &NodePath) -> Result<BodyView> {
        body_view(&self.guide, path)
    }

    /// Rename the Node at `path` via the Core-model mutation API.
    pub fn set_title(&mut self, path: &NodePath, title: impl Into<String>) -> Result<()> {
        self.guide.set_title(path, title)?;
        self.dirty = true;
        Ok(())
    }

    /// Replace the Node body with a subset rewrite from `doc`.
    ///
    /// Only this Node’s RTF bytes change; siblings keep their original opaque
    /// payloads. Soft-fail / exotic markup is intentionally replaced once the
    /// caller supplies an edit Document.
    pub fn set_body(&mut self, path: &NodePath, doc: &Document) -> Result<()> {
        let original = self.guide.node(path)?.rtf_body().to_vec();
        let bytes = bytes_for_save(&original, Some(doc));
        self.guide.set_rtf_body(path, bytes)?;
        self.dirty = true;
        Ok(())
    }

    /// Insert a new empty child under `parent` at `index` (`len` = append).
    ///
    /// New Nodes get an empty title, minimal valid empty RTF (`nextguide-rtf`),
    /// no children, and an empty Preserve-through bag (library default).
    pub fn insert_child(&mut self, parent: &NodePath, index: usize) -> Result<NodePath> {
        let path = self.guide.insert_child(parent, index)?;
        if let Err(err) = self.guide.set_rtf_body(&path, empty_rtf()) {
            let _ = self.guide.remove_node(&path);
            return Err(err);
        }
        self.dirty = true;
        Ok(path)
    }

    /// Remove the Node at `path` and its subtree. Refuses the Dummy root.
    ///
    /// Preserve-through bags for the removed subtree are dropped with it.
    pub fn remove_node(&mut self, path: &NodePath) -> Result<()> {
        self.guide.remove_node(path)?;
        self.dirty = true;
        Ok(())
    }

    /// Move the Node at `from` under `to_parent` at `index` (`len` = append).
    ///
    /// Preserve-through bags move with the Node. Same-parent moves adjust
    /// `index` after removal when needed (library behaviour).
    pub fn move_node(
        &mut self,
        from: &NodePath,
        to_parent: &NodePath,
        index: usize,
    ) -> Result<NodePath> {
        let path = self.guide.move_node(from, to_parent, index)?;
        self.dirty = true;
        Ok(path)
    }

    /// Persist the Guide to its opened path and clear dirty.
    pub fn save(&mut self) -> Result<()> {
        let path = self.path.clone().ok_or_else(|| {
            nextguide_gde::Error::InvalidEdit("save requires an opened path".into())
        })?;
        self.save_to(path)
    }

    /// Persist the Guide to `path`, bind the session to that path, and clear dirty.
    ///
    /// Untouched bodies keep original opaque bytes; bodies updated via
    /// [`set_body`] are already subset-rewritten on the Guide.
    pub fn save_as(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        nextguide_gde::save_path(&self.guide, path)?;
        self.path = Some(path.to_path_buf());
        self.dirty = false;
        Ok(())
    }

    /// Persist the Guide to `path` and clear dirty without rebinding the session path.
    ///
    /// Prefer [`save_as`] for Save As / first save of an untitled Guide.
    pub fn save_to(&mut self, path: impl AsRef<Path>) -> Result<()> {
        nextguide_gde::save_path(&self.guide, path)?;
        self.dirty = false;
        Ok(())
    }

    /// Save without requiring `&mut` when the Guide is known clean (open→save).
    ///
    /// Prefer [`save`] / [`save_to`] after edits so dirty state clears.
    pub fn save_unchanged(&self) -> Result<()> {
        let path = self.path.as_ref().ok_or_else(|| {
            nextguide_gde::Error::InvalidEdit("save unchanged requires an opened path".into())
        })?;
        nextguide_gde::save_path(&self.guide, path)
    }

    /// Save to an explicit path without clearing dirty (tests / Save As staging).
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

fn document_to_display_text(doc: &Document) -> String {
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
