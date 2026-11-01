//! Editor window presentation. GTK-free: snapshots in, enablement and hints out.

use nextguide::{present, ActionOutcome, EditorSnapshot};

#[test]
fn save_is_grey_when_the_guide_is_clean() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: false,
        has_nodes: false,
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
    };
    assert!(present(&snapshot, None).save_enabled);
}

#[test]
fn empty_guide_shows_the_add_a_node_hint_in_the_body() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: false,
        has_nodes: false,
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
    };
    assert_eq!(present(&snapshot, None).empty_hint, None);
}

#[test]
fn successful_save_publishes_no_status_sentence() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: true,
        has_nodes: true,
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
fn save_with_no_guide_says_nothing_to_save() {
    let snapshot = EditorSnapshot {
        guide_open: false,
        dirty: false,
        has_nodes: false,
    };
    let presentation = present(&snapshot, Some(ActionOutcome::NothingToSave));
    assert_eq!(
        presentation.status_sentence.as_deref(),
        Some("Nothing to save.")
    );
}
