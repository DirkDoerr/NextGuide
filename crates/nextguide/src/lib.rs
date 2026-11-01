//! Editor v1 application helpers shared by the Relm4 shell.
//!
//! Widget code lives in the binary; this library stays free of GTK types so
//! CI can exercise the in-process `nextguide-gde` / `nextguide-rtf` link without a display.

mod presentation;
mod recent;
mod session;

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use nextguide_gde::{Guide, Result};

pub use presentation::{
    present, ActionOutcome, EditorSnapshot, FocusedPane, FontFamilyControl, FontSizeControl,
    FormatToggle, FormattingControls, FormattingSnapshot, ListLineKind, Presentation,
    StructureRefusal,
};
pub use recent::{RecentFiles, RECENT_LIMIT};
pub use session::{
    body_view, body_view_from_rtf, visible_tree, BodyView, EditorDoc, TreeRow, TREE_UNDO_LIMIT,
    UNDO_DEPTH_LIMIT,
};

/// GApplication / Relm4 application id.
pub const APP_ID: &str = "org.nextguide.NextGuide";

/// User choice from a dirty Save / Discard / Cancel prompt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirtyPromptChoice {
    /// Persist the current Guide, then proceed with the pending action.
    Save,
    /// Drop unsaved edits and proceed.
    Discard,
    /// Abort the pending action; keep the current Guide.
    Cancel,
}

impl DirtyPromptChoice {
    /// Button / menu label for this choice.
    pub fn label(self) -> &'static str {
        match self {
            Self::Save => "Save",
            Self::Discard => "Discard",
            Self::Cancel => "Cancel",
        }
    }
}

/// Whether New / Open / quit must prompt before replacing or closing the Guide.
pub fn needs_dirty_prompt(dirty: bool) -> bool {
    dirty
}

/// Window title used for an untitled New Guide session.
pub fn untitled_window_title() -> &'static str {
    "Untitled"
}

/// Window title for the current document path (`None` = untitled).
///
/// When `dirty` is true, appends ` *` so the unsaved indicator is visible.
pub fn window_title_for_path(path: Option<&Path>, dirty: bool) -> String {
    let base = match path.and_then(|p| p.file_name()) {
        Some(name) if !name.is_empty() => name.to_string_lossy().into_owned(),
        _ => untitled_window_title().to_string(),
    };
    if dirty {
        format!("{base} *")
    } else {
        base
    }
}

/// Load a Guide through Format v1 (in-process `nextguide-gde`).
pub fn load_guide(path: impl AsRef<Path>) -> Result<Guide> {
    nextguide_gde::load_path(path)
}

/// First CLI path argument after the program name, if any.
///
/// `None` means bare launch → untitled New Guide. Does not validate that the
/// path exists or is a `.gde`.
pub fn launch_path_from_args(args: impl IntoIterator<Item = OsString>) -> Option<PathBuf> {
    let mut args = args.into_iter();
    let _program = args.next();
    args.next().map(PathBuf::from)
}

/// Default on-disk store for the recent-files list (`$XDG_CONFIG_HOME/nextguide/recent`
/// or `~/.config/nextguide/recent`).
pub fn default_recent_store_path() -> PathBuf {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("nextguide").join("recent")
}
