//! Recent Guides list (MRU, capped at five) with optional on-disk store.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Maximum recent Guides shown in the session menu.
pub const RECENT_LIMIT: usize = 5;

/// Most-recently-used list of Guide paths.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RecentFiles {
    paths: Vec<PathBuf>,
}

impl RecentFiles {
    /// Empty list.
    pub fn new() -> Self {
        Self { paths: Vec::new() }
    }

    /// Paths in MRU order (newest first).
    pub fn paths(&self) -> &[PathBuf] {
        &self.paths
    }

    /// Record `path` as most recent (dedupe, cap at [`RECENT_LIMIT`]).
    pub fn record(&mut self, path: impl Into<PathBuf>) {
        let path = path.into();
        self.paths.retain(|p| p != &path);
        self.paths.insert(0, path);
        self.paths.truncate(RECENT_LIMIT);
    }

    /// Load newline-separated absolute/relative paths from `store`.
    ///
    /// Missing file → empty list. Blank lines ignored.
    pub fn load_from(store: impl AsRef<Path>) -> io::Result<Self> {
        let store = store.as_ref();
        if !store.exists() {
            return Ok(Self::new());
        }
        let text = fs::read_to_string(store)?;
        let mut recent = Self::new();
        // File is stored newest-first; record in reverse so MRU order is preserved.
        let lines: Vec<_> = text
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .collect();
        for line in lines.into_iter().rev() {
            recent.record(PathBuf::from(line));
        }
        Ok(recent)
    }

    /// Persist newest-first paths, one per line.
    pub fn save_to(&self, store: impl AsRef<Path>) -> io::Result<()> {
        let store = store.as_ref();
        if let Some(parent) = store.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut body = String::new();
        for path in &self.paths {
            body.push_str(&path.to_string_lossy());
            body.push('\n');
        }
        fs::write(store, body)
    }
}
