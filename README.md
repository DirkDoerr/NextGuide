# NextGuide

A hierarchical tree based note taking SW.

This application is based on the project:

https://theguide.sourceforge.net/

in terms of functionality and allowing to read/write the same file format.

It targets Windows, MacOS and Linux. Optionally Android and iOS.

It is also an exercise in building with AI.

## Format v1

Format v1 is a greenfield library and CLI for The Guide **v2** `.gde` Core model (tree, titles, opaque RTF bodies).

Requires a stable Rust toolchain (`rust-toolchain.toml`; install via rustup or Homebrew `rust`).

- Library: `nextguide-gde` (`crates/nextguide-gde`)
- CLI: `nextguide-cli` (`crates/nextguide-cli`) — `dump` and `check` (`--byte-stable` for owned goldens)
- Editor: `nextguide` (`crates/nextguide`) — GTK4 + Relm4 stub (Editor v1)
- Fixtures: `data/`

```bash
cargo test
cargo test -p nextguide --no-default-features
cargo run -p nextguide-cli -- dump data/ImportSample.gde
cargo run -p nextguide-cli -- check data/ImportSample.gde
cargo run -p nextguide-cli -- check --byte-stable data/canonical-golden.gde
```

### Editor (`nextguide`)

Needs a system GTK4 development install to build the UI (`ui` feature, on by default for that package). Workspace `default-members` exclude the editor so plain `cargo test` stays headless-friendly.

```bash
# macOS (Homebrew): brew install gtk4
# Debian/Ubuntu: sudo apt install libgtk-4-dev
cargo run -p nextguide
```

**Manual check (UI smoke deferred in CI):** launch `nextguide`, confirm a window titled `Untitled` with the stub label, then quit.

