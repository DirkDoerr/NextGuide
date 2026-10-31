# 13: Byte-stable canonical golden

**What to build:** At least one owned golden Guide produced by Format v1’s canonical writer Round-trips Byte-stable (`load→save` bytes equal), with a library helper and test so encoder stability is locked for CI.

**Blocked by:** 12 — Save + Structural Round-trip + Preserve-through

**Status:** resolved

- [x] An owned golden exists under `data/` (or is generated and stored there) from the canonical writer
- [x] Byte-stable Round-trip helper passes for that golden
- [x] Structural Round-trip still passes for that golden

## Answer

Owned golden at `data/canonical-golden.gde` from the canonical writer; `check_byte_stable` + `check_structural` both pass.
