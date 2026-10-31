//! Edit model → pragmatic subset RTF (The Guide / Msftedit-friendly).

use crate::model::{Block, Document, Run, SpanStyle};

/// Minimal valid empty body The Guide accepts (`{\rtf1}`-family).
pub fn empty_rtf() -> Vec<u8> {
    b"{\\rtf1\\ansi\\pard\\par}\n".to_vec()
}

/// Serialize an edit-model document to subset RTF bytes.
pub fn write(doc: &Document) -> Vec<u8> {
    let mut out = String::from("{\\rtf1\\ansi\\deff0\n");
    let mut number = 1u32;

    for block in &doc.blocks {
        match block {
            Block::Paragraph(runs) => {
                out.push_str("\\pard\n");
                write_runs(&mut out, runs);
                out.push_str("\\par\n");
            }
            Block::BulletItem(runs) => {
                out.push_str(
                    "\\pard{\\pntext\\f0\\'B7\\tab}{\\*\\pn\\pnlvlblt\\pnf0\\pnindent0{\\pntxtb\\'B7}}\n",
                );
                out.push_str("\\fi-360\\li360\n");
                write_runs(&mut out, runs);
                out.push_str("\\par\n");
            }
            Block::NumberedItem(runs) => {
                out.push_str(&format!(
                    "\\pard{{\\pntext\\f0 {number}.\\tab}}{{\\*\\pn\\pnlvlbody\\pnf0\\pnindent0\\pnstart1\\pndec{{\\pntxta.}}}}\n"
                ));
                out.push_str("\\fi-360\\li360\n");
                write_runs(&mut out, runs);
                out.push_str("\\par\n");
                number += 1;
            }
        }
    }

    if doc.blocks.is_empty() {
        out.push_str("\\pard\\par\n");
    }

    out.push('}');
    out.into_bytes()
}

fn write_runs(out: &mut String, runs: &[Run]) {
    let mut cur = SpanStyle::default();
    for run in runs {
        emit_style_delta(out, cur, run.style);
        cur = run.style;
        escape_text(out, &run.text);
    }
    emit_style_delta(out, cur, SpanStyle::default());
}

fn emit_style_delta(out: &mut String, from: SpanStyle, to: SpanStyle) {
    if from.bold != to.bold {
        out.push_str(if to.bold { "\\b " } else { "\\b0 " });
    }
    if from.italic != to.italic {
        out.push_str(if to.italic { "\\i " } else { "\\i0 " });
    }
    if from.underline != to.underline {
        out.push_str(if to.underline {
            "\\ul "
        } else {
            "\\ulnone "
        });
    }
}

fn escape_text(out: &mut String, text: &str) {
    for ch in text.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '{' => out.push_str("\\{"),
            '}' => out.push_str("\\}"),
            '\t' => out.push_str("\\tab "),
            '\n' => out.push_str("\\line "),
            c if (c as u32) < 128 && !c.is_control() => out.push(c),
            c => {
                let code = c as u32;
                if code <= 0xFF {
                    out.push_str(&format!("\\'{code:02x}"));
                } else {
                    // Signed 16-bit \uN plus ASCII fallback.
                    let n = code as i32;
                    let signed = if n > 32767 { n - 65536 } else { n };
                    out.push_str(&format!("\\u{signed}?"));
                }
            }
        }
    }
}
