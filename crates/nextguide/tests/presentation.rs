//! Editor window presentation. GTK-free: snapshots in, enablement and hints out.

use nextguide::{
    present, ActionOutcome, EditorSnapshot, FocusedPane, FormattingSnapshot, ListLineKind,
};
use nextguide_rtf::{FontFamily, FontSize};

#[test]
fn save_is_grey_when_the_guide_is_clean() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: false,
        has_nodes: false,
        focused_pane: FocusedPane::Tree,
        body_undo_empty: true,
        tree_undo_empty: true,
        body_redo_empty: true,
        tree_redo_empty: true,
        formatting: FormattingSnapshot::default(),
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
        focused_pane: FocusedPane::Tree,
        body_undo_empty: true,
        tree_undo_empty: true,
        body_redo_empty: true,
        tree_redo_empty: true,
        formatting: FormattingSnapshot::default(),
    };
    assert!(present(&snapshot, None).save_enabled);
}

#[test]
fn empty_guide_shows_the_add_a_node_hint_in_the_body() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: false,
        has_nodes: false,
        focused_pane: FocusedPane::Tree,
        body_undo_empty: true,
        tree_undo_empty: true,
        body_redo_empty: true,
        tree_redo_empty: true,
        formatting: FormattingSnapshot::default(),
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
        focused_pane: FocusedPane::Tree,
        body_undo_empty: true,
        tree_undo_empty: true,
        body_redo_empty: true,
        tree_redo_empty: true,
        formatting: FormattingSnapshot::default(),
    };
    assert_eq!(present(&snapshot, None).empty_hint, None);
}

#[test]
fn successful_save_publishes_no_status_sentence() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: true,
        has_nodes: true,
        focused_pane: FocusedPane::Tree,
        body_undo_empty: true,
        tree_undo_empty: true,
        body_redo_empty: true,
        tree_redo_empty: true,
        formatting: FormattingSnapshot::default(),
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
        focused_pane: FocusedPane::Tree,
        body_undo_empty: true,
        tree_undo_empty: true,
        body_redo_empty: true,
        tree_redo_empty: true,
        formatting: FormattingSnapshot::default(),
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
fn bold_is_grey_and_not_pressed_when_no_body_text_is_selected() {
    // A stale "all bold" fact must not make a grey character control look pressed.
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: false,
        has_nodes: true,
        focused_pane: FocusedPane::Tree,
        body_undo_empty: true,
        tree_undo_empty: true,
        body_redo_empty: true,
        tree_redo_empty: true,
        formatting: FormattingSnapshot {
            text_selected: false,
            all_bold: true,
            ..FormattingSnapshot::default()
        },
    };
    let bold = present(&snapshot, None).formatting.bold;
    assert!(!bold.enabled);
    assert!(!bold.pressed);
}

#[test]
fn bold_looks_pressed_only_when_the_whole_selection_is_bold() {
    let pressed = present(
        &EditorSnapshot {
            guide_open: true,
            dirty: false,
            has_nodes: true,
            focused_pane: FocusedPane::Tree,
            body_undo_empty: true,
            tree_undo_empty: true,
            body_redo_empty: true,
            tree_redo_empty: true,
            formatting: FormattingSnapshot {
                text_selected: true,
                all_bold: true,
                ..FormattingSnapshot::default()
            },
        },
        None,
    )
    .formatting
    .bold;
    assert!(pressed.enabled);
    assert!(pressed.pressed);

    let plain = present(
        &EditorSnapshot {
            guide_open: true,
            dirty: false,
            has_nodes: true,
            focused_pane: FocusedPane::Tree,
            body_undo_empty: true,
            tree_undo_empty: true,
            body_redo_empty: true,
            tree_redo_empty: true,
            formatting: FormattingSnapshot {
                text_selected: true,
                all_bold: false,
                ..FormattingSnapshot::default()
            },
        },
        None,
    )
    .formatting
    .bold;
    assert!(plain.enabled);
    assert!(!plain.pressed);
}

#[test]
fn italic_is_grey_unless_text_is_selected_and_pressed_only_when_all_italic() {
    let grey = present(
        &open_guide(FormattingSnapshot {
            text_selected: false,
            all_italic: true,
            ..FormattingSnapshot::default()
        }),
        None,
    )
    .formatting
    .italic;
    assert!(!grey.enabled);
    assert!(!grey.pressed);

    let uniform = present(
        &open_guide(FormattingSnapshot {
            text_selected: true,
            all_italic: true,
            ..FormattingSnapshot::default()
        }),
        None,
    )
    .formatting
    .italic;
    assert!(uniform.enabled);
    assert!(uniform.pressed);

    let mixed = present(
        &open_guide(FormattingSnapshot {
            text_selected: true,
            all_italic: false,
            ..FormattingSnapshot::default()
        }),
        None,
    )
    .formatting
    .italic;
    assert!(mixed.enabled);
    assert!(!mixed.pressed);
}

