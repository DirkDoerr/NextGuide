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

/// Decide Save’s enabled state, the empty-Guide hint, and the bottom line.
///
/// Save is available only while an open Guide is dirty. An untitled Guide has
/// no path; dirtiness alone makes Save available. A success publishes nothing.
pub fn present(snapshot: &EditorSnapshot, outcome: Option<ActionOutcome>) -> Presentation {
    Presentation {
        save_enabled: snapshot.guide_open && snapshot.dirty,
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
