//! Format v1 Core-model codec for The Guide v2 `.gde` files.

#![deny(missing_docs)]

mod codec;
mod error;
mod model;
mod path;
mod roundtrip;

pub use error::Error;
pub use model::{Guide, Node};
pub use path::NodePath;
pub use roundtrip::{check_byte_stable, check_structural};

/// Result alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Load a Guide from a filesystem path.
pub fn load_path(path: impl AsRef<std::path::Path>) -> Result<Guide> {
    let path = path.as_ref();
    let bytes = std::fs::read(path).map_err(|source| Error::Io {
        path: Some(path.to_path_buf()),
        source,
    })?;
    load_bytes(&bytes)
}

/// Load a Guide from an in-memory `.gde` buffer.
pub fn load_bytes(bytes: &[u8]) -> Result<Guide> {
    codec::load(bytes)
}

/// Save a Guide to a filesystem path.
pub fn save_path(guide: &Guide, path: impl AsRef<std::path::Path>) -> Result<()> {
    let path = path.as_ref();
    let bytes = save_bytes(guide)?;
    std::fs::write(path, bytes).map_err(|source| Error::Io {
        path: Some(path.to_path_buf()),
        source,
    })
}

/// Save a Guide to an in-memory `.gde` buffer.
pub fn save_bytes(guide: &Guide) -> Result<Vec<u8>> {
    codec::save(guide)
}

#[cfg(test)]
mod fixture_path {
    use std::path::PathBuf;

    /// Resolve a fixture under the repo `data/` directory.
    pub fn data_fixture(name: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.pop(); // crates
        path.pop(); // repo root
        path.push("data");
        path.push(name);
        path
    }
}

#[cfg(test)]
mod tests {
    use super::fixture_path::data_fixture;
    use super::*;

    #[test]
    fn data_fixture_import_sample_exists() {
        let path = data_fixture("ImportSample.gde");
        assert!(
            path.is_file(),
            "expected fixture at {}",
            path.display()
        );
    }

    #[test]
    fn load_import_sample_exposes_dummy_root_and_tree() {
        let guide = load_path(data_fixture("ImportSample.gde")).expect("load");
        let root = guide.root();
        assert_eq!(root.title(), "dummy");
        assert_eq!(root.rtf_body(), b"dummy");
        assert_eq!(root.children().len(), 1);

        let first = &root.children()[0];
        assert_eq!(first.title(), "Page1");
        assert!(first.rtf_body().starts_with(b"{\\rtf1"));
        assert_eq!(first.children().len(), 2);
        assert_eq!(first.children()[0].title(), "Child1");
        assert_eq!(first.children()[1].title(), "Child2");
        assert_eq!(first.children()[0].children()[0].title(), "Grandchild");
    }

    #[test]
    fn load_bytes_matches_load_path_for_import_sample() {
        let path = data_fixture("ImportSample.gde");
        let bytes = std::fs::read(&path).unwrap();
        let from_path = load_path(&path).unwrap();
        let from_bytes = load_bytes(&bytes).unwrap();
        assert_eq!(from_path.root().title(), from_bytes.root().title());
        assert_eq!(
            from_path.root().children().len(),
            from_bytes.root().children().len()
        );
    }

    #[test]
    fn load_rejects_truncated_as_corrupt() {
        let err = load_bytes(b"GDE").expect_err("truncated");
        assert!(matches!(err, Error::Corrupt(_)), "{err}");
    }

    #[test]
    fn load_rejects_garbage_as_corrupt() {
        let err = load_bytes(b"NOTAGUIDEFILE!!!!!!!!!!").expect_err("garbage");
        assert!(matches!(err, Error::Corrupt(_)), "{err}");
    }

    #[test]
    fn load_rejects_v1_marker_as_unsupported() {
        // Format-1 detection uses the 16-byte marker at offset 4.
        let mut buf = vec![0u8; 20];
        buf[4..20].copy_from_slice(b"\0\0\0\0dummy\0dummy\0");
        let err = load_bytes(&buf).expect_err("v1");
        assert!(matches!(err, Error::Unsupported(_)), "{err}");
    }