#[test]
fn underline_is_grey_unless_text_is_selected_and_pressed_only_when_all_underlined() {
    let grey = present(
        &open_guide(FormattingSnapshot {
            text_selected: false,
            all_underline: true,
            ..FormattingSnapshot::default()
        }),
        None,
    )
    .formatting
    .underline;
    assert!(!grey.enabled);
    assert!(!grey.pressed);

    let uniform = present(
        &open_guide(FormattingSnapshot {
            text_selected: true,
            all_underline: true,
            ..FormattingSnapshot::default()
        }),
        None,
    )
    .formatting
    .underline;
    assert!(uniform.enabled);
    assert!(uniform.pressed);

    let mixed = present(
        &open_guide(FormattingSnapshot {
            text_selected: true,
            all_underline: false,
            ..FormattingSnapshot::default()
        }),
        None,
    )
    .formatting
    .underline;
    assert!(mixed.enabled);
    assert!(!mixed.pressed);
}

#[test]
fn family_dropdown_is_grey_and_blank_when_no_body_text_is_selected() {
    let family = present(
        &open_guide(FormattingSnapshot {
            text_selected: false,
            shared_family: Some(FontFamily::Arial),
            ..FormattingSnapshot::default()
        }),
        None,
    )
    .formatting
    .family;
    assert!(!family.enabled);
    assert_eq!(family.displayed, None);
}

#[test]
fn family_dropdown_shows_a_value_only_when_the_whole_selection_shares_it() {
    let arial = present(
        &open_guide(FormattingSnapshot {
            text_selected: true,
            shared_family: Some(FontFamily::Arial),
            ..FormattingSnapshot::default()
        }),
        None,
    )
    .formatting
    .family;
    assert!(arial.enabled);
    assert_eq!(arial.displayed, Some(FontFamily::Arial));

    let sans = present(
        &open_guide(FormattingSnapshot {
            text_selected: true,
            shared_family: Some(FontFamily::Sans),
            ..FormattingSnapshot::default()
        }),
        None,
    )
    .formatting
    .family;
    assert_eq!(sans.displayed, Some(FontFamily::Sans));

    let mixed = present(
        &open_guide(FormattingSnapshot {
            text_selected: true,
            shared_family: None,
            ..FormattingSnapshot::default()
        }),
        None,
    )
    .formatting
    .family;
    assert!(mixed.enabled);
    assert_eq!(mixed.displayed, None);
}

#[test]
fn size_dropdown_is_grey_without_a_selection_and_shows_a_size_only_when_shared() {
    let grey = present(
        &open_guide(FormattingSnapshot {
            text_selected: false,
            shared_size: FontSize::from_pt(12),
            ..FormattingSnapshot::default()
        }),
        None,
    )
    .formatting
    .size;
    assert!(!grey.enabled);
    assert_eq!(grey.displayed, None);

    let uniform = present(
        &open_guide(FormattingSnapshot {
            text_selected: true,
            shared_size: FontSize::from_pt(18),
            ..FormattingSnapshot::default()
        }),
        None,
    )
    .formatting
    .size;
    assert!(uniform.enabled);
    assert_eq!(uniform.displayed, FontSize::from_pt(18));

    let mixed = present(
        &open_guide(FormattingSnapshot {
            text_selected: true,
            shared_size: None,
            ..FormattingSnapshot::default()
        }),
        None,
    )
    .formatting
    .size;
    assert!(mixed.enabled);
    assert_eq!(mixed.displayed, None);
}

#[test]
fn list_buttons_are_grey_and_not_pressed_when_no_node_body_is_being_edited() {
    let formatting = present(
        &open_guide(FormattingSnapshot {
            body_editing: false,
            lines: vec![ListLineKind::Bullet, ListLineKind::Bullet],
            ..FormattingSnapshot::default()
        }),
        None,
    )
    .formatting;
    assert!(!formatting.bullet_list.enabled);
    assert!(!formatting.bullet_list.pressed);
    assert!(!formatting.numbered_list.enabled);
    assert!(!formatting.numbered_list.pressed);
}

