//! Toolkit-agnostic edit model for the Editor v1 pragmatic RTF subset.

/// A Node body as editable rich text (bold / italic / underline / paragraphs / simple lists).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Document {
    /// Ordered blocks (paragraphs and list items).
    pub blocks: Vec<Block>,
}

impl Document {
    /// Empty document (one empty paragraph).
    pub fn empty() -> Self {
        Self {
            blocks: vec![Block::Paragraph(Vec::new())],
        }
    }

    /// Plain-text document (single paragraph, no styles).
    pub fn plain(text: impl Into<String>) -> Self {
        let text = text.into();
        if text.is_empty() {
            return Self::empty();
        }
        Self {
            blocks: vec![Block::Paragraph(vec![Run {
                text,
                style: SpanStyle::default(),
            }])],
        }
    }
}

/// One block in the document.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Block {
    /// Ordinary paragraph.
    Paragraph(Vec<Run>),
    /// Bullet list item.
    BulletItem(Vec<Run>),
    /// Numbered list item (writer renumbers in document order).
    NumberedItem(Vec<Run>),
}

impl Block {
    /// Styled runs in this block.
    pub fn runs(&self) -> &[Run] {
        match self {
            Block::Paragraph(runs) | Block::BulletItem(runs) | Block::NumberedItem(runs) => runs,
        }
    }
}

/// A contiguous styled text run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Run {
    /// UTF-8 text (may be empty).
    pub text: String,
    /// Character formatting for this run.
    pub style: SpanStyle,
}

/// Font family in the Editor v1 subset (Arial or portable system Sans).
///
/// RTF mapping (see `write`): Arial is font-table entry `\f0` (`\fnil\fcharset0 Arial`,
/// as The Guide writes it); Sans is `\f1` (`\fswiss\fcharset0 Helvetica`, a portable
/// sans name). On load any non-Arial, non-Symbol face maps to [`FontFamily::Sans`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FontFamily {
    /// Arial (default; matches The Guide bodies).
    #[default]
    Arial,
    /// System default sans-serif face.
    Sans,
}

impl FontFamily {
    /// Every family offered in the Format UI, in menu order.
    pub const ALL: [FontFamily; 2] = [FontFamily::Arial, FontFamily::Sans];

    /// Human-readable UI label.
    pub fn label(self) -> &'static str {
        match self {
            FontFamily::Arial => "Arial",
            FontFamily::Sans => "System Sans",
        }
    }
}

/// Font size restricted to the agreed point sizes (see [`FontSize::ALLOWED_PT`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FontSize(u8);

impl FontSize {
    /// Allowed sizes in points, ascending.
    pub const ALLOWED_PT: [u8; 7] = [9, 10, 11, 12, 14, 18, 24];

    /// Default size (9 pt, i.e. The Guide's common `\fs18`).
    pub const DEFAULT: FontSize = FontSize(9);

    /// Every size offered in the Format UI, ascending (same order as [`Self::ALLOWED_PT`]).
    pub const ALL: [FontSize; 7] = [
        FontSize(9),
        FontSize(10),
        FontSize(11),
        FontSize(12),
        FontSize(14),
        FontSize(18),
        FontSize(24),
    ];

    /// Size for an allowed point value, or `None` if `pt` is not in the set.
    pub fn from_pt(pt: u8) -> Option<FontSize> {
        Self::ALLOWED_PT.contains(&pt).then_some(FontSize(pt))
    }

    /// Nearest allowed size for an RTF `\fsN` half-point value (ties pick the smaller
    /// size; non-positive values give the default).
    pub fn snap_half_points(half_points: i32) -> FontSize {
        if half_points <= 0 {
            return Self::DEFAULT;
        }
        let mut best = Self::ALLOWED_PT[0];
        let mut best_dist = i32::MAX;
        for pt in Self::ALLOWED_PT {
            let dist = (i32::from(pt) * 2 - half_points).abs();
            if dist < best_dist {
                best = pt;
                best_dist = dist;
            }
        }
        FontSize(best)
    }

    /// Size in points.
    pub fn pt(self) -> u8 {
        self.0
    }

    /// Size in RTF half-points (`\fsN`).
    pub fn half_points(self) -> u16 {
        u16::from(self.0) * 2
    }
}

impl Default for FontSize {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Character styles in the pragmatic subset.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SpanStyle {
    /// Bold.
    pub bold: bool,
    /// Italic.
    pub italic: bool,
    /// Underline.
    pub underline: bool,
    /// Font family.
    pub family: FontFamily,
    /// Font size.
    pub size: FontSize,
}
