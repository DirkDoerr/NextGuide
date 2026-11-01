//! Pragmatic subset RTF → edit model.

use crate::model::{Block, Document, Run, SpanStyle};
use crate::LoadBody;

/// Load RTF bytes into an edit model, or soft-fail with a plain-text fallback.
pub fn load(rtf: &[u8]) -> LoadBody {
    let text = match std::str::from_utf8(rtf) {
        Ok(s) => s,
        Err(_) => {
            // Lossy UTF-8 for soft UI fallback; original bytes stay with the caller.
            let lossy = String::from_utf8_lossy(rtf).into_owned();
            return LoadBody::Unparseable {
                plain_fallback: strip_controls_rough(&lossy),
            };
        }
    };

    let trimmed = text.trim_start();
    if !trimmed.starts_with("{\\rtf") {
        return LoadBody::Unparseable {
            plain_fallback: strip_controls_rough(trimmed),
        };
    }

    match parse_document(trimmed) {
        Ok(doc) => LoadBody::Editable(doc),
        Err(_) => LoadBody::Unparseable {
            plain_fallback: strip_controls_rough(trimmed),
        },
    }
}

#[derive(Debug)]
struct ParseError;

fn parse_document(input: &str) -> Result<Document, ParseError> {
    let mut p = Parser::new(input);
    p.expect(b'{')?;
    p.control_word()?; // rtf1 / rtf
    let mut style = SpanStyle::default();
    let mut blocks = Vec::new();
    let mut runs = Vec::new();
    let mut list_kind: Option<ListKind> = None;
    let mut depth = 1i32;

    while p.pos < p.bytes.len() && depth > 0 {
        match p.peek() {
            Some(b'{') => {
                p.bump();
                depth += 1;
                if p.starts_with(b"\\*\\pn") {
                    // List destination: capture bullet vs numbered, then skip the group.
                    list_kind = Some(p.peek_list_kind_in_group());
                    p.skip_group_from_inside(&mut depth)?;
                } else if p.starts_with(b"\\fonttbl")
                    || p.starts_with(b"\\*")
                    || p.starts_with(b"\\pntext")
                {
                    // Font table / destinations / list markers: skip (font names must not
                    // become Run text).
                    p.skip_group_from_inside(&mut depth)?;
                }
            }
            Some(b'}') => {
                p.bump();
                depth -= 1;
            }
            Some(b'\\') => {
                let word = p.control_word()?;
                match word.as_str() {
                    "par" => {
                        flush_block(&mut blocks, &mut runs, list_kind);
                        list_kind = None;
                        style = SpanStyle::default();
                    }
                    "pard" => {
                        // Reset paragraph; keep open runs until \par.
                        list_kind = None;
                    }
                    "b" => style.bold = word_on(&p),
                    "i" => style.italic = word_on(&p),
                    "ul" => style.underline = true,
                    "ulnone" => style.underline = false,
                    "tab" => push_text(&mut runs, style, "\t"),
                    "line" => push_text(&mut runs, style, "\n"),
                    "'" => {
                        if let Some(ch) = p.hex_char()? {
                            push_text(&mut runs, style, &ch.to_string());
                        }
                    }
                    "u" => {
                        if let Some(ch) = p.unicode_char()? {
                            push_text(&mut runs, style, &ch.to_string());
                        }
                    }
                    _ => {
                        // Unknown / destination controls: ignore (best-effort subset).
                    }
                }
            }
            Some(b'\r') | Some(b'\n') => {
                p.bump();
            }
            Some(_) => {
                let ch = p.char()?;
                if !ch.is_control() {
                    push_text(&mut runs, style, &ch.to_string());
                }
            }
            None => break,
        }
    }

    if !runs.is_empty() || blocks.is_empty() {
        flush_block(&mut blocks, &mut runs, list_kind);
    }

    Ok(Document { blocks })
}

#[derive(Clone, Copy)]
enum ListKind {
    Bullet,
    Numbered,
}

