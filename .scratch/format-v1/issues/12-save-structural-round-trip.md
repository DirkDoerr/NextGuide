# 12: Save + Structural Round-trip + Preserve-through

**What to build:** Callers can `save` a loaded Guide to a path or bytes; Structural Round-trip (`load→save→load`) preserves Core-model topology, titles, and opaque RTF; non-Core attributes Preserve-through on save without appearing on the Core-model API. Library helper + tests prove Structural Round-trip on real fixtures.

**Blocked by:** 11 — Load v2 Guide into Core model (fail closed)

**Status:** ready-for-agent

- [ ] `save` to path and to bytes works for a loaded Guide
- [ ] Structural Round-trip helper passes for `data/ImportSample.gde` (and similar)
- [ ] Preserve-through keeps non-Core/unknown attrs across save without Core API exposure
- [ ] Greenfield v2 writer respects agreed layout assumptions (little-endian, 32-bit fake ids)
