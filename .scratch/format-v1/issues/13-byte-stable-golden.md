# 13: Byte-stable canonical golden

**What to build:** At least one owned golden Guide produced by Format v1’s canonical writer Round-trips Byte-stable (`load→save` bytes equal), with a library helper and test so encoder stability is locked for CI.

**Blocked by:** 12 — Save + Structural Round-trip + Preserve-through

**Status:** ready-for-agent

- [ ] An owned golden exists under `data/` (or is generated and stored there) from the canonical writer
- [ ] Byte-stable Round-trip helper passes for that golden
- [ ] Structural Round-trip still passes for that golden
