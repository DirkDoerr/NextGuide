//! Core-model Guide and Node types (walk/inspect only).

use std::fmt;

/// In-memory Guide: ordered tree rooted at the Dummy root.
#[derive(Clone)]
pub struct Guide {
    pub(crate) root: Node,
    pub(crate) preserve: crate::codec::PreserveBag,
}

impl Guide {
    /// Dummy root of the on-disk tree.
    pub fn root(&self) -> &Node {
        &self.root
    }
}

impl fmt::Debug for Guide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Omit Preserve-through bags from Debug (not part of the Core-model API).
        f.debug_struct("Guide").field("root", &self.root).finish()
    }
}

/// One Node in the Core-model tree.
#[derive(Clone)]
pub struct Node {
    pub(crate) title: String,
    pub(crate) rtf_body: Vec<u8>,
    pub(crate) children: Vec<Node>,
    pub(crate) preserve: crate::codec::NodePreserve,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("title", &self.title)
            .field("rtf_body_len", &self.rtf_body.len())
            .field("children", &self.children)
            .finish()
    }
}

impl Node {
    /// Node title (UTF-8).
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Opaque RTF body bytes.
    pub fn rtf_body(&self) -> &[u8] {
        &self.rtf_body
    }

    /// Ordered children.
    pub fn children(&self) -> &[Node] {
        &self.children
    }
}
