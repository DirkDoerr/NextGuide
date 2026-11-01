//! GTK-free toolbar presentation. The shell reports facts; this module decides
//! what the window should show.

use nextguide_rtf::{FontFamily, FontSize};

/// Pane whose undo and redo stacks the toolbar is showing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FocusedPane {
    /// Tree and title history.
    Tree,
    /// Body buffer history.
    Body,
}

/// Facts the shell has already read from the open Guide. No widget types.
#[derive(Clone, Debug, PartialEq, Eq)]
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
    /// Body-selection facts for the formatting controls.
    pub formatting: FormattingSnapshot,
}

/// Kind of one body line. A caret contributes its line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListLineKind {
    /// Not a list.
    Paragraph,
    /// Bullet list.
    Bullet,
    /// Numbered list.
    Numbered,
}

/// What the shell has read about the body selection for formatting controls.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FormattingSnapshot {
    /// The selection covers at least one character of body text.
    pub text_selected: bool,
    /// Every selected character is bold. Ignored unless `text_selected`.
    pub all_bold: bool,
    /// Every selected character is italic. Ignored unless `text_selected`.
    pub all_italic: bool,
    /// Every selected character is underlined. Ignored unless `text_selected`.
    pub all_underline: bool,
    /// Family when every selected character shares one. `None` when the selection is mixed.
    pub shared_family: Option<FontFamily>,
    /// Point size when every selected character shares one. `None` when the selection is mixed.
    pub shared_size: Option<FontSize>,
    /// A Node body is the writing surface.
    pub body_editing: bool,
    /// Line kinds touched by the selection. A caret is its one line.
    pub lines: Vec<ListLineKind>,
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
    /// Bold, italic, underline, lists, and the font dropdowns.
    pub formatting: FormattingControls,
}

/// One formatting icon button. A grey control is not pressed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormatToggle {
    /// Grey when false.
    pub enabled: bool,
    /// Shown pressed. Never true when `enabled` is false.
    pub pressed: bool,
    /// Command name and the existing shortcut. Kept while the control is grey.
    pub tooltip: &'static str,
}

/// Family dropdown. Blank when `displayed` is `None`, including when grey.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FontFamilyControl {
    /// Grey when false.
    pub enabled: bool,
    /// Shared family, or blank.
    pub displayed: Option<FontFamily>,
    /// Families the dropdown offers, in menu order.
    pub choices: &'static [FontFamily],
}

/// Size dropdown. Blank when `displayed` is `None`, including when grey.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FontSizeControl {
    /// Grey when false.
    pub enabled: bool,
    /// Shared point size, or blank.
    pub displayed: Option<FontSize>,
    /// Point sizes the dropdown offers, ascending.
    pub choices: &'static [FontSize],
}

/// Formatting controls the toolbar renders from a snapshot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormattingControls {
    pub bold: FormatToggle,
    pub italic: FormatToggle,
    pub underline: FormatToggle,
    pub family: FontFamilyControl,
    pub size: FontSizeControl,
    pub bullet_list: FormatToggle,
    pub numbered_list: FormatToggle,
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
    /// A format toggle, font change, or list toggle finished.
    Formatted,
}

/// Decide Save, Undo, and Redo enablement, the formatting controls, the empty-Guide
/// hint, and the bottom line.
///
/// Save is available only while an open Guide is dirty. An untitled Guide has
/// no path; dirtiness alone makes Save available. Undo and Redo each follow
/// the focused pane’s own stack and ignore the other pane. Formatting controls
/// follow the body-selection facts. A success publishes nothing.
pub fn present(snapshot: &EditorSnapshot, outcome: Option<ActionOutcome>) -> Presentation {
    let text_selected = snapshot.formatting.text_selected;
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
        formatting: FormattingControls {
            bold: character_toggle(text_selected, snapshot.formatting.all_bold, "Bold (Ctrl+B)"),
            italic: character_toggle(
                text_selected,
                snapshot.formatting.all_italic,
                "Italic (Ctrl+I)",
            ),
            underline: character_toggle(
                text_selected,
                snapshot.formatting.all_underline,
                "Underline (Ctrl+U)",
            ),
            family: FontFamilyControl {
                enabled: text_selected,
                displayed: shared_when_selected(text_selected, snapshot.formatting.shared_family),
                choices: &FontFamily::ALL,
            },
            size: FontSizeControl {
                enabled: text_selected,
                displayed: shared_when_selected(text_selected, snapshot.formatting.shared_size),
                choices: &FontSize::ALL,
            },
            bullet_list: list_toggle(
                snapshot.formatting.body_editing,
                &snapshot.formatting.lines,
                ListLineKind::Bullet,
                "Bullet List (Ctrl+Shift+8)",
            ),
            numbered_list: list_toggle(
                snapshot.formatting.body_editing,
                &snapshot.formatting.lines,
                ListLineKind::Numbered,
                "Numbered List (Ctrl+Shift+7)",
            ),
        },
    }
}

/// A dropdown shows a shared value only while some body text is selected.
fn shared_when_selected<T>(text_selected: bool, shared: Option<T>) -> Option<T> {
    if text_selected {
        shared
    } else {
        None
    }
}

/// List buttons stay available while a Node body is edited. Pressed only when every
/// reported line is that kind. A grey button is not pressed.
fn list_toggle(
    body_editing: bool,
    lines: &[ListLineKind],
    kind: ListLineKind,
    tooltip: &'static str,
) -> FormatToggle {
    let pressed = body_editing && !lines.is_empty() && lines.iter().all(|line| *line == kind);
    FormatToggle {
        enabled: body_editing,
        pressed,
        tooltip,
    }
}

/// Character controls are grey, and not pressed, unless some body text is selected.
fn character_toggle(
    text_selected: bool,
    whole_selection: bool,
    tooltip: &'static str,
) -> FormatToggle {
    FormatToggle {
        enabled: text_selected,
        pressed: text_selected && whole_selection,
        tooltip,
    }
}

fn status_sentence(outcome: Option<ActionOutcome>) -> Option<String> {
    match outcome {
        Some(ActionOutcome::SaveFailed { detail }) => Some(format!("Save failed: {detail}")),
        Some(ActionOutcome::NothingToSave) => Some("Nothing to save.".to_string()),
        Some(ActionOutcome::Success | ActionOutcome::Formatted) | None => None,
    }
}
