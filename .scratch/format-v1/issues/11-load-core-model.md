# 11: Load v2 Guide into Core model (fail closed)

**What to build:** Callers can `load` a The Guide v2 `.gde` from a path or bytes into a readable Core-model tree (Dummy root visible; Node titles, opaque RTF bodies, ordered children), and bad inputs fail closed with typed errors (I/O, unsupported, corrupt) — never a partial Guide. Verified with `ImportSample.gde` and deliberate bad fixtures.

**Blocked by:** 10 — Scaffold Format v1 Cargo workspace

**Status:** resolved

- [x] `load` from path and from bytes returns a Core-model `Guide`
- [x] Dummy root, titles, opaque RTF bodies, and child order are observable
- [x] Non-v2 / corrupt / truncated inputs return typed errors; no partial `Guide`
- [x] Library tests cover `data/ImportSample.gde` and fail-closed cases

## Answer

`load_path` / `load_bytes` parse v2 little-endian 32-bit layout into `Guide`/`Node` (Dummy root visible). Typed `Error::{Io, Unsupported, Corrupt}`; v1 marker and non-v2 versions are Unsupported.