fn word_on(p: &Parser<'_>) -> bool {
    // Control words like \b / \b0: Parser stores numeric suffix separately.
    p.last_param.map(|n| n != 0).unwrap_or(true)
}

fn push_text(runs: &mut Vec<Run>, style: SpanStyle, text: &str) {
    if text.is_empty() {
        return;
    }
    if let Some(last) = runs.last_mut() {
        if last.style == style {
            last.text.push_str(text);
            return;
        }
    }
    runs.push(Run {
        text: text.to_string(),
        style,
    });
}

fn flush_block(blocks: &mut Vec<Block>, runs: &mut Vec<Run>, list_kind: Option<ListKind>) {
    let taken = std::mem::take(runs);
    let block = match list_kind {
        Some(ListKind::Bullet) => Block::BulletItem(taken),
        Some(ListKind::Numbered) => Block::NumberedItem(taken),
        None => Block::Paragraph(taken),
    };
    blocks.push(block);
}

struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
    last_param: Option<i32>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            bytes: input.as_bytes(),
            pos: 0,
            last_param: None,
        }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn bump(&mut self) {
        self.pos += 1;
    }

    fn starts_with(&self, prefix: &[u8]) -> bool {
        self.bytes[self.pos..].starts_with(prefix)
    }

    fn expect(&mut self, b: u8) -> Result<(), ParseError> {
        if self.peek() != Some(b) {
            return Err(ParseError);
        }
        self.bump();
        Ok(())
    }

    fn char(&mut self) -> Result<char, ParseError> {
        let s = std::str::from_utf8(&self.bytes[self.pos..]).map_err(|_| ParseError)?;
        let ch = s.chars().next().ok_or(ParseError)?;
        self.pos += ch.len_utf8();
        Ok(ch)
    }

    /// Read a control word (without the leading backslash). Sets `last_param`.
    fn control_word(&mut self) -> Result<String, ParseError> {
        self.expect(b'\\')?;
        self.last_param = None;

        if self.peek() == Some(b'\'') {
            self.bump();
            return Ok("'".into());
        }

        // Single-char controls: \\ \{ \} \~
        match self.peek() {
            Some(b'\\') | Some(b'{') | Some(b'}') | Some(b'~') | Some(b'_') | Some(b'-') => {
                let c = self.char()?;
                return Ok(c.to_string());
            }
            _ => {}
        }

        let mut name = String::new();
        while let Some(b) = self.peek() {
            if b.is_ascii_alphabetic() {
                name.push(b as char);
                self.bump();
            } else {
                break;
            }
        }
        if name.is_empty() {
            // Unknown one-byte control; consume it.
            let _ = self.char()?;
            return Ok(String::new());
        }

        // Optional numeric parameter.
        let mut sign = 1i32;
        if self.peek() == Some(b'-') {
            sign = -1;
            self.bump();
        }
        if self.peek().is_some_and(|b| b.is_ascii_digit()) {
            let mut n = 0i32;
            while let Some(b) = self.peek() {
                if b.is_ascii_digit() {
                    n = n.saturating_mul(10).saturating_add((b - b'0') as i32);
                    self.bump();
                } else {
                    break;
                }
            }
            self.last_param = Some(sign * n);
        }

        // Optional space delimiter after control word.
        if self.peek() == Some(b' ') {
            self.bump();
        }
        Ok(name)
    }

    fn hex_char(&mut self) -> Result<Option<char>, ParseError> {
        // \'hh already consumed '\'' as word; read two hex digits from here.
        // Actually control_word for ' returns "'" and leaves hh.
        let h1 = self.peek().ok_or(ParseError)?;
        self.bump();
        let h2 = self.peek().ok_or(ParseError)?;
        self.bump();
        let hi = hex_val(h1).ok_or(ParseError)?;
        let lo = hex_val(h2).ok_or(ParseError)?;
        Ok(Some((hi * 16 + lo) as char))
    }

    fn unicode_char(&mut self) -> Result<Option<char>, ParseError> {
        let n = self.last_param.ok_or(ParseError)?;
        // \uN is followed by an ANSI fallback char to skip (often '?').
        if self.peek().is_some() {
            let _ = self.char()?;
        }
        let code = if n < 0 { (n as i32 + 65536) as u32 } else { n as u32 };
        Ok(char::from_u32(code))
    }

    /// Scan the current open group for `\pnlvlblt` / `\pnlvlbody` without consuming.
    fn peek_list_kind_in_group(&self) -> ListKind {
        let slice = &self.bytes[self.pos..];
        if find_control(slice, b"pnlvlblt") {
            ListKind::Bullet
        } else {
            ListKind::Numbered
        }
    }

    fn skip_group_from_inside(&mut self, depth: &mut i32) -> Result<(), ParseError> {
        // We are already inside the `{` that opened this group; depth includes it.
        let start_depth = *depth;
        while self.pos < self.bytes.len() && *depth >= start_depth {
            match self.peek() {
                Some(b'{') => {
                    self.bump();
                    *depth += 1;
                }
                Some(b'}') => {
                    self.bump();
                    *depth -= 1;
                }
                Some(b'\\') => {
                    let _ = self.control_word()?;
                }
                Some(_) => self.bump(),
                None => break,
            }
        }
        Ok(())
    }
}

