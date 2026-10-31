Type: grilling
Status: resolved
Blocked by: 02

## Question

What CLI commands and outputs should Format v1 ship to prove Round-trip against the chosen fixture equality bar?

Name the minimal command set (e.g. inspect/dump, load-save round-trip check, exit codes) and what a successful vs failed proof looks like for humans and CI.

## Answer

Binary name: **`nextguide`**.

Subcommands:

- **`nextguide dump <file.gde>`** — print the Core-model tree (titles; body lengths as needed) for humans.
- **`nextguide check <file.gde>`** — Structural Round-trip (`load→save→load`). Exit `0` on pass, non-zero on fail; failure details on stderr. **`--byte-stable`** selects Byte-stable Round-trip for canonical writer / owned goldens.

No separate `rewrite` command required for Format v1 proof.
