Type: grilling
Status: resolved
Blocked by: 04, 05

## Question

What **testing / proof bar** should the Editor v1 spec require so CI (and humans) can trust Core-model editing without running The Guide GUI in every pipeline?

Cover library/mutation tests, Round-trip after edits, and how thin UI smoke tests should be relative to Format v1’s existing gates.

## Answer

**Libraries prove editing honesty; UI stays thin; The Guide stays a human gate.**

- Keep Format v1 CI gates (`nextguide-gde` Structural/Byte-stable; `nextguide-cli` smokes) — no regressions.
- **Primary new seam:** `nextguide-gde` mutation tests (edit → Structural Round-trip / fail-closed) and `nextguide-rtf` tests (subset convert; untouched-bytes; soft-fail unparseable). External behaviour only — not private Preserve-through bag layout.
- **Secondary:** thin `nextguide` UI smoke (start / open fixture headless if practical); no full widget matrix in CI.
- **Optional human:** open Round-tripped Guides in The Guide after meaningful releases — never CI.
