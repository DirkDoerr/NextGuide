//! Editor helpers at the `nextguide` ↔ `nextguide-gde` / `nextguide-rtf` seam (no GTK).

use std::path::Path;
use std::path::PathBuf;

use nextguide_gde::{check_structural, NodePath};
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
    assert_eq!(window_title_for_path(None), "Untitled");
}

#[test]
fn open_path_window_title_uses_file_name() {
    assert_eq!(
        window_title_for_path(Some(Path::new("/tmp/ImportSample.gde"))),
        "ImportSample.gde"
    );
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
    let doc = EditorDoc::open(&src).expect("open");
    let before_rtf = doc.guide().root().children()[0].rtf_body().to_vec();

    let out = std::env::temp_dir().join("nextguide-save-unchanged.gde");
    doc.save_unchanged_to(&out).expect("save");
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
