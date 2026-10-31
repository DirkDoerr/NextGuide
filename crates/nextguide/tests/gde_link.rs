//! Editor helpers at the `nextguide` ↔ `nextguide-gde` / `nextguide-rtf` seam (no GTK).

use std::path::Path;
use std::path::PathBuf;

use nextguide_gde::{check_structural, NodePath};
use nextguide_rtf::{Document, Run, SpanStyle};
use nextguide::{
    body_view_from_rtf, launch_path_from_args, load_guide, needs_dirty_prompt, visible_tree,
    window_title_for_path, DirtyPromptChoice, EditorDoc, RecentFiles, TREE_UNDO_LIMIT,
};

fn repo_data(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.pop();
    path.push("data");
    path.push(name);
    path
}

#[test]
fn bare_launch_window_title_is_untitled() {
    assert_eq!(window_title_for_path(None, false), "Untitled");
}

#[test]
fn new_guide_is_untitled_clean_and_empty_tree() {
    let doc = EditorDoc::new();
    assert!(doc.path().is_none());
    assert!(!doc.is_dirty());
    assert!(doc.visible_tree().is_empty());
    assert_eq!(doc.guide().root().title(), "dummy");
    assert_eq!(window_title_for_path(doc.path(), doc.is_dirty()), "Untitled");
}

#[test]
fn save_as_writes_updates_path_and_clears_dirty() {
    let mut doc = EditorDoc::new();
    let page = doc
        .insert_child(&NodePath::root(), 0)
        .expect("insert top-level");
    doc.set_title(&page, "Only").expect("title");
    assert!(doc.is_dirty());

    let out = std::env::temp_dir().join("nextguide-save-as.gde");
    doc.save_as(&out).expect("save as");
    assert!(!doc.is_dirty());
    assert_eq!(doc.path(), Some(out.as_path()));
    assert_eq!(
        window_title_for_path(doc.path(), doc.is_dirty()),
        "nextguide-save-as.gde"
    );

    let rewritten = std::fs::read(&out).unwrap();
    let _ = std::fs::remove_file(&out);
    check_structural(&rewritten).expect("structural after save as");
    let again = nextguide_gde::load_bytes(&rewritten).unwrap();
    assert_eq!(again.root().children()[0].title(), "Only");
}

#[test]
fn dirty_prompt_needed_only_when_dirty() {
    assert!(!needs_dirty_prompt(false));
    assert!(needs_dirty_prompt(true));
}

#[test]
fn dirty_prompt_choice_labels_match_session_policy() {
    assert_eq!(DirtyPromptChoice::Save.label(), "Save");
    assert_eq!(DirtyPromptChoice::Discard.label(), "Discard");
    assert_eq!(DirtyPromptChoice::Cancel.label(), "Cancel");
}

#[test]
fn recent_files_records_mru_dedupes_and_caps_at_five() {
    let mut recent = RecentFiles::new();
    for i in 1..=6 {
        recent.record(PathBuf::from(format!("/tmp/g{i}.gde")));
    }
    let paths: Vec<_> = recent.paths().iter().map(|p| p.display().to_string()).collect();
    assert_eq!(
        paths,
        [
            "/tmp/g6.gde",
            "/tmp/g5.gde",
            "/tmp/g4.gde",
            "/tmp/g3.gde",
            "/tmp/g2.gde",
        ]
    );

    recent.record(PathBuf::from("/tmp/g3.gde"));
    let paths: Vec<_> = recent.paths().iter().map(|p| p.display().to_string()).collect();
    assert_eq!(
        paths,
        [
            "/tmp/g3.gde",
            "/tmp/g6.gde",
            "/tmp/g5.gde",
            "/tmp/g4.gde",
            "/tmp/g2.gde",
        ]
    );
}

