# 14: `nextguide dump`

**What to build:** The `nextguide dump` command prints a human-readable Core-model tree (titles; body lengths as needed) for a `.gde` path; load failures exit `1`, usage/args errors exit `2`, success exits `0`.

**Blocked by:** 11 — Load v2 Guide into Core model (fail closed)

**Status:** ready-for-agent

- [ ] `nextguide dump <file.gde>` prints Core-model tree information
- [ ] Exit codes: `0` success, `1` load failure, `2` usage/args
- [ ] Smoke test or documented manual check covers `ImportSample.gde`
