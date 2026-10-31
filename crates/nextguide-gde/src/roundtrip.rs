//! Shared Structural and Byte-stable Round-trip helpers.

use crate::{load_bytes, save_bytes, Error, Guide, Result};

/// Structural Round-trip: `load → save → load` preserves Core model.
pub fn check_structural(bytes: &[u8]) -> Result<()> {
    let first = load_bytes(bytes)?;
    let rewritten = save_bytes(&first)?;
    let second = load_bytes(&rewritten)?;
    assert_core_equal(&first, &second)
}

/// Byte-stable Round-trip: `load → save` bytes equal the input.
pub fn check_byte_stable(bytes: &[u8]) -> Result<()> {
    let guide = load_bytes(bytes)?;
    let rewritten = save_bytes(&guide)?;
    if rewritten == bytes {
        Ok(())
    } else {
        Err(Error::Corrupt(
            "byte-stable round-trip failed: output bytes differ from input".into(),
        ))
    }
}

fn assert_core_equal(a: &Guide, b: &Guide) -> Result<()> {
    assert_node_equal(a.root(), b.root())
}

fn assert_node_equal(a: &crate::Node, b: &crate::Node) -> Result<()> {
    if a.title() != b.title() {
        return Err(Error::Corrupt(format!(
            "structural mismatch: title {:?} vs {:?}",
            a.title(),
            b.title()
        )));
    }
    if a.rtf_body() != b.rtf_body() {
        return Err(Error::Corrupt(format!(
            "structural mismatch: rtf body lengths {} vs {} for title {:?}",
            a.rtf_body().len(),
            b.rtf_body().len(),
            a.title()
        )));
    }
    if a.children().len() != b.children().len() {
        return Err(Error::Corrupt(format!(
            "structural mismatch: child count {} vs {} for title {:?}",
            a.children().len(),
            b.children().len(),
            a.title()
        )));
    }
    for (ca, cb) in a.children().iter().zip(b.children().iter()) {
        assert_node_equal(ca, cb)?;
    }
    Ok(())
}
