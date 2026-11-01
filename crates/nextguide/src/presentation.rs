//! GTK-free toolbar presentation. The shell reports facts; this module decides
//! what the window should show.

/// Facts the shell has already read from the open Guide. No widget types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EditorSnapshot {
    /// A Guide is open in this window.
    pub guide_open: bool,
    /// The open Guide has unsaved edits.
    pub dirty: bool,
    /// At least one visible Node. The Dummy root does not count.
    pub has_nodes: bool,
    /// A visible Node is selected. The Dummy root is never a selection.
    pub node_selected: bool,
}

/// What the shell should render for the current snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Presentation {
    /// Save can be activated. Grey when false.
    pub save_enabled: bool,
    /// Body text when the Guide has no Nodes. `None` leaves any other empty body blank.
    pub empty_hint: Option<&'static str>,
    /// Bottom-line sentence to publish. `None` leaves the line unchanged.
    pub status_sentence: Option<String>,
    /// Insert Child. Available whenever a Guide is open, including when nothing is selected.
    pub insert_child_enabled: bool,
    /// Insert Sibling. Grey when no Node is selected.
    pub insert_sibling_enabled: bool,
    /// Delete. Grey when no Node is selected.
    pub delete_enabled: bool,
    /// Move Up. Grey when no Node is selected.
    pub move_up_enabled: bool,
    /// Move Down. Grey when no Node is selected.
    pub move_down_enabled: bool,
    /// Indent. Grey when no Node is selected.
    pub indent_enabled: bool,
    /// Outdent. Grey when no Node is selected.
    pub outdent_enabled: bool,
}

/// Result of a command the shell just ran, when the bottom line might change.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActionOutcome {
    /// The command finished. Publish no sentence.
    Success,
    /// Save did not complete. `detail` is the error text the save path already has.
    SaveFailed { detail: String },
    /// There is no Guide to write.
    NothingToSave,
    /// A structure command was refused. The sentence is the one Editor v1 already shows.
    StructureRefused(StructureRefusal),
    /// Insert Child failed. `detail` is the error the edit path already produced.
    InsertChildFailed { detail: String },
    /// Insert Sibling failed. `detail` is the error the edit path already produced.
    InsertSiblingFailed { detail: String },
    /// Delete failed. `detail` is the error the edit path already produced.
    DeleteFailed { detail: String },
    /// Move Up or Move Down failed. `detail` is the error the edit path already produced.
    ReorderFailed { detail: String },
    /// Indent failed. `detail` is the error the edit path already produced.
    IndentFailed { detail: String },
    /// Outdent failed. `detail` is the error the edit path already produced.
    OutdentFailed { detail: String },
}

/// Why a structure command did not change the Guide.
///
/// Each sentence is the one Editor v1 already shows for that refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StructureRefusal {
    /// No Guide is open.
    NoGuide,
    /// Insert Sibling with nothing selected.
    SelectNodeToInsertSibling,
    /// The Dummy root has no sibling.
    CannotInsertSiblingOfDummyRoot,
    /// The selected path has no parent.
    InsertSiblingNoParent,
    /// The selected path has no index.
    InsertSiblingNoIndex,
    /// Delete with nothing selected.
    SelectNodeToDelete,
    /// Delete targeted the Dummy root.
    CannotDeleteDummyRoot,
    /// Move Up or Move Down with nothing selected.
    SelectNodeToReorder,
    /// Move Up or Move Down targeted the Dummy root.
    CannotReorderDummyRoot,
    /// The selected path has no index to reorder.
    ReorderNoIndex,
    /// Move Up or Move Down when the Node is already first or last among siblings.
    AlreadyAtEdgeOfSiblings,
    /// Indent with nothing selected.
    SelectNodeToIndent,
    /// Indent targeted the Dummy root.
    CannotIndentDummyRoot,
    /// The selected path has no index to indent.
    IndentNoIndex,
    /// Indent when the Node has no previous sibling to become its parent.
    IndentNeedsPreviousSibling,
    /// Outdent with nothing selected.
    SelectNodeToOutdent,
    /// Outdent targeted the Dummy root.
    CannotOutdentDummyRoot,
    /// Outdent when the Node is already a child of the Dummy root.
    AlreadyAtTopLevel,
    /// The parent path has no index to outdent under.
    OutdentNoParentIndex,
}

