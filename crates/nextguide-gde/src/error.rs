//! Typed errors for Format v1 load/save.

use std::fmt;
use std::io;
use std::path::PathBuf;

/// Fail-closed error categories for the Core-model codec.
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
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io { source, .. } => Some(source),
            Error::Unsupported(_) | Error::Corrupt(_) => None,
        }
    }
}
