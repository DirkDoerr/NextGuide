//! Editor helpers at the `nextguide` ↔ `nextguide-gde` seam (no GTK).

use std::path::Path;
use std::path::PathBuf;

use nextguide::{load_guide, window_title_for_path};

fn repo_data(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.pop();
    path.push("data");
    path.push(name);
    path
}

#[test]
fn bare_launch_window_title_is_untitled() {
    assert_eq!(window_title_for_path(None), "Untitled");
}

#[test]
fn open_path_window_title_uses_file_name() {
    assert_eq!(
        window_title_for_path(Some(Path::new("/tmp/ImportSample.gde"))),
        "ImportSample.gde"
    );
}

#[test]
fn load_guide_links_nextguide_gde_on_fixture() {
    let guide = load_guide(repo_data("ImportSample.gde")).expect("load via editor helper");
    assert_eq!(guide.root().title(), "dummy");
    assert_eq!(guide.root().children()[0].title(), "Page1");
}
