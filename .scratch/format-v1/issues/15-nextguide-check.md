# 15: `nextguide check` (+ `--byte-stable`)

**What to build:** The `nextguide check` command proves Structural Round-trip via the library helper (exit `0`/`1`/`2`); `--byte-stable` selects Byte-stable Round-trip for owned goldens. CLI does not reimplement codec logic.

**Blocked by:** 12 — Save + Structural Round-trip + Preserve-through; 13 — Byte-stable canonical golden

**Status:** ready-for-agent

- [ ] `nextguide check <file.gde>` runs Structural Round-trip using the library helper
- [ ] `nextguide check --byte-stable <file.gde>` runs Byte-stable Round-trip using the library helper
- [ ] Exit codes: `0` pass, `1` failure, `2` usage/args; failure details on stderr
- [ ] Smoke tests cover structural check on `ImportSample.gde` and byte-stable on the owned golden
