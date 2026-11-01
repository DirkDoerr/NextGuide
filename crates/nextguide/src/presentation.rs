//! GTK-free toolbar presentation. The shell reports facts; this module decides
//! what the window should show.

/// Pane whose undo and redo stacks the toolbar is showing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FocusedPane {
    /// Tree and title history.
    Tree,
    /// Body buffer history.
    Body,
}

/// Facts the shell has already read from the open Guide. No widget types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EditorSnapshot {
    /// A Guide is open in this window.
    pub guide_open: bool,
    /// The open Guide has unsaved edits.
    pub dirty: bool,
    /// At least one visible Node. The Dummy root does not count.
    pub has_nodes: bool,
    /// Pane focused before a toolbar click. Undo and Redo read this pane’s stacks.
    pub focused_pane: FocusedPane,
    /// The body pane’s undo stack has no steps.
    pub body_undo_empty: bool,
    /// The tree pane’s undo stack has no steps.
    pub tree_undo_empty: bool,
    /// The body pane’s redo stack has no steps.
    pub body_redo_empty: bool,
    /// The tree pane’s redo stack has no steps.
    pub tree_redo_empty: bool,
}

/// What the shell should render for the current snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Presentation {
    /// Save can be activated. Grey when false.
    pub save_enabled: bool,
    /// Undo can be activated. Grey when the focused pane’s undo stack is empty.
    pub undo_enabled: bool,
    /// Redo can be activated. Grey when the focused pane’s redo stack is empty.
    pub redo_enabled: bool,
    /// Body text when the Guide has no Nodes. `None` leaves any other empty body blank.
    pub empty_hint: Option<&'static str>,
    /// Bottom-line sentence to publish. `None` leaves the line unchanged.
    pub status_sentence: Option<String>,
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
}

/// Decide Save, Undo, and Redo enablement, the empty-Guide hint, and the bottom line.
///
/// Save is available only while an open Guide is dirty. An untitled Guide has
/// no path; dirtiness alone makes Save available. Undo and Redo each follow
/// the focused pane’s own stack and ignore the other pane. A success publishes
/// nothing.
pub fn present(snapshot: &EditorSnapshot, outcome: Option<ActionOutcome>) -> Presentation {
    let (undo_stack_empty, redo_stack_empty) = match snapshot.focused_pane {
        FocusedPane::Body => (snapshot.body_undo_empty, snapshot.body_redo_empty),
        FocusedPane::Tree => (snapshot.tree_undo_empty, snapshot.tree_redo_empty),
    };
    Presentation {
        save_enabled: snapshot.guide_open && snapshot.dirty,
        undo_enabled: !undo_stack_empty,
        redo_enabled: !redo_stack_empty,
        empty_hint: if snapshot.guide_open && !snapshot.has_nodes {
            Some("Untitled Guide — add a Node to begin.")
        } else {
            None
        },
        status_sentence: status_sentence(outcome),
    }
}

fn status_sentence(outcome: Option<ActionOutcome>) -> Option<String> {
    match outcome {
        Some(ActionOutcome::SaveFailed { detail }) => Some(format!("Save failed: {detail}")),
        Some(ActionOutcome::NothingToSave) => Some("Nothing to save.".to_string()),
        Some(ActionOutcome::Success) | None => None,
    }
}