#[test]
fn recent_files_round_trips_through_store_file() {
    let store = std::env::temp_dir().join("nextguide-recent-test.txt");
    let _ = std::fs::remove_file(&store);

    let mut recent = RecentFiles::new();
    recent.record(PathBuf::from("/tmp/a.gde"));
    recent.record(PathBuf::from("/tmp/b.gde"));
    recent.save_to(&store).expect("write recent");

    let loaded = RecentFiles::load_from(&store).expect("load recent");
    let paths: Vec<_> = loaded.paths().iter().map(|p| p.display().to_string()).collect();
    assert_eq!(paths, ["/tmp/b.gde", "/tmp/a.gde"]);
    let _ = std::fs::remove_file(&store);
}

#[test]
fn launch_path_from_args_opens_path_or_bare_new() {
    assert_eq!(
        launch_path_from_args(["nextguide".into()].into_iter()),
        None,
        "bare launch → untitled New"
    );
    assert_eq!(
        launch_path_from_args(
            ["nextguide".into(), "/tmp/ImportSample.gde".into()].into_iter()
        ),
        Some(PathBuf::from("/tmp/ImportSample.gde"))
    );
}

#[test]
fn open_path_window_title_uses_file_name() {
    assert_eq!(
        window_title_for_path(Some(Path::new("/tmp/ImportSample.gde")), false),
        "ImportSample.gde"
    );
}

#[test]
fn dirty_window_title_marks_filename() {
    assert_eq!(
        window_title_for_path(Some(Path::new("/tmp/ImportSample.gde")), true),
        "ImportSample.gde *"
    );
    assert_eq!(window_title_for_path(None, true), "Untitled *");
}

#[test]
fn load_guide_links_nextguide_gde_on_fixture() {
    let guide = load_guide(repo_data("ImportSample.gde")).expect("load via editor helper");
    assert_eq!(guide.root().title(), "dummy");
    assert_eq!(guide.root().children()[0].title(), "Page1");
}

#[test]
fn visible_tree_hides_dummy_root_and_maps_paths() {
    let doc = EditorDoc::open(repo_data("ImportSample.gde")).expect("open");
    let tree = doc.visible_tree();
    assert_eq!(tree.len(), 1);
    assert_eq!(tree[0].title, "Page1");
    assert_eq!(tree[0].path, NodePath::from_indices(vec![0]));
    assert_eq!(tree[0].children.len(), 2);
    assert_eq!(tree[0].children[0].title, "Child1");
    assert_eq!(tree[0].children[0].path, NodePath::from_indices(vec![0, 0]));
    assert_eq!(
        tree[0].children[0].children[0].title,
        "Grandchild"
    );
    assert_eq!(
        tree[0].children[0].children[0].path,
        NodePath::from_indices(vec![0, 0, 0])
    );
    // Dummy root must not appear as a row title.
    assert!(visible_tree(doc.guide())
        .iter()
        .all(|r| r.title != "dummy"));
}

#[test]
fn body_view_soft_fails_unparseable_without_requiring_edit() {
    let view = body_view_from_rtf(b"not-rtf");
    assert!(view.soft_fail);
    assert!(view.display_text.contains("not-rtf"));
}

#[test]
fn body_view_shows_guide_node_text() {
    let doc = EditorDoc::open(repo_data("ImportSample.gde")).expect("open");
    let page1 = NodePath::from_indices(vec![0]);
    let view = doc.body_view(&page1).expect("body");
    assert!(!view.soft_fail);
    // Page1 body is mostly empty paragraphs in the fixture; ensure we got a view.
    let _ = view.display_text;
}

