# 14: `nextguide dump`

**What to build:** The `nextguide dump` command prints a human-readable Core-model tree (titles; body lengths as needed) for a `.gde` path; load failures exit `1`, usage/args errors exit `2`, success exits `0`.

**Blocked by:** 11 — Load v2 Guide into Core model (fail closed)

**Status:** resolved

- [x] `nextguide dump <file.gde>` prints Core-model tree information
- [x] Exit codes: `0` success, `1` load failure, `2` usage/args
- [x] Smoke test or documented manual check covers `ImportSample.gde`

## Answer

`nextguide dump` prints indented titles with RTF lengths; exit 0/1/2; CLI smoke covers ImportSample.
