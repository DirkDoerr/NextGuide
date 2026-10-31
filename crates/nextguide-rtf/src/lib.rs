//! Pragmatic RTF ↔ edit-model conversion for Editor v1.
//!
//! Public API is toolkit-agnostic (no GTK types). The Relm4 editor maps the
//! [`Document`] model onto GTK `TextBuffer` / `TextTag` separately.

#![deny(missing_docs)]

mod model;
mod parse;
mod write;

pub use model::{Block, Document, Run, SpanStyle};
pub use parse::load;
pub use write::{empty_rtf, write};

/// Result of loading opaque RTF body bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadBody {
    /// Parsed into the pragmatic edit subset (exotic attrs may be dropped on rewrite).
    Editable(Document),
    /// Soft-fail: UI may show [`plain_fallback`](LoadBody::Unparseable::plain_fallback);
    /// keep the original bytes until the user intentionally replaces the body.
    Unparseable {
        /// Best-effort plain text for display.
        plain_fallback: String,
    },
}

/// Bytes to persist for a Node body under Editor v1 Round-trip policy.
///
/// - `edited = None` → return `original` unchanged (untouched-bytes).
/// - `edited = Some(doc)` → subset rewrite via [`write`].
pub fn bytes_for_save(original: &[u8], edited: Option<&Document>) -> Vec<u8> {
    match edited {
        None => original.to_vec(),
        Some(doc) => write(doc),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bold_hello() -> Document {
        Document {
            blocks: vec![Block::Paragraph(vec![Run {
                text: "Hello".into(),
                style: SpanStyle {
                    bold: true,
                    ..SpanStyle::default()
                },
            }])],
        }
    }

    #[test]
    fn empty_rtf_is_minimal_valid() {
        let bytes = empty_rtf();
        assert!(bytes.starts_with(b"{\\rtf1"));
        match load(&bytes) {
            LoadBody::Editable(doc) => {
                assert!(!doc.blocks.is_empty());
            }
            LoadBody::Unparseable { .. } => panic!("empty_rtf must parse"),
        }
    }

    #[test]
    fn subset_round_trip_bold_italic_underline_paragraphs() {
        let doc = Document {
            blocks: vec![
                Block::Paragraph(vec![
                    Run {
                        text: "bold".into(),
                        style: SpanStyle {
                            bold: true,
                            ..Default::default()
                        },
                    },
                    Run {
                        text: " ".into(),
                        style: SpanStyle::default(),
                    },
                    Run {
                        text: "italic".into(),
                        style: SpanStyle {
                            italic: true,
                            ..Default::default()
                        },
                    },
                ]),
                Block::Paragraph(vec![Run {
                    text: "under".into(),
                    style: SpanStyle {
                        underline: true,
                        ..Default::default()
                    },
                }]),
            ],
        };
        let bytes = write(&doc);
        let LoadBody::Editable(again) = load(&bytes) else {
            panic!("rewrite must parse");
        };
        assert_eq!(again.blocks.len(), 2);
        assert_eq!(again.blocks[0].runs()[0].text, "bold");
        assert!(again.blocks[0].runs()[0].style.bold);
        assert!(again.blocks[0].runs().iter().any(|r| r.style.italic && r.text == "italic"));
        assert_eq!(again.blocks[1].runs()[0].text, "under");
        assert!(again.blocks[1].runs()[0].style.underline);
    }

    #[test]
    fn subset_round_trip_simple_lists() {
        let doc = Document {
            blocks: vec![
                Block::BulletItem(vec![Run {
                    text: "one".into(),
                    style: SpanStyle::default(),
                }]),
                Block::NumberedItem(vec![Run {
                    text: "two".into(),
                    style: SpanStyle::default(),
                }]),
            ],
        };
        let bytes = write(&doc);
        let LoadBody::Editable(again) = load(&bytes) else {
            panic!("list rewrite must parse");
        };
        assert!(matches!(again.blocks[0], Block::BulletItem(_)));
        assert!(matches!(again.blocks[1], Block::NumberedItem(_)));
        assert_eq!(again.blocks[0].runs()[0].text, "one");
        assert_eq!(again.blocks[1].runs()[0].text, "two");
    }

    #[test]
    fn load_guide_style_empty_paragraph_body() {
        let sample = b"{\\rtf1\\ansi\\ansicpg1252\\deff0{\\fonttbl{\\f0\\fnil\\fcharset0 Arial;}}\r\n{\\*\\generator Msftedit 5.41.21.2500;}\\viewkind4\\uc1\\pard\\lang1031\\fs18\\par\r\n}\r\n";
        match load(sample) {
            LoadBody::Editable(doc) => {
                assert!(!doc.blocks.is_empty());
            }
            LoadBody::Unparseable { plain_fallback } => {
                panic!("expected editable, got soft-fail {plain_fallback:?}");
            }
        }
    }

    #[test]
    fn load_guide_bold_hello_fragment() {
        let sample = b"{\\rtf1\\ansi\\deff0{\\fonttbl{\\f0\\fnil\\fcharset0 Arial;}}\r\n\\pard\\b Hello\\b0\\par\r\n}\r\n";
        let LoadBody::Editable(doc) = load(sample) else {
            panic!("expected editable");
        };
        let runs = doc.blocks[0].runs();
        assert!(runs.iter().any(|r| r.text.contains("Hello") && r.style.bold));
    }

    #[test]
    fn unparseable_soft_fails_with_plain_fallback() {
        match load(b"not rtf at all") {
            LoadBody::Unparseable { plain_fallback } => {
                assert!(plain_fallback.contains("not rtf"));
            }
            LoadBody::Editable(_) => panic!("should soft-fail"),
        }
    }

    #[test]
    fn untouched_bytes_policy_returns_original() {
        let original = b"{\\rtf1\\ansi exotic \\highlight1 keep me\\par}";
        assert_eq!(bytes_for_save(original, None), original);
    }

    #[test]
    fn edited_bytes_policy_rewrites_subset() {
        let original = b"{\\rtf1\\ansi old\\par}";
        let saved = bytes_for_save(original, Some(&bold_hello()));
        assert_ne!(saved, original);
        assert!(saved.windows(b"Hello".len()).any(|w| w == b"Hello"));
        assert!(saved.windows(b"\\b".len()).any(|w| w == b"\\b"));
    }

    #[test]
    fn document_plain_helper() {
        let doc = Document::plain("hi");
        assert_eq!(doc.blocks[0].runs()[0].text, "hi");
    }
}
