//! Pragmatic RTF ↔ edit-model conversion for Editor v1.
//!
//! Public API is toolkit-agnostic (no GTK types). The Relm4 editor maps the
//! [`Document`] model onto GTK `TextBuffer` / `TextTag` separately.

#![deny(missing_docs)]

mod model;
mod parse;
mod write;

pub use model::{Block, Document, FontFamily, FontSize, Run, SpanStyle};
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

    fn all_run_text(doc: &Document) -> String {
        doc.blocks
            .iter()
            .flat_map(|b| b.runs().iter().map(|r| r.text.as_str()))
            .collect()
    }

    #[test]
    fn load_guide_style_empty_paragraph_body() {
        let sample = b"{\\rtf1\\ansi\\ansicpg1252\\deff0{\\fonttbl{\\f0\\fnil\\fcharset0 Arial;}}\r\n{\\*\\generator Msftedit 5.41.21.2500;}\\viewkind4\\uc1\\pard\\lang1031\\fs18\\par\r\n}\r\n";
        match load(sample) {
            LoadBody::Editable(doc) => {
                assert!(!doc.blocks.is_empty());
                let text = all_run_text(&doc);
                assert!(
                    !text.contains("Arial") && !text.contains(';'),
                    "fonttbl must not leak into empty body runs: {text:?}"
                );
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
        let text = all_run_text(&doc);
        assert!(
            !text.contains("Arial") && text == "Hello",
            "fonttbl must not leak before body text: {text:?}"
        );
    }

    #[test]
    fn load_guide_fonttbl_does_not_leak_arial_into_runs() {
        // The Guide–shaped header: font table + real content.
        let sample = b"{\\rtf1\\ansi\\ansicpg1252\\deff0{\\fonttbl{\\f0\\fnil\\fcharset0 Arial;}}\r\n\\pard\\f0\\fs18 Hello\\par\r\n}\r\n";
        let LoadBody::Editable(doc) = load(sample) else {
            panic!("expected editable");
        };
        let text = all_run_text(&doc);
        assert_eq!(text, "Hello");
        assert!(!text.contains("Arial"));
        assert!(!text.contains(';'));
    }

    #[test]
    fn load_colortbl_semicolons_do_not_leak_into_runs() {
        // The Guide writes `{\colortbl ;\red0…;}` whose entries are `;`-terminated.
        let sample = b"{\\rtf1\\ansi\\deff0{\\fonttbl{\\f0\\fnil\\fcharset0 Arial;}}\r\n{\\colortbl ;\\red0\\green0\\blue255;\\red255\\green255\\blue255;}\r\n\\pard\\cf1 Hello\\par\r\n}\r\n";
        let LoadBody::Editable(doc) = load(sample) else {
            panic!("expected editable");
        };
        assert_eq!(all_run_text(&doc), "Hello");
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
    fn soft_fail_strip_drops_fonttbl_names() {
        // Not a well-formed `{\rtf…}` document, so load soft-fails; fallback must
        // still strip fonttbl destinations rather than promote "Arial;" as content.
        let sample = "{\\fonttbl{\\f0\\fnil\\fcharset0 Arial;}}Hello";
        match load(sample.as_bytes()) {
            LoadBody::Unparseable { plain_fallback } => {
                assert!(
                    !plain_fallback.contains("Arial"),
                    "fonttbl name leaked into fallback: {plain_fallback:?}"
                );
                assert!(plain_fallback.contains("Hello"));
            }
            LoadBody::Editable(_) => panic!("expected soft-fail for non-rtf wrapper"),
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

    fn styled_run(text: &str, family: FontFamily, pt: u8) -> Run {
        Run {
            text: text.into(),
            style: SpanStyle {
                family,
                size: FontSize::from_pt(pt).expect("allowed size"),
                ..SpanStyle::default()
            },
        }
    }

    fn reload(doc: &Document) -> Document {
        match load(&write(doc)) {
            LoadBody::Editable(d) => d,
            LoadBody::Unparseable { plain_fallback } => {
                panic!("rewrite must parse, got {plain_fallback:?}")
            }
        }
    }

    #[test]
    fn default_style_is_arial_nine_point() {
        let style = SpanStyle::default();
        assert_eq!(style.family, FontFamily::Arial);
        assert_eq!(style.size.pt(), 9);
        assert_eq!(style.size.half_points(), 18);
    }

    #[test]
    fn sans_label_says_it_is_the_system_face() {
        assert_eq!(FontFamily::Arial.label(), "Arial");
        assert_eq!(FontFamily::Sans.label(), "System Sans");
    }

    #[test]
    fn font_size_all_mirrors_allowed_pt() {
        let pts: Vec<u8> = FontSize::ALL.iter().map(|s| s.pt()).collect();
        assert_eq!(pts, FontSize::ALLOWED_PT);
    }

    #[test]
    fn round_trip_arial_twelve_point() {
        let doc = Document {
            blocks: vec![Block::Paragraph(vec![styled_run("Hi", FontFamily::Arial, 12)])],
        };
        let bytes = write(&doc);
        let text = String::from_utf8(bytes).unwrap();
        assert!(text.contains("\\fs24"), "12pt is \\fs24: {text}");
        assert_eq!(reload(&doc), doc);
    }

    #[test]
    fn round_trip_sans_and_every_allowed_size() {
        for pt in FontSize::ALLOWED_PT {
            for family in [FontFamily::Arial, FontFamily::Sans] {
                let doc = Document {
                    blocks: vec![Block::Paragraph(vec![styled_run("x", family, pt)])],
                };
                assert_eq!(reload(&doc), doc, "{family:?} {pt}pt");
            }
        }
    }

    #[test]
    fn round_trip_mixed_family_size_and_bold_in_one_paragraph() {
        let mut bold_sans = styled_run("b", FontFamily::Sans, 18);
        bold_sans.style.bold = true;
        let doc = Document {
            blocks: vec![
                Block::Paragraph(vec![
                    styled_run("a", FontFamily::Arial, 9),
                    bold_sans,
                    styled_run("c", FontFamily::Arial, 24),
                ]),
                Block::BulletItem(vec![styled_run("d", FontFamily::Sans, 14)]),
                Block::NumberedItem(vec![styled_run("e", FontFamily::Sans, 10)]),
            ],
        };
        assert_eq!(reload(&doc), doc);
    }

    #[test]
    fn writer_declares_font_table_with_arial_as_f0() {
        let text = String::from_utf8(write(&Document::plain("x"))).unwrap();
        assert!(text.contains("{\\fonttbl{\\f0\\fnil\\fcharset0 Arial;}"), "{text}");
        assert!(text.contains("\\fswiss"), "Sans must be a swiss family: {text}");
    }

    #[test]
    fn load_missing_fs_is_nine_point_default() {
        let sample = b"{\\rtf1\\ansi\\pard Plain\\par}";
        let LoadBody::Editable(doc) = load(sample) else { panic!() };
        assert_eq!(doc.blocks[0].runs()[0].style.size.pt(), 9);
        assert_eq!(doc.blocks[0].runs()[0].style.family, FontFamily::Arial);
    }

    #[test]
    fn load_guide_f0_fs18_is_arial_nine() {
        let sample = b"{\\rtf1\\ansi\\deff0{\\fonttbl{\\f0\\fnil\\fcharset0 Arial;}}\r\n\\pard\\f0\\fs18 Hello\\par\r\n}\r\n";
        let LoadBody::Editable(doc) = load(sample) else { panic!() };
        let style = doc.blocks[0].runs()[0].style;
        assert_eq!((style.family, style.size.pt()), (FontFamily::Arial, 9));
    }

    #[test]
    fn load_fs_in_half_points() {
        let sample = b"{\\rtf1\\ansi\\pard\\fs24 Twelve\\par}";
        let LoadBody::Editable(doc) = load(sample) else { panic!() };
        assert_eq!(doc.blocks[0].runs()[0].style.size.pt(), 12);
    }

    #[test]
    fn load_out_of_set_size_snaps_to_nearest_allowed() {
        // ImportSample uses \fs44 (22pt): nearest allowed is 24pt.
        let sample = b"{\\rtf1\\ansi\\pard\\fs44 Demo\\par\\pard\\fs30 Odd\\par}";
        let LoadBody::Editable(doc) = load(sample) else { panic!() };
        assert_eq!(doc.blocks[0].runs()[0].style.size.pt(), 24);
        assert_eq!(doc.blocks[1].runs()[0].style.size.pt(), 14);
    }

    #[test]
    fn load_unknown_non_arial_face_is_sans_but_symbol_is_not_a_body_face() {
        let sample = b"{\\rtf1\\ansi\\deff0{\\fonttbl{\\f0\\fnil\\fcharset0 Arial;}{\\f1\\fnil\\fcharset2 Symbol;}{\\f2\\froman Times New Roman;}}\\pard\\f2 serif\\f1 sym\\par}";
        let LoadBody::Editable(doc) = load(sample) else { panic!() };
        let runs = doc.blocks[0].runs();
        assert_eq!(runs[0].style.family, FontFamily::Sans);
        // Symbol is not a body face: the run stays Sans (previous face), never mapped.
        assert_eq!(all_run_text(&doc), "serifsym");
        assert_eq!(runs.len(), 1);
    }

    #[test]
    fn char_formatting_persists_across_par() {
        let sample = b"{\\rtf1\\ansi{\\fonttbl{\\f0 Arial;}{\\f1\\fswiss Helvetica;}}\\pard\\f1\\fs24\\b one\\par\\pard two\\par}";
        let LoadBody::Editable(doc) = load(sample) else { panic!() };
        for block in &doc.blocks {
            let style = block.runs()[0].style;
            assert_eq!(style.family, FontFamily::Sans);
            assert_eq!(style.size.pt(), 12);
            assert!(style.bold);
        }
    }

    #[test]
    fn char_formatting_is_scoped_to_braced_groups() {
        let sample = b"{\\rtf1\\ansi\\pard a{\\b\\fs48 b}c\\par}";
        let LoadBody::Editable(doc) = load(sample) else { panic!() };
        let runs = doc.blocks[0].runs();
        assert_eq!(runs.len(), 3);
        assert!(!runs[0].style.bold && !runs[2].style.bold);
        assert_eq!(runs[1].style.size.pt(), 24);
        assert_eq!(runs[2].style.size.pt(), 9);
    }

    #[test]
    fn untouched_bytes_stay_verbatim_with_font_controls() {
        let original = b"{\\rtf1\\ansi\\pard\\f0\\fs44 Demo\\par}";
        assert_eq!(bytes_for_save(original, None), original);
    }
}
