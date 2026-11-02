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
    /// A visible Node is selected. The Dummy root is never a selection.
    pub node_selected: bool,
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
    /// A Guide was opened. Publish no sentence.
    Opened,
    /// A Node was renamed. Publish no sentence.
    Renamed,
    /// A visible Node was selected. Publish no sentence.
    Selected,
    /// A file dialog was dismissed. The dirty prompt's Cancel uses the same
    /// outcome. Neither publishes a sentence.
    FileDialogDismissed,
    /// Save did not complete. `detail` is the error text the save path already has.
    SaveFailed { detail: String },
    /// There is no Guide to write.
    NothingToSave,
    /// A format toggle, font change, or list toggle finished.
    Formatted,
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
    /// The selected Node body could not be parsed. `path` is the Node path text.
    UnparseableBody { path: String },
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

/// Decide Save’s enabled state, formatting and structure controls, the empty-Guide
/// hint, and the bottom line.
///
/// Save is available only while an open Guide is dirty. An untitled Guide has
/// no path; dirtiness alone makes Save available. Insert Child is available
/// whenever a Guide is open, including when nothing is selected. The other
/// structure controls are grey until a Node is selected, and stay available
/// then even if the move will be refused. Undo and Redo each follow the focused
/// pane’s own stack. A success publishes nothing, including open, rename,
/// selection, and dismissing a file dialog. An unparseable Node body still
/// returns its existing soft-fail sentence.
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
        insert_child_enabled: snapshot.guide_open,
        // A selected Node keeps these clickable even when the move will be refused.
        insert_sibling_enabled: snapshot.node_selected,
        delete_enabled: snapshot.node_selected,
        move_up_enabled: snapshot.node_selected,
        move_down_enabled: snapshot.node_selected,
        indent_enabled: snapshot.node_selected,
        outdent_enabled: snapshot.node_selected,
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
        Some(ActionOutcome::UnparseableBody { path }) => Some(format!(
            "Node {path} (unparseable RTF — edit replaces original bytes)"
        )),
        Some(
            ActionOutcome::Success
            | ActionOutcome::Formatted
            | ActionOutcome::Opened
            | ActionOutcome::Renamed
            | ActionOutcome::Selected
            | ActionOutcome::FileDialogDismissed,
        )
        | None => None,
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
