# 15: Scaffold `nextguide` Relm4/GTK app

**What to build:** A Relm4/GTK desktop binary named **`nextguide`** that launches a window and depends on `nextguide-gde` in-process (stub UI is enough), so later tickets have an app crate to land in. Thin smoke if headless GTK is practical.

**Blocked by:** 12 — Rename Round-trip CLI to `nextguide-cli`

**Status:** resolved

- [x] Workspace builds binary `nextguide` (GTK4 + Relm4) linking `nextguide-gde`
- [x] App starts a window on a developer machine with GTK available
- [x] Thin smoke test exists or is explicitly deferred with a documented manual check if CI cannot run GTK yet

## Answer

Added workspace crate `crates/nextguide` (binary name **`nextguide`**) depending on `nextguide-gde`, with optional Relm4/GTK4 shell behind the default `ui` feature. Stub RelmApp window uses title `Untitled` and a placeholder label.

CI / headless: workspace `default-members` are only `nextguide-gde` + `nextguide-cli`, so plain `cargo test` does not need GTK. CI also runs `cargo test -p nextguide --no-default-features` (window-title + fixture load via `load_guide`). Full UI smoke is **deferred** — manual check documented in README: install GTK4, `cargo run -p nextguide`, confirm Untitled stub window.