#[test]
fn list_buttons_stay_available_at_a_caret_and_match_that_line() {
    let bullet = present(
        &open_guide(FormattingSnapshot {
            body_editing: true,
            text_selected: false,
            lines: vec![ListLineKind::Bullet],
            ..FormattingSnapshot::default()
        }),
        None,
    )
    .formatting;
    assert!(!bullet.bold.enabled);
    assert!(!bullet.family.enabled);
    assert!(bullet.bullet_list.enabled);
    assert!(bullet.bullet_list.pressed);
    assert!(bullet.numbered_list.enabled);
    assert!(!bullet.numbered_list.pressed);

    let numbered = present(
        &open_guide(FormattingSnapshot {
            body_editing: true,
            text_selected: false,
            lines: vec![ListLineKind::Numbered],
            ..FormattingSnapshot::default()
        }),
        None,
    )
    .formatting;
    assert!(numbered.numbered_list.enabled);
    assert!(numbered.numbered_list.pressed);
    assert!(!numbered.bullet_list.pressed);

    let paragraph = present(
        &open_guide(FormattingSnapshot {
            body_editing: true,
            text_selected: false,
            lines: vec![ListLineKind::Paragraph],
            ..FormattingSnapshot::default()
        }),
        None,
    )
    .formatting;
    assert!(paragraph.bullet_list.enabled);
    assert!(!paragraph.bullet_list.pressed);
    assert!(paragraph.numbered_list.enabled);
    assert!(!paragraph.numbered_list.pressed);
}

#[test]
fn list_button_looks_pressed_only_when_every_selected_line_is_that_list() {
    let mixed = present(
        &open_guide(FormattingSnapshot {
            body_editing: true,
            text_selected: true,
            lines: vec![
                ListLineKind::Bullet,
                ListLineKind::Numbered,
                ListLineKind::Paragraph,
            ],
            ..FormattingSnapshot::default()
        }),
        None,
    )
    .formatting;
    assert!(mixed.bullet_list.enabled);
    assert!(!mixed.bullet_list.pressed);
    assert!(mixed.numbered_list.enabled);
    assert!(!mixed.numbered_list.pressed);

    let bullets = present(
        &open_guide(FormattingSnapshot {
            body_editing: true,
            text_selected: true,
            lines: vec![ListLineKind::Bullet, ListLineKind::Bullet],
            ..FormattingSnapshot::default()
        }),
        None,
    )
    .formatting;
    assert!(bullets.bullet_list.pressed);
    assert!(!bullets.numbered_list.pressed);

    let numbered = present(
        &open_guide(FormattingSnapshot {
            body_editing: true,
            text_selected: true,
            lines: vec![ListLineKind::Numbered, ListLineKind::Numbered],
            ..FormattingSnapshot::default()
        }),
        None,
    )
    .formatting;
    assert!(!numbered.bullet_list.pressed);
    assert!(numbered.numbered_list.pressed);
}

#[test]
fn formatting_icon_tooltips_name_the_command_and_its_shortcut() {
    // Grey controls keep the tooltip. This snapshot selects nothing and edits no body.
    let formatting = present(&open_guide(FormattingSnapshot::default()), None).formatting;
    assert_eq!(formatting.bold.tooltip, "Bold (Ctrl+B)");
    assert_eq!(formatting.italic.tooltip, "Italic (Ctrl+I)");
    assert_eq!(formatting.underline.tooltip, "Underline (Ctrl+U)");
    assert_eq!(formatting.bullet_list.tooltip, "Bullet List (Ctrl+Shift+8)");
    assert_eq!(
        formatting.numbered_list.tooltip,
        "Numbered List (Ctrl+Shift+7)"
    );
}

#[test]
fn font_dropdowns_offer_the_closed_family_and_size_sets() {
    let formatting = present(&open_guide(FormattingSnapshot::default()), None).formatting;
    let families: Vec<_> = formatting
        .family
        .choices
        .iter()
        .map(|family| family.label())
        .collect();
    assert_eq!(families, ["Arial", "System Sans"]);
    let sizes: Vec<_> = formatting
        .size
        .choices
        .iter()
        .map(|size| size.pt())
        .collect();
    assert_eq!(sizes, [9, 10, 11, 12, 14, 18, 24]);
}

#[test]
fn a_successful_format_change_publishes_no_success_sentence() {
    let presentation = present(
        &open_guide(FormattingSnapshot {
            body_editing: true,
            text_selected: true,
            all_bold: true,
            ..FormattingSnapshot::default()
        }),
        Some(ActionOutcome::Formatted),
    );
    assert_eq!(presentation.status_sentence, None);
}

fn open_guide(formatting: FormattingSnapshot) -> EditorSnapshot {
    EditorSnapshot {
        guide_open: true,
        dirty: false,
        has_nodes: true,
        focused_pane: FocusedPane::Tree,
        body_undo_empty: true,
        tree_undo_empty: true,
        body_redo_empty: true,
        tree_redo_empty: true,
        formatting,
    }
}