#[test]
fn save_unchanged_keeps_structural_round_trip_and_rtf_bytes() {
    let src = repo_data("ImportSample.gde");
    let original = std::fs::read(&src).unwrap();
    let mut doc = EditorDoc::open(&src).expect("open");
    let before_rtf = doc.guide().root().children()[0].rtf_body().to_vec();

    let out = std::env::temp_dir().join("nextguide-save-unchanged.gde");
    doc.save_to(&out).expect("save");
    assert!(!doc.is_dirty());
    let rewritten = std::fs::read(&out).unwrap();
    let _ = std::fs::remove_file(&out);

    check_structural(&rewritten).expect("structural after save unchanged");
    let again = nextguide_gde::load_bytes(&rewritten).unwrap();
    assert_eq!(again.root().children()[0].rtf_body(), before_rtf.as_slice());

    // Codec may canonicalize ids/attrs; untouched RTF payloads must survive.
    let once = nextguide_gde::save_bytes(&nextguide_gde::load_bytes(&original).unwrap()).unwrap();
    let via_editor =
        nextguide_gde::save_bytes(&nextguide_gde::load_bytes(&rewritten).unwrap()).unwrap();
    assert_eq!(once, via_editor);
}

#[test]
fn set_title_marks_dirty_and_updates_visible_tree() {
    let mut doc = EditorDoc::open(repo_data("ImportSample.gde")).expect("open");
    assert!(!doc.is_dirty());
    let page1 = NodePath::from_indices(vec![0]);
    doc.set_title(&page1, "Page1-renamed").expect("rename");
    assert!(doc.is_dirty());
    assert_eq!(doc.visible_tree()[0].title, "Page1-renamed");
    assert_eq!(
        window_title_for_path(doc.path(), doc.is_dirty()),
        "ImportSample.gde *"
    );
}

#[test]
fn set_body_rewrites_only_that_node_and_marks_dirty() {
    let mut doc = EditorDoc::open(repo_data("ImportSample.gde")).expect("open");
    let page1 = NodePath::from_indices(vec![0]);
    let child1 = NodePath::from_indices(vec![0, 0]);
    let sibling_before = doc.guide().node(&child1).unwrap().rtf_body().to_vec();
    let page1_before = doc.guide().node(&page1).unwrap().rtf_body().to_vec();

    let edited = Document {
        blocks: vec![nextguide_rtf::Block::Paragraph(vec![Run {
            text: "Edited body".into(),
            style: SpanStyle {
                bold: true,
                ..SpanStyle::default()
            },
        }])],
    };
    doc.set_body(&page1, &edited).expect("set body");
    assert!(doc.is_dirty());

    let page1_after = doc.guide().node(&page1).unwrap().rtf_body().to_vec();
    assert_ne!(page1_after, page1_before);
    assert!(page1_after.windows(b"Edited body".len()).any(|w| w == b"Edited body"));
    assert_eq!(
        doc.guide().node(&child1).unwrap().rtf_body(),
        sibling_before.as_slice(),
        "untouched sibling body must keep original RTF bytes"
    );
}

#[test]
fn title_and_body_edits_preserve_through_non_core_attrs() {
    // ImportSample Child1 carries color attr 0xffffffff (LE); observe via saved bytes.
    let mut needle = Vec::new();
    needle.extend_from_slice(&4u32.to_le_bytes());
    needle.extend_from_slice(&4u32.to_le_bytes());
    needle.extend_from_slice(&0xffff_ffffu32.to_le_bytes());

    let mut doc = EditorDoc::open(repo_data("ImportSample.gde")).expect("open");
    let child1 = NodePath::from_indices(vec![0, 0]);
    doc.set_title(&child1, "Child1-edited").expect("title");
    doc.set_body(&child1, &Document::plain("body edit"))
        .expect("body");

    let out = std::env::temp_dir().join("nextguide-preserve-edit.gde");
    doc.save_to(&out).expect("save");
    let rewritten = std::fs::read(&out).unwrap();
    let _ = std::fs::remove_file(&out);

    assert!(
        rewritten.windows(needle.len()).any(|w| w == needle),
        "Preserve-through color attr must survive EditorDoc title/body edits"
    );
    check_structural(&rewritten).expect("structural");
}