/// Decide Save’s enabled state, structure-control enablement, the empty-Guide
/// hint, and the bottom line.
///
/// Save is available only while an open Guide is dirty. An untitled Guide has
/// no path; dirtiness alone makes Save available. Insert Child is available
/// whenever a Guide is open, including when nothing is selected. The other
/// structure controls are grey until a Node is selected, and stay available
/// then even if the move will be refused. A success publishes nothing.
pub fn present(snapshot: &EditorSnapshot, outcome: Option<ActionOutcome>) -> Presentation {
    Presentation {
        save_enabled: snapshot.guide_open && snapshot.dirty,
        empty_hint: if snapshot.guide_open && !snapshot.has_nodes {
            Some("Untitled Guide — add a Node to begin.")
        } else {
            None
        },
        status_sentence: status_sentence(outcome),
        insert_child_enabled: snapshot.guide_open,
        // A selected Node keeps these clickable even when the move will be refused.
        insert_sibling_enabled: snapshot.node_selected,
        delete_enabled: snapshot.node_selected,
        move_up_enabled: snapshot.node_selected,
        move_down_enabled: snapshot.node_selected,
        indent_enabled: snapshot.node_selected,
        outdent_enabled: snapshot.node_selected,
    }
}

fn status_sentence(outcome: Option<ActionOutcome>) -> Option<String> {
    match outcome {
        Some(ActionOutcome::SaveFailed { detail }) => Some(format!("Save failed: {detail}")),
        Some(ActionOutcome::NothingToSave) => Some("Nothing to save.".to_string()),
        Some(ActionOutcome::StructureRefused(refusal)) => Some(refusal.sentence().to_string()),
        Some(ActionOutcome::InsertChildFailed { detail }) => {
            Some(format!("Insert failed: {detail}"))
        }
        Some(ActionOutcome::InsertSiblingFailed { detail }) => {
            Some(format!("Insert sibling failed: {detail}"))
        }
        Some(ActionOutcome::DeleteFailed { detail }) => Some(format!("Delete failed: {detail}")),
        Some(ActionOutcome::ReorderFailed { detail }) => Some(format!("Reorder failed: {detail}")),
        Some(ActionOutcome::IndentFailed { detail }) => Some(format!("Indent failed: {detail}")),
        Some(ActionOutcome::OutdentFailed { detail }) => Some(format!("Outdent failed: {detail}")),
        Some(ActionOutcome::Success) | None => None,
    }
}

impl StructureRefusal {
    fn sentence(self) -> &'static str {
        match self {
            Self::NoGuide => "Open a Guide before restructuring.",
            Self::SelectNodeToInsertSibling => "Select a Node to insert a sibling.",
            Self::CannotInsertSiblingOfDummyRoot => "Cannot insert a sibling of the Dummy root.",
            Self::InsertSiblingNoParent => "Insert sibling failed: no parent.",
            Self::InsertSiblingNoIndex => "Insert sibling failed: no index.",
            Self::SelectNodeToDelete => "Select a Node to delete.",
            Self::CannotDeleteDummyRoot => "Cannot delete the Dummy root.",
            Self::SelectNodeToReorder => "Select a Node to reorder.",
            Self::CannotReorderDummyRoot => "Cannot reorder the Dummy root.",
            Self::ReorderNoIndex => "Reorder failed: no index.",
            Self::AlreadyAtEdgeOfSiblings => "Already at edge of siblings.",
            Self::SelectNodeToIndent => "Select a Node to indent.",
            Self::CannotIndentDummyRoot => "Cannot indent the Dummy root.",
            Self::IndentNoIndex => "Indent failed: no index.",
            Self::IndentNeedsPreviousSibling => "Indent needs a previous sibling.",
            Self::SelectNodeToOutdent => "Select a Node to outdent.",
            Self::CannotOutdentDummyRoot => "Cannot outdent the Dummy root.",
            Self::AlreadyAtTopLevel => "Already at top level.",
            Self::OutdentNoParentIndex => "Outdent failed: no parent index.",
        }
    }
}
