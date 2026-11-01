//! Editor window presentation. GTK-free: snapshots in, enablement and hints out.

use nextguide::{present, ActionOutcome, EditorSnapshot, StructureRefusal};

#[test]
fn save_is_grey_when_the_guide_is_clean() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: false,
        has_nodes: false,
        node_selected: false,
    };
    assert!(!present(&snapshot, None).save_enabled);
}

#[test]
fn save_is_available_when_the_guide_is_dirty_including_untitled() {
    // Untitled is an open Guide with unsaved edits. Path is not a fact here.
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: true,
        has_nodes: false,
        node_selected: false,
    };
    assert!(present(&snapshot, None).save_enabled);
}

#[test]
fn empty_guide_shows_the_add_a_node_hint_in_the_body() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: false,
        has_nodes: false,
        node_selected: false,
    };
    assert_eq!(
        present(&snapshot, None).empty_hint,
        Some("Untitled Guide — add a Node to begin.")
    );
}

#[test]
fn guide_with_nodes_leaves_an_empty_body_blank() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: true,
        has_nodes: true,
        node_selected: false,
    };
    assert_eq!(present(&snapshot, None).empty_hint, None);
}

#[test]
fn successful_save_publishes_no_status_sentence() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: true,
        has_nodes: true,
        node_selected: false,
    };
    let presentation = present(&snapshot, Some(ActionOutcome::Success));
    assert_eq!(presentation.status_sentence, None);
}

#[test]
fn failed_save_still_shows_the_failure_sentence() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: true,
        has_nodes: true,
        node_selected: false,
    };
    let presentation = present(
        &snapshot,
        Some(ActionOutcome::SaveFailed {
            detail: "disk full".into(),
        }),
    );
    assert_eq!(
        presentation.status_sentence.as_deref(),
        Some("Save failed: disk full")
    );
}

#[test]
fn insert_child_is_available_when_a_guide_is_open_and_nothing_is_selected() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: false,
        has_nodes: false,
        node_selected: false,
    };
    assert!(present(&snapshot, None).insert_child_enabled);
}

#[test]
fn structure_controls_are_grey_when_no_node_is_selected() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: false,
        has_nodes: true,
        node_selected: false,
    };
    let presentation = present(&snapshot, None);
    assert!(!presentation.insert_sibling_enabled);
    assert!(!presentation.delete_enabled);
    assert!(!presentation.move_up_enabled);
    assert!(!presentation.move_down_enabled);
    assert!(!presentation.indent_enabled);
    assert!(!presentation.outdent_enabled);
}

#[test]
fn structure_controls_stay_available_when_a_node_is_selected() {
    // Enablement does not ask whether the move itself is legal.
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: true,
        has_nodes: true,
        node_selected: true,
    };
    let presentation = present(&snapshot, None);
    assert!(presentation.insert_child_enabled);
    assert!(presentation.insert_sibling_enabled);
    assert!(presentation.delete_enabled);
    assert!(presentation.move_up_enabled);
    assert!(presentation.move_down_enabled);
    assert!(presentation.indent_enabled);
    assert!(presentation.outdent_enabled);
}

#[test]
fn illegal_move_still_returns_its_refusal_sentence() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: true,
        has_nodes: true,
        node_selected: true,
    };
    let presentation = present(
        &snapshot,
        Some(ActionOutcome::StructureRefused(
            StructureRefusal::AlreadyAtEdgeOfSiblings,
        )),
    );
    assert_eq!(
        presentation.status_sentence.as_deref(),
        Some("Already at edge of siblings.")
    );
}

#[test]
fn indent_without_a_previous_sibling_returns_its_refusal_sentence() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: true,
        has_nodes: true,
        node_selected: true,
    };
    let presentation = present(
        &snapshot,
        Some(ActionOutcome::StructureRefused(
            StructureRefusal::IndentNeedsPreviousSibling,
        )),
    );
    assert_eq!(
        presentation.status_sentence.as_deref(),
        Some("Indent needs a previous sibling.")
    );
}

#[test]
fn outdent_at_top_level_returns_its_refusal_sentence() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: true,
        has_nodes: true,
        node_selected: true,
    };
    let presentation = present(
        &snapshot,
        Some(ActionOutcome::StructureRefused(
            StructureRefusal::AlreadyAtTopLevel,
        )),
    );
    assert_eq!(
        presentation.status_sentence.as_deref(),
        Some("Already at top level.")
    );
}

#[test]
fn successful_structure_edit_publishes_no_status_sentence() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: true,
        has_nodes: true,
        node_selected: true,
    };
    let presentation = present(&snapshot, Some(ActionOutcome::Success));
    assert_eq!(presentation.status_sentence, None);
}

#[test]
fn insert_child_failure_still_shows_the_failure_sentence() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: true,
        has_nodes: false,
        node_selected: false,
    };
    let presentation = present(
        &snapshot,
        Some(ActionOutcome::InsertChildFailed {
            detail: "disk full".into(),
        }),
    );
    assert_eq!(
        presentation.status_sentence.as_deref(),
        Some("Insert failed: disk full")
    );
}

#[test]
fn save_with_no_guide_says_nothing_to_save() {
    let snapshot = EditorSnapshot {
        guide_open: false,
        dirty: false,
        has_nodes: false,
        node_selected: false,
    };
    let presentation = present(&snapshot, Some(ActionOutcome::NothingToSave));
    assert_eq!(
        presentation.status_sentence.as_deref(),
        Some("Nothing to save.")
    );
}
