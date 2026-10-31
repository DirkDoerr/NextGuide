//! `nextguide-cli` CLI — Format v1 dump / check.

use std::path::Path;
use std::process::ExitCode;

use nextguide_gde::{check_byte_stable, check_structural, load_path, Guide, Node};

fn main() -> ExitCode {
    match run(std::env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(code) => ExitCode::from(code),
    }
}

fn run(args: Vec<String>) -> Result<(), u8> {
    let mut args = args.into_iter();
    let cmd = args.next().ok_or_else(|| {
        usage();
        2u8
    })?;

    match cmd.as_str() {
        "dump" => {
            let path = require_one_path(args)?;
            dump_file(&path)
        }
        "check" => {
            let mut byte_stable = false;
            let mut path: Option<String> = None;
            for arg in args {
                match arg.as_str() {
                    "--byte-stable" => byte_stable = true,
                    s if s.starts_with('-') => {
                        eprintln!("error: unknown option '{s}'");
                        usage();
                        return Err(2);
                    }
                    s => {
                        if path.is_some() {
                            eprintln!("error: unexpected extra argument '{s}'");
                            usage();
                            return Err(2);
                        }
                        path = Some(s.to_string());
                    }
                }
            }
            let path = path.ok_or_else(|| {
                eprintln!("error: missing <file.gde>");
                usage();
                2u8
            })?;
            check_file(Path::new(&path), byte_stable)
        }
        other => {
            eprintln!("error: unknown command '{other}'");
            usage();
            Err(2)
        }
    }
}

fn require_one_path(mut args: impl Iterator<Item = String>) -> Result<std::path::PathBuf, u8> {
    let path = args.next().ok_or_else(|| {
        eprintln!("error: missing <file.gde>");
        usage();
        2u8
    })?;
    if let Some(extra) = args.next() {
        eprintln!("error: unexpected extra argument '{extra}'");
        usage();
        return Err(2);
    }
    if path.starts_with('-') {
        eprintln!("error: unexpected option '{path}'");
        usage();
        return Err(2);
    }
    Ok(std::path::PathBuf::from(path))
}

fn dump_file(path: &Path) -> Result<(), u8> {
    let guide = load_path(path).map_err(|e| {
        eprintln!("error: {e}");
        1u8
    })?;
    print_tree(guide.root(), 0);
    Ok(())
}

fn print_tree(node: &Node, depth: usize) {
    let indent = "  ".repeat(depth);
    println!(
        "{indent}{} (rtf {} bytes, {} children)",
        node.title(),
        node.rtf_body().len(),
        node.children().len()
    );
    for child in node.children() {
        print_tree(child, depth + 1);
    }
}

fn check_file(path: &Path, byte_stable: bool) -> Result<(), u8> {
    let bytes = std::fs::read(path).map_err(|e| {
        eprintln!("error: I/O error for {}: {e}", path.display());
        1u8
    })?;

    // Fail closed on load before round-trip (helpers also load, but surface a clear message).
    let _: Guide = nextguide_gde::load_bytes(&bytes).map_err(|e| {
        eprintln!("error: {e}");
        1u8
    })?;

    let result = if byte_stable {
        check_byte_stable(&bytes)
    } else {
        check_structural(&bytes)
    };

    match result {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("error: {e}");
            Err(1)
        }
    }
}

fn usage() {
    eprintln!("usage: nextguide-cli dump <file.gde>");
    eprintln!("       nextguide-cli check [--byte-stable] <file.gde>");
}