#[test]
fn save_after_title_and_body_edits_clears_dirty_and_round_trips() {
    let src = repo_data("ImportSample.gde");
    let mut doc = EditorDoc::open(&src).expect("open");
    let page1 = NodePath::from_indices(vec![0]);
    let child1 = NodePath::from_indices(vec![0, 0]);
    let sibling_before = doc.guide().node(&child1).unwrap().rtf_body().to_vec();

    doc.set_title(&page1, "Page1-edited").expect("title");
    doc.set_body(&page1, &Document::plain("New page body"))
        .expect("body");

    let out = std::env::temp_dir().join("nextguide-edit-save.gde");
    doc.save_to(&out).expect("save");
    assert!(!doc.is_dirty());
    assert_eq!(
        window_title_for_path(doc.path(), doc.is_dirty()),
        "ImportSample.gde"
    );

    let rewritten = std::fs::read(&out).unwrap();
    let _ = std::fs::remove_file(&out);
    check_structural(&rewritten).expect("structural after edits");

    let again = nextguide_gde::load_bytes(&rewritten).unwrap();
    assert_eq!(again.node(&page1).unwrap().title(), "Page1-edited");
    let body = again.node(&page1).unwrap().rtf_body();
    assert!(body.windows(b"New page body".len()).any(|w| w == b"New page body"));
    assert_eq!(
        again.node(&child1).unwrap().rtf_body(),
        sibling_before.as_slice()
    );
}

fn color_attr_needle() -> Vec<u8> {
    // ImportSample Child1 color attr 0xffffffff (LE).
    let mut needle = Vec::new();
    needle.extend_from_slice(&4u32.to_le_bytes());
    needle.extend_from_slice(&4u32.to_le_bytes());
    needle.extend_from_slice(&0xffff_ffffu32.to_le_bytes());
    needle
}

#[test]
fn insert_child_marks_dirty_uses_empty_rtf_and_updates_tree() {
    let mut doc = EditorDoc::open(repo_data("ImportSample.gde")).expect("open");
    let page1 = NodePath::from_indices(vec![0]);
    let before_len = doc.visible_tree()[0].children.len();

    let inserted = doc.insert_child(&page1, before_len).expect("insert");
    assert!(doc.is_dirty());
    assert_eq!(inserted, NodePath::from_indices(vec![0, before_len]));
    assert_eq!(doc.visible_tree()[0].children.len(), before_len + 1);
    assert_eq!(doc.guide().node(&inserted).unwrap().title(), "");
    let rtf = doc.guide().node(&inserted).unwrap().rtf_body();
    assert!(!rtf.is_empty(), "editor policy: new bodies get minimal empty RTF");
    assert!(rtf.windows(b"\\rtf1".len()).any(|w| w == b"\\rtf1"));
}

#[test]
fn remove_node_marks_dirty_refuses_dummy_root_and_updates_tree() {
    let mut doc = EditorDoc::open(repo_data("ImportSample.gde")).expect("open");
    let child1 = NodePath::from_indices(vec![0, 0]);
    doc.remove_node(&child1).expect("remove");
    assert!(doc.is_dirty());
    let tree = doc.visible_tree();
    let titles: Vec<_> = tree[0].children.iter().map(|r| r.title.as_str()).collect();
    assert_eq!(titles, ["Child2"]);

    let err = doc
        .remove_node(&NodePath::root())
        .expect_err("dummy root");
    assert!(
        matches!(err, nextguide_gde::Error::InvalidEdit(_)),
        "{err}"
    );
}

#[test]
fn move_node_reorders_and_marks_dirty() {
    let mut doc = EditorDoc::open(repo_data("ImportSample.gde")).expect("open");
    let from = NodePath::from_indices(vec![0, 0]);
    let parent = NodePath::from_indices(vec![0]);
    let new_path = doc.move_node(&from, &parent, 2).expect("move");
    assert!(doc.is_dirty());
    assert_eq!(new_path, NodePath::from_indices(vec![0, 1]));
    let tree = doc.visible_tree();
    let titles: Vec<_> = tree[0].children.iter().map(|r| r.title.as_str()).collect();
    assert_eq!(titles, ["Child2", "Child1"]);
}