fn find_control(hay: &[u8], name: &[u8]) -> bool {
    let mut i = 0;
    while i + 1 + name.len() <= hay.len() {
        if hay[i] == b'\\' && hay[i + 1..].starts_with(name) {
            let after = i + 1 + name.len();
            let ok_end = after >= hay.len()
                || !hay[after].is_ascii_alphabetic();
            if ok_end {
                return true;
            }
        }
        i += 1;
    }
    false
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn strip_controls_rough(s: &str) -> String {
    // Best-effort plain text for soft UI fallback.
    let mut out = String::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'{' => {
                // Skip fonttbl / ignorable destinations so font names never dominate fallback.
                let rest = &bytes[i + 1..];
                if rest.starts_with(b"\\fonttbl") || rest.starts_with(b"\\*") {
                    i = skip_braced_group(bytes, i);
                    continue;
                }
                i += 1;
            }
            b'}' => i += 1,
            b'\\' => {
                i += 1;
                if i < bytes.len() && bytes[i] == b'\'' {
                    i += 1;
                    if i + 1 < bytes.len() {
                        if let (Some(hi), Some(lo)) = (hex_val(bytes[i]), hex_val(bytes[i + 1])) {
                            out.push((hi * 16 + lo) as char);
                            i += 2;
                            continue;
                        }
                    }
                }
                while i < bytes.len() && bytes[i].is_ascii_alphabetic() {
                    i += 1;
                }
                if i < bytes.len() && bytes[i] == b'-' {
                    i += 1;
                }
                while i < bytes.len() && bytes[i].is_ascii_digit() {
                    i += 1;
                }
                if i < bytes.len() && bytes[i] == b' ' {
                    i += 1;
                }
            }
            b'\r' | b'\n' => i += 1,
            c => {
                out.push(c as char);
                i += 1;
            }
        }
    }
    out
}

/// Advance past a `{…}` group starting at `open` (index of `{`). Returns index after `}`.
fn skip_braced_group(bytes: &[u8], open: usize) -> usize {
    let mut depth = 0i32;
    let mut i = open;
    while i < bytes.len() {
        match bytes[i] {
            b'{' => {
                depth += 1;
                i += 1;
            }
            b'}' => {
                depth -= 1;
                i += 1;
                if depth == 0 {
                    return i;
                }
            }
            b'\\' => {
                // Keep brace depth correct inside skipped groups (\{ \} \\).
                i += 1;
                if i < bytes.len() {
                    i += 1;
                }
            }
            _ => i += 1,
        }
    }
    i
}
