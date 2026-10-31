//! Thin CLI smoke tests at the `nextguide-cli` binary seam.

use std::path::PathBuf;
use std::process::Command;

fn repo_data(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.pop();
    path.push("data");
    path.push(name);
    path
}

fn bin() -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_nextguide-cli"));
    cmd.current_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."));
    cmd
}

#[test]
fn dump_import_sample_exits_0_and_prints_tree() {
    let out = bin()
        .args(["dump", "data/ImportSample.gde"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("dummy"));
    assert!(stdout.contains("Page1"));
    assert!(stdout.contains("Grandchild"));
}

#[test]
fn dump_missing_file_exits_1() {
    let out = bin()
        .args(["dump", "data/missing-nope.gde"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
    assert!(!out.stderr.is_empty());
}

#[test]
fn dump_usage_exits_2() {
    let out = bin().args(["dump"]).output().unwrap();
    assert_eq!(out.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("nextguide-cli"),
        "usage should name the renamed binary: {stderr}"
    );
}

#[test]
fn check_import_sample_structural_exits_0() {
    let out = bin()
        .args(["check", "data/ImportSample.gde"])
        .output()
        .unwrap();
    assert_eq!(
        out.status.code(),
        Some(0),
        "{:?}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn check_golden_byte_stable_exits_0() {
    assert!(repo_data("canonical-golden.gde").is_file());
    let out = bin()
        .args(["check", "--byte-stable", "data/canonical-golden.gde"])
        .output()
        .unwrap();
    assert_eq!(
        out.status.code(),
        Some(0),
        "{:?}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn check_usage_exits_2() {
    let out = bin().args(["check"]).output().unwrap();
    assert_eq!(out.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("nextguide-cli"),
        "usage should name the renamed binary: {stderr}"
    );
}

#[test]
fn check_load_failure_exits_1() {
    let out = bin()
        .args(["check", "data/missing-nope.gde"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
    assert!(!out.stderr.is_empty());
}

#[test]
fn unknown_command_exits_2() {
    let out = bin().args(["nope"]).output().unwrap();
    assert_eq!(out.status.code(), Some(2));
}
