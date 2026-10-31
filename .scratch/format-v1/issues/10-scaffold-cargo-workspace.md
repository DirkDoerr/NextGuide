# 10: Scaffold Format v1 Cargo workspace

**What to build:** A Rust workspace that builds with a Core-model library crate and a `nextguide` binary crate (stubs are enough), plus a smoke test that can resolve Guide fixtures under `data/`, so later tickets have a place to land.

**Blocked by:** None (can start immediately)

**Status:** ready-for-agent

- [ ] Cargo workspace builds with library `nextguide-gde` and binary `nextguide`
- [ ] Minimal-dependencies posture is in place (no unnecessary crates)
- [ ] A smoke test (or equivalent) can locate fixtures under `data/`
