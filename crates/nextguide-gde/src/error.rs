//! Typed errors for Format v1 load/save.

use std::fmt;
use std::io;
use std::path::PathBuf;

/// Fail-closed error categories for the Core-model codec and mutations.
#[derive(Debug)]
pub enum Error {
    /// Filesystem or other I/O failure.
    Io {
        /// Optional path that failed.
        path: Option<PathBuf>,
        /// Underlying I/O error.
        source: io::Error,
    },
    /// Supported generation is The Guide v2 only.
    Unsupported(String),
    /// Truncated, garbage, or otherwise invalid structure.
    Corrupt(String),
    /// Node path does not address a Node in this Guide.
    InvalidPath(String),
    /// Mutation is illegal for this Guide (Dummy root protected, cycle, bad index).
    InvalidEdit(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io { path: Some(path), source } => {
                write!(f, "I/O error for {}: {source}", path.display())
            }
            Error::Io { path: None, source } => write!(f, "I/O error: {source}"),
            Error::Unsupported(msg) => write!(f, "unsupported: {msg}"),
            Error::Corrupt(msg) => write!(f, "corrupt: {msg}"),
            Error::InvalidPath(msg) => write!(f, "invalid path: {msg}"),
            Error::InvalidEdit(msg) => write!(f, "invalid edit: {msg}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io { source, .. } => Some(source),
            Error::Unsupported(_)
            | Error::Corrupt(_)
            | Error::InvalidPath(_)
            | Error::InvalidEdit(_) => None,
        }
    }
}