#[test]
fn undo_is_grey_when_the_body_panes_undo_stack_is_empty() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: false,
        has_nodes: true,
        formatting: FormattingSnapshot::default(),
        focused_pane: FocusedPane::Body,
        body_undo_empty: true,
        tree_undo_empty: true,
        body_redo_empty: true,
        tree_redo_empty: true,
    };
    assert!(!present(&snapshot, None).undo_enabled);
}

#[test]
fn undo_stays_grey_when_the_body_stack_is_empty_and_the_tree_stack_is_not() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: false,
        has_nodes: true,
        formatting: FormattingSnapshot::default(),
        focused_pane: FocusedPane::Body,
        body_undo_empty: true,
        tree_undo_empty: false,
        body_redo_empty: true,
        tree_redo_empty: true,
    };
    assert!(!present(&snapshot, None).undo_enabled);
}

#[test]
fn undo_is_available_when_the_body_stack_has_a_step_and_the_tree_stack_is_empty() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: true,
        has_nodes: true,
        formatting: FormattingSnapshot::default(),
        focused_pane: FocusedPane::Body,
        body_undo_empty: false,
        tree_undo_empty: true,
        body_redo_empty: true,
        tree_redo_empty: true,
    };
    assert!(present(&snapshot, None).undo_enabled);
}

#[test]
fn undo_is_grey_when_the_tree_stack_is_empty_and_the_body_stack_is_not() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: true,
        has_nodes: true,
        formatting: FormattingSnapshot::default(),
        focused_pane: FocusedPane::Tree,
        body_undo_empty: false,
        tree_undo_empty: true,
        body_redo_empty: true,
        tree_redo_empty: true,
    };
    assert!(!present(&snapshot, None).undo_enabled);
}

#[test]
fn undo_is_available_when_the_tree_stack_has_a_step_and_the_body_stack_is_empty() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: false,
        has_nodes: true,
        formatting: FormattingSnapshot::default(),
        focused_pane: FocusedPane::Tree,
        body_undo_empty: true,
        tree_undo_empty: false,
        body_redo_empty: true,
        tree_redo_empty: true,
    };
    assert!(present(&snapshot, None).undo_enabled);
}

#[test]
fn redo_stays_grey_when_the_body_stack_is_empty_and_the_tree_stack_is_not() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: false,
        has_nodes: true,
        formatting: FormattingSnapshot::default(),
        focused_pane: FocusedPane::Body,
        body_undo_empty: false,
        tree_undo_empty: true,
        body_redo_empty: true,
        tree_redo_empty: false,
    };
    assert!(!present(&snapshot, None).redo_enabled);
}

#[test]
fn redo_is_available_when_the_body_stack_has_a_step_and_the_tree_stack_is_empty() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: true,
        has_nodes: true,
        formatting: FormattingSnapshot::default(),
        focused_pane: FocusedPane::Body,
        body_undo_empty: true,
        tree_undo_empty: false,
        body_redo_empty: false,
        tree_redo_empty: true,
    };
    assert!(present(&snapshot, None).redo_enabled);
}

#[test]
fn redo_is_grey_when_the_tree_stack_is_empty_and_the_body_stack_is_not() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: true,
        has_nodes: true,
        formatting: FormattingSnapshot::default(),
        focused_pane: FocusedPane::Tree,
        body_undo_empty: true,
        tree_undo_empty: false,
        body_redo_empty: false,
        tree_redo_empty: true,
    };
    assert!(!present(&snapshot, None).redo_enabled);
}

#[test]
fn redo_is_available_when_the_tree_stack_has_a_step_and_the_body_stack_is_empty() {
    let snapshot = EditorSnapshot {
        guide_open: true,
        dirty: false,
        has_nodes: true,
        formatting: FormattingSnapshot::default(),
        focused_pane: FocusedPane::Tree,
        body_undo_empty: false,
        tree_undo_empty: true,
        body_redo_empty: true,
        tree_redo_empty: false,
    };
    assert!(present(&snapshot, None).redo_enabled);
}

#[test]
fn save_with_no_guide_says_nothing_to_save() {
    let snapshot = EditorSnapshot {
        guide_open: false,
        dirty: false,
        has_nodes: false,
        focused_pane: FocusedPane::Tree,
        body_undo_empty: true,
        tree_undo_empty: true,
        body_redo_empty: true,
        tree_redo_empty: true,
        formatting: FormattingSnapshot::default(),
    };
    let presentation = present(&snapshot, Some(ActionOutcome::NothingToSave));
    assert_eq!(
        presentation.status_sentence.as_deref(),
        Some("Nothing to save.")
    );
}