#[test]
fn save_after_restructure_round_trips_and_preserves_attrs_on_move() {
    let mut doc = EditorDoc::open(repo_data("ImportSample.gde")).expect("open");
    // Reorder Child1 (has color attrs) after Child2.
    doc.move_node(
        &NodePath::from_indices(vec![0, 0]),
        &NodePath::from_indices(vec![0]),
        2,
    )
    .expect("move");
    // Insert a sibling, then delete Child2.
    let inserted = doc
        .insert_child(&NodePath::from_indices(vec![0]), 2)
        .expect("insert");
    doc.set_title(&inserted, "Inserted").expect("title");
    doc.remove_node(&NodePath::from_indices(vec![0, 0]))
        .expect("remove Child2");

    let out = std::env::temp_dir().join("nextguide-restructure.gde");
    doc.save_to(&out).expect("save");
    assert!(!doc.is_dirty());
    let rewritten = std::fs::read(&out).unwrap();
    let _ = std::fs::remove_file(&out);

    check_structural(&rewritten).expect("structural after restructure");
    assert!(
        rewritten
            .windows(color_attr_needle().len())
            .any(|w| w == color_attr_needle()),
        "Preserve-through color attr must move with Child1"
    );

    let again = nextguide_gde::load_bytes(&rewritten).unwrap();
    let titles: Vec<_> = again
        .root()
        .children()[0]
        .children()
        .iter()
        .map(|n| n.title())
        .collect();
    assert_eq!(titles, ["Child1", "Inserted"]);
}

#[test]
fn remove_node_drops_deleted_subtree_from_saved_guide() {
    let mut doc = EditorDoc::open(repo_data("ImportSample.gde")).expect("open");
    // Delete Page1 (and Child1 color attrs with it).
    doc.remove_node(&NodePath::from_indices(vec![0]))
        .expect("remove Page1");
    assert!(doc.is_dirty());
    assert!(doc.visible_tree().is_empty());

    let out = std::env::temp_dir().join("nextguide-restructure-delete.gde");
    doc.save_to(&out).expect("save");
    let rewritten = std::fs::read(&out).unwrap();
    let _ = std::fs::remove_file(&out);

    check_structural(&rewritten).expect("structural after delete");
    assert!(!rewritten.windows(b"Page1".len()).any(|w| w == b"Page1"));
    assert!(!rewritten.windows(b"Child1".len()).any(|w| w == b"Child1"));
}

#[test]
fn undo_title_restores_previous_title_and_dirty() {
    let mut doc = EditorDoc::open(repo_data("ImportSample.gde")).expect("open");
    let page1 = NodePath::from_indices(vec![0]);
    let before = doc.guide().node(&page1).unwrap().title().to_string();
    assert!(!doc.can_undo_tree());
    assert!(!doc.can_redo_tree());

    doc.set_title(&page1, "Page1-renamed").expect("rename");
    assert!(doc.is_dirty());
    assert!(doc.can_undo_tree());
    assert!(!doc.can_redo_tree());

    assert!(doc.undo_tree());
    assert_eq!(doc.guide().node(&page1).unwrap().title(), before);
    assert!(!doc.is_dirty());
    assert!(!doc.can_undo_tree());
    assert!(doc.can_redo_tree());
}

#[test]
fn redo_title_restores_renamed_title() {
    let mut doc = EditorDoc::open(repo_data("ImportSample.gde")).expect("open");
    let page1 = NodePath::from_indices(vec![0]);
    doc.set_title(&page1, "Page1-renamed").expect("rename");
    assert!(doc.undo_tree());
    assert!(doc.redo_tree());
    assert_eq!(doc.guide().node(&page1).unwrap().title(), "Page1-renamed");
    assert!(doc.is_dirty());
    assert!(doc.can_undo_tree());
    assert!(!doc.can_redo_tree());
}

#[test]
fn set_body_does_not_push_tree_undo_stack() {
    let mut doc = EditorDoc::open(repo_data("ImportSample.gde")).expect("open");
    let page1 = NodePath::from_indices(vec![0]);
    doc.set_body(&page1, &Document::plain("body only"))
        .expect("body");
    assert!(doc.is_dirty());
    assert!(!doc.can_undo_tree());
    assert!(!doc.undo_tree());
}

