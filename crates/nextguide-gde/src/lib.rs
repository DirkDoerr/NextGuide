//! Format v1 Core-model codec for The Guide v2 `.gde` files.

#![deny(missing_docs)]

mod codec;
mod error;
mod model;
mod roundtrip;

pub use error::Error;
pub use model::{Guide, Node};
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
}