    #[test]
    fn load_rejects_non_v2_version_as_unsupported() {
        let mut buf = Vec::new();
        buf.extend_from_slice(b"GDE");
        buf.extend_from_slice(&3u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        // pad to >= 20
        buf.resize(20, 0);
        let err = load_bytes(&buf).expect_err("v3");
        assert!(matches!(err, Error::Unsupported(_)), "{err}");
    }

    #[test]
    fn load_missing_path_is_io() {
        let err = load_path(data_fixture("does-not-exist.gde")).expect_err("io");
        assert!(matches!(err, Error::Io { .. }), "{err}");
    }

    #[test]
    fn structural_round_trip_import_sample() {
        let bytes = std::fs::read(data_fixture("ImportSample.gde")).unwrap();
        check_structural(&bytes).expect("structural round-trip");
    }

    #[test]
    fn save_path_round_trips_core_model() {
        let src = data_fixture("ImportSample.gde");
        let guide = load_path(&src).unwrap();
        let dir = std::env::temp_dir().join("nextguide-gde-save-path-test.gde");
        save_path(&guide, &dir).unwrap();
        let again = load_path(&dir).unwrap();
        let _ = std::fs::remove_file(&dir);
        assert_eq!(again.root().title(), "dummy");
        assert_eq!(again.root().children()[0].title(), "Page1");
        assert_eq!(
            again.root().children()[0].children()[0].title(),
            "Child1"
        );
    }

    #[test]
    fn writer_is_idempotent_after_canonicalize() {
        // Preserve-through + canonical ids: second save must match first.
        let bytes = std::fs::read(data_fixture("ImportSample.gde")).unwrap();
        let once = save_bytes(&load_bytes(&bytes).unwrap()).unwrap();
        let twice = save_bytes(&load_bytes(&once).unwrap()).unwrap();
        assert_eq!(once, twice);
    }

    #[test]
    fn byte_stable_round_trip_owned_golden() {
        let bytes = std::fs::read(data_fixture("canonical-golden.gde")).unwrap();
        check_byte_stable(&bytes).expect("byte-stable");
        check_structural(&bytes).expect("structural on golden");
        let guide = load_bytes(&bytes).unwrap();
        assert_eq!(guide.root().title(), "dummy");
        assert_eq!(guide.root().children()[0].title(), "Hello");
    }

    #[test]
    fn preserve_through_keeps_node_attr_payloads_on_disk() {
        // ImportSample Child1 has bgcolor/color attrs of 0xffffffff (LE).
        let bytes = std::fs::read(data_fixture("ImportSample.gde")).unwrap();
        let rewritten = save_bytes(&load_bytes(&bytes).unwrap()).unwrap();
        let needle = {
            let mut v = Vec::new();
            v.extend_from_slice(&4u32.to_le_bytes()); // color attr id
            v.extend_from_slice(&4u32.to_le_bytes()); // len
            v.extend_from_slice(&0xffff_ffffu32.to_le_bytes());
            v
        };
        assert!(
            rewritten.windows(needle.len()).any(|w| w == needle),
            "expected Preserve-through of color attr 0xffffffff in saved bytes"
        );
    }

    fn color_attr_needle() -> Vec<u8> {
        let mut v = Vec::new();
        v.extend_from_slice(&4u32.to_le_bytes());
        v.extend_from_slice(&4u32.to_le_bytes());
        v.extend_from_slice(&0xffff_ffffu32.to_le_bytes());
        v
    }

    #[test]
    fn new_empty_guide_has_dummy_root_only() {
        let guide = Guide::new_empty();
        assert_eq!(guide.root().title(), "dummy");
        assert_eq!(guide.root().rtf_body(), b"dummy");
        assert!(guide.root().children().is_empty());
        assert_eq!(guide.node(&NodePath::root()).unwrap().title(), "dummy");
    }

    #[test]
    fn set_title_and_rtf_body_update_core_fields() {
        let mut guide = load_path(data_fixture("ImportSample.gde")).unwrap();
        let page1 = NodePath::from_indices(vec![0]);
        guide.set_title(&page1, "Renamed").unwrap();
        guide.set_rtf_body(&page1, b"{\\rtf1 new}").unwrap();
        assert_eq!(guide.node(&page1).unwrap().title(), "Renamed");
        assert_eq!(guide.node(&page1).unwrap().rtf_body(), b"{\\rtf1 new}");
    }

    #[test]
    fn set_title_rejects_invalid_path() {
        let mut guide = Guide::new_empty();
        let err = guide
            .set_title(&NodePath::from_indices(vec![0]), "x")
            .expect_err("missing child");
        assert!(matches!(err, Error::InvalidPath(_)), "{err}");
    }

    #[test]
    fn insert_child_appends_and_returns_path() {
        let mut guide = Guide::new_empty();
        let path = guide.insert_child(&NodePath::root(), 0).unwrap();
        assert_eq!(path, NodePath::from_indices(vec![0]));
        let node = guide.node(&path).unwrap();
        assert_eq!(node.title(), "");
        assert!(node.rtf_body().is_empty());
        assert!(node.children().is_empty());
    }

    #[test]
    fn insert_child_rejects_index_past_len() {
        let mut guide = Guide::new_empty();
        let err = guide
            .insert_child(&NodePath::root(), 1)
            .expect_err("past end");
        assert!(matches!(err, Error::InvalidEdit(_)), "{err}");
    }

    #[test]
    fn remove_node_drops_subtree_and_refuses_dummy_root() {
        let mut guide = load_path(data_fixture("ImportSample.gde")).unwrap();
        let child1 = NodePath::from_indices(vec![0, 0]);
        assert_eq!(guide.node(&child1).unwrap().title(), "Child1");
        guide.remove_node(&child1).unwrap();
        assert_eq!(guide.root().children()[0].children().len(), 1);
        assert_eq!(guide.root().children()[0].children()[0].title(), "Child2");

        let err = guide
            .remove_node(&NodePath::root())
            .expect_err("dummy root");
        assert!(matches!(err, Error::InvalidEdit(_)), "{err}");
    }

    #[test]
    fn move_node_reorders_same_parent_with_index_adjustment() {
        let mut guide = load_path(data_fixture("ImportSample.gde")).unwrap();
        // Under Page1: Child1, Child2 — move Child1 after Child2.
        let from = NodePath::from_indices(vec![0, 0]);
        let parent = NodePath::from_indices(vec![0]);
        let new_path = guide.move_node(&from, &parent, 2).unwrap();
        assert_eq!(new_path, NodePath::from_indices(vec![0, 1]));
        let titles: Vec<_> = guide
            .root()
            .children()[0]
            .children()
            .iter()
            .map(|n| n.title())
            .collect();
        assert_eq!(titles, ["Child2", "Child1"]);
        // Grandchild stays under Child1.
        assert_eq!(
            guide.node(&NodePath::from_indices(vec![0, 1, 0]))
                .unwrap()
                .title(),
            "Grandchild"
        );
    }

    #[test]
    fn move_node_reparents_across_parents() {
        let mut guide = load_path(data_fixture("ImportSample.gde")).unwrap();
        // Move Grandchild ([0,0,0]) under Child2 ([0,1]).
        let from = NodePath::from_indices(vec![0, 0, 0]);
        let to_parent = NodePath::from_indices(vec![0, 1]);
        let new_path = guide.move_node(&from, &to_parent, 0).unwrap();
        assert_eq!(new_path, NodePath::from_indices(vec![0, 1, 0]));
        assert!(guide
            .node(&NodePath::from_indices(vec![0, 0]))
            .unwrap()
            .children()
            .is_empty());
        assert_eq!(
            guide.node(&new_path).unwrap().title(),
            "Grandchild"
        );
    }

    #[test]
    fn move_node_refuses_dummy_root_and_into_descendant() {
        let mut guide = load_path(data_fixture("ImportSample.gde")).unwrap();
        let err = guide
            .move_node(&NodePath::root(), &NodePath::from_indices(vec![0]), 0)
            .expect_err("dummy");
        assert!(matches!(err, Error::InvalidEdit(_)), "{err}");

        let page1 = NodePath::from_indices(vec![0]);
        let child1 = NodePath::from_indices(vec![0, 0]);
        let err = guide
            .move_node(&page1, &child1, 0)
            .expect_err("into descendant");
        assert!(matches!(err, Error::InvalidEdit(_)), "{err}");
    }

    #[test]
    fn title_edit_preserves_non_core_attrs_on_disk() {
        let bytes = std::fs::read(data_fixture("ImportSample.gde")).unwrap();
        let mut guide = load_bytes(&bytes).unwrap();
        guide
            .set_title(&NodePath::from_indices(vec![0, 0]), "Child1-renamed")
            .unwrap();
        let rewritten = save_bytes(&guide).unwrap();
        assert!(
            rewritten
                .windows(color_attr_needle().len())
                .any(|w| w == color_attr_needle()),
            "color attr must survive title edit"
        );
        check_structural(&rewritten).expect("structural after title edit");
    }

    #[test]
    fn rtf_edit_preserves_non_core_attrs_on_disk() {
        let bytes = std::fs::read(data_fixture("ImportSample.gde")).unwrap();
        let mut guide = load_bytes(&bytes).unwrap();
        guide
            .set_rtf_body(&NodePath::from_indices(vec![0, 0]), b"{\\rtf1 edited}")
            .unwrap();
        let rewritten = save_bytes(&guide).unwrap();
        assert!(
            rewritten
                .windows(color_attr_needle().len())
                .any(|w| w == color_attr_needle()),
            "color attr must survive RTF edit"
        );
        check_structural(&rewritten).expect("structural after RTF edit");
    }

    fn file_attr_id1_needle() -> Vec<u8> {
        // ImportSample file attr id=1 value=5 (LE).
        let mut v = Vec::new();
        v.extend_from_slice(&1u32.to_le_bytes());
        v.extend_from_slice(&4u32.to_le_bytes());
        v.extend_from_slice(&5u32.to_le_bytes());
        v
    }

    #[test]
    fn guide_file_attrs_survive_core_edits() {
        let bytes = std::fs::read(data_fixture("ImportSample.gde")).unwrap();
        assert!(bytes
            .windows(file_attr_id1_needle().len())
            .any(|w| w == file_attr_id1_needle()));
        let mut guide = load_bytes(&bytes).unwrap();
        guide
            .set_title(&NodePath::from_indices(vec![0]), "Page1-x")
            .unwrap();
        guide
            .insert_child(&NodePath::from_indices(vec![0]), 0)
            .unwrap();
        let rewritten = save_bytes(&guide).unwrap();
        assert!(
            rewritten
                .windows(file_attr_id1_needle().len())
                .any(|w| w == file_attr_id1_needle()),
            "file-level Preserve-through must survive Core edits"
        );
    }

    #[test]
    fn insert_child_starts_empty_and_round_trips() {
        let mut guide = Guide::new_empty();
        let path = guide.insert_child(&NodePath::root(), 0).unwrap();
        assert_eq!(guide.node(&path).unwrap().title(), "");
        assert!(guide.node(&path).unwrap().rtf_body().is_empty());
        let saved = save_bytes(&guide).unwrap();
        check_structural(&saved).expect("structural after empty insert");
        let again = load_bytes(&saved).unwrap();
        assert_eq!(again.root().children().len(), 1);
        assert_eq!(again.root().children()[0].title(), "");
        assert!(again.root().children()[0].rtf_body().is_empty());
    }

    #[test]
    fn move_preserves_non_core_attrs_with_node() {
        let bytes = std::fs::read(data_fixture("ImportSample.gde")).unwrap();
        let mut guide = load_bytes(&bytes).unwrap();
        // Move Child1 (has color attrs) after Child2.
        guide
            .move_node(
                &NodePath::from_indices(vec![0, 0]),
                &NodePath::from_indices(vec![0]),
                2,
            )
            .unwrap();
        let rewritten = save_bytes(&guide).unwrap();
        assert!(
            rewritten
                .windows(color_attr_needle().len())
                .any(|w| w == color_attr_needle()),
            "color attr must move with Child1"
        );
        check_structural(&rewritten).expect("structural after move");
    }

    #[test]
    fn remove_drops_preserve_through_for_deleted_subtree() {
        let bytes = std::fs::read(data_fixture("ImportSample.gde")).unwrap();
        let before = save_bytes(&load_bytes(&bytes).unwrap()).unwrap();
        assert!(before
            .windows(color_attr_needle().len())
            .any(|w| w == color_attr_needle()));

        let mut guide = load_bytes(&bytes).unwrap();
        // Delete entire Page1 subtree (includes Child1 color attrs).
        guide
            .remove_node(&NodePath::from_indices(vec![0]))
            .unwrap();
        let after = save_bytes(&guide).unwrap();
        // New empty children use default attrs which also include 0xffffffff colors.
        // Prove Child1 title is gone and Structural Round-trip still holds.
        assert!(!after.windows(b"Child1".len()).any(|w| w == b"Child1"));
        assert!(!after.windows(b"Page1".len()).any(|w| w == b"Page1"));
        check_structural(&after).expect("structural after delete");
    }

    #[test]
    fn insert_then_structural_round_trip_import_sample() {
        let bytes = std::fs::read(data_fixture("ImportSample.gde")).unwrap();
        let mut guide = load_bytes(&bytes).unwrap();
        let inserted = guide
            .insert_child(&NodePath::from_indices(vec![0]), 1)
            .unwrap();
        guide.set_title(&inserted, "Inserted").unwrap();
        guide.set_rtf_body(&inserted, b"{\\rtf1 insert}").unwrap();
        let saved = save_bytes(&guide).unwrap();
        check_structural(&saved).expect("structural after insert");
        let again = load_bytes(&saved).unwrap();
        assert_eq!(again.root().children()[0].children()[1].title(), "Inserted");
    }

    #[test]
    fn failed_edit_leaves_guide_unchanged() {
        let mut guide = load_path(data_fixture("ImportSample.gde")).unwrap();
        let before = save_bytes(&guide).unwrap();
        let _ = guide
            .move_node(
                &NodePath::from_indices(vec![0]),
                &NodePath::from_indices(vec![0, 0]),
                0,
            )
            .expect_err("cycle");
        let after = save_bytes(&guide).unwrap();
        assert_eq!(before, after);
    }
}