#[test]
fn undo_insert_and_remove_restores_tree() {
    let mut doc = EditorDoc::open(repo_data("ImportSample.gde")).expect("open");
    let page1 = NodePath::from_indices(vec![0]);
    let before_len = doc.guide().node(&page1).unwrap().children().len();

    let inserted = doc.insert_child(&page1, before_len).expect("insert");
    assert_eq!(
        doc.guide().node(&page1).unwrap().children().len(),
        before_len + 1
    );
    assert!(doc.undo_tree());
    assert_eq!(
        doc.guide().node(&page1).unwrap().children().len(),
        before_len
    );
    assert!(doc
        .guide()
        .node(&inserted)
        .is_err());

    doc.remove_node(&NodePath::from_indices(vec![0, 0]))
        .expect("remove Child1");
    assert_eq!(
        doc.guide().node(&page1).unwrap().children().len(),
        before_len - 1
    );
    assert!(doc.undo_tree());
    assert_eq!(
        doc.guide().node(&page1).unwrap().children().len(),
        before_len
    );
    assert_eq!(
        doc.guide()
            .node(&NodePath::from_indices(vec![0, 0]))
            .unwrap()
            .title(),
        "Child1"
    );
}

#[test]
fn undo_move_restores_sibling_order() {
    let mut doc = EditorDoc::open(repo_data("ImportSample.gde")).expect("open");
    let parent = NodePath::from_indices(vec![0]);
    let from = NodePath::from_indices(vec![0, 0]);
    let titles_before: Vec<_> = doc
        .guide()
        .node(&parent)
        .unwrap()
        .children()
        .iter()
        .map(|n| n.title().to_string())
        .collect();

    doc.move_node(&from, &parent, 2).expect("move");
    let titles_moved: Vec<_> = doc
        .guide()
        .node(&parent)
        .unwrap()
        .children()
        .iter()
        .map(|n| n.title().to_string())
        .collect();
    assert_ne!(titles_before, titles_moved);

    assert!(doc.undo_tree());
    let titles_after: Vec<_> = doc
        .guide()
        .node(&parent)
        .unwrap()
        .children()
        .iter()
        .map(|n| n.title().to_string())
        .collect();
    assert_eq!(titles_before, titles_after);
}

#[test]
fn tree_undo_survives_intervening_selection_style_ops() {
    // Selection changes are a UI concern; the app stack itself must keep history
    // across any number of non-tree ops (here: body rewrite).
    let mut doc = EditorDoc::open(repo_data("ImportSample.gde")).expect("open");
    let page1 = NodePath::from_indices(vec![0]);
    let before = doc.guide().node(&page1).unwrap().title().to_string();
    doc.set_title(&page1, "Renamed").expect("title");
    doc.set_body(&page1, &Document::plain("later body"))
        .expect("body");
    assert!(doc.can_undo_tree());
    assert!(doc.undo_tree());
    assert_eq!(doc.guide().node(&page1).unwrap().title(), before);
}

#[test]
fn tree_undo_depth_is_capped() {
    let mut doc = EditorDoc::new();
    let root = NodePath::root();
    for i in 0..(TREE_UNDO_LIMIT + 5) {
        let path = doc.insert_child(&root, i).expect("insert");
        doc.set_title(&path, format!("N{i}")).expect("title");
    }
    let mut undos = 0;
    while doc.undo_tree() {
        undos += 1;
        assert!(undos <= TREE_UNDO_LIMIT);
    }
    assert_eq!(undos, TREE_UNDO_LIMIT);
}

#[test]
fn failed_tree_op_does_not_push_undo() {
    let mut doc = EditorDoc::open(repo_data("ImportSample.gde")).expect("open");
    assert!(doc.remove_node(&NodePath::root()).is_err());
    assert!(!doc.can_undo_tree());
}
