# 10: Scaffold Format v1 Cargo workspace

**What to build:** A Rust workspace that builds with a Core-model library crate and a `nextguide` binary crate (stubs are enough), plus a smoke test that can resolve Guide fixtures under `data/`, so later tickets have a place to land.

**Blocked by:** None (can start immediately)

**Status:** resolved

- [x] Cargo workspace builds with library `nextguide-gde` and binary `nextguide`
- [x] Minimal-dependencies posture is in place (no unnecessary crates)
- [x] A smoke test (or equivalent) can locate fixtures under `data/`

## Answer

Workspace at repo root with `crates/nextguide-gde` (lib) and `crates/nextguide` (bin); zero third-party deps; `data_fixture_import_sample_exists` resolves `data/ImportSample.gde`.
