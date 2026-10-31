//! Public Node path addressing for Core-model mutations.

use std::fmt;
use std::ops::Deref;

/// Address of a Node: 0-based child indices from the Dummy root (`[]` = Dummy root).
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct NodePath(Vec<usize>);

impl NodePath {
    /// Empty path — the Dummy root.
    pub fn root() -> Self {
        Self(Vec::new())
    }

    /// Path from a sequence of child indices.
    pub fn from_indices(indices: impl Into<Vec<usize>>) -> Self {
        Self(indices.into())
    }

    /// Child indices from the Dummy root.
    pub fn indices(&self) -> &[usize] {
        &self.0
    }

    /// Whether this path is the Dummy root.
    pub fn is_root(&self) -> bool {
        self.0.is_empty()
    }

    /// Parent path, if any (`None` for the Dummy root).
    pub fn parent(&self) -> Option<NodePath> {
        if self.0.is_empty() {
            None
        } else {
            let mut p = self.0.clone();
            p.pop();
            Some(NodePath(p))
        }
    }

    /// Final child index within the parent, if any.
    pub fn last_index(&self) -> Option<usize> {
        self.0.last().copied()
    }

    /// Path to the `index`-th child of this path.
    pub fn child(&self, index: usize) -> NodePath {
        let mut p = self.0.clone();
        p.push(index);
        NodePath(p)
    }
}

impl Deref for NodePath {
    type Target = [usize];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<usize>> for NodePath {
    fn from(indices: Vec<usize>) -> Self {
        Self(indices)
    }
}

impl From<&[usize]> for NodePath {
    fn from(indices: &[usize]) -> Self {
        Self(indices.to_vec())
    }
}

impl fmt::Display for NodePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            write!(f, "[]")
        } else {
            write!(f, "[")?;
            for (i, idx) in self.0.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{idx}")?;
            }
            write!(f, "]")
        }
    }
}
