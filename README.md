# NextGuide

A hierarchical tree based note taking SW.

This application is based on the project:

https://theguide.sourceforge.net/

in terms of functionality and allowing to read/write the same file format.

It targets Windows, MacOS and Linux. Optionally Android and iOS.

It is also an exercise in building with AI.

## Format v1

Format v1 is a greenfield library and CLI for The Guide **v2** `.gde` Core model (tree, titles, opaque RTF bodies). No editor UI.

- Library: `nextguide-gde` (`crates/nextguide-gde`)
- CLI: `nextguide` (`crates/nextguide`) — `dump` and `check` (`--byte-stable` for owned goldens)
- Fixtures: `data/`

```bash
cargo test
cargo run -p nextguide -- dump data/ImportSample.gde
cargo run -p nextguide -- check data/ImportSample.gde
cargo run -p nextguide -- check --byte-stable data/canonical-golden.gde
```

