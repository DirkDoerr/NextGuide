//! Editor v1 application helpers shared by the Relm4 shell.
//!
//! Widget code lives in the binary; this library stays free of GTK types so
//! CI can exercise the in-process `nextguide-gde` link without a display.

use std::path::Path;

use nextguide_gde::{Guide, Result};

/// GApplication / Relm4 application id.
pub const APP_ID: &str = "org.nextguide.NextGuide";

/// Window title used for an untitled New Guide session.
pub fn untitled_window_title() -> &'static str {
    "Untitled"
}

/// Window title for the current document path (`None` = untitled).
pub fn window_title_for_path(path: Option<&Path>) -> String {
    match path.and_then(|p| p.file_name()) {
        Some(name) if !name.is_empty() => name.to_string_lossy().into_owned(),
        _ => untitled_window_title().to_string(),
    }
}

/// Load a Guide through Format v1 (in-process `nextguide-gde`).
pub fn load_guide(path: impl AsRef<Path>) -> Result<Guide> {
    nextguide_gde::load_path(path)
}
