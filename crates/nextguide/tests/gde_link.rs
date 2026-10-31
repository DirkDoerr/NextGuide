//! Editor helpers at the `nextguide` ↔ `nextguide-gde` / `nextguide-rtf` seam (no GTK).

use std::path::Path;
use std::path::PathBuf;

use nextguide_gde::{check_structural, NodePath};
use nextguide_rtf::{Document, Run, SpanStyle};
use nextguide::{
    body_view_from_rtf, load_guide, visible_tree, window_title_for_path, EditorDoc,
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
