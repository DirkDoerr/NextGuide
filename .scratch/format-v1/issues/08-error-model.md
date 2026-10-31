Type: grilling
Status: resolved

## Question

What error model should Format v1 use for corrupt, truncated, non-v2, or otherwise unreadable `.gde` inputs?

Decide how `load` / `nextguide check` / `nextguide dump` fail (error types vs strings, exit codes), what is considered unsupported vs corrupt, and whether partial trees are ever returned.

## Answer

**Fail closed + typed errors:**

- `load` returns `Result`; **never** a partial `Guide`.
- Error categories (Rust enum + `Display`; no dedicated error crate unless forced): **I/O**, **unsupported** (e.g. non-v2 generation), **corrupt/truncated/invalid** structure.
- CLI: exit `0` success; `1` load/check failure; `2` usage/args. `dump` and `check` fail entirely when load fails.
