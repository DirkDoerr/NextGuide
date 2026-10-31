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

/// Character styles in the pragmatic subset.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SpanStyle {
    /// Bold.
    pub bold: bool,
    /// Italic.
    pub italic: bool,
    /// Underline.
    pub underline: bool,
}
