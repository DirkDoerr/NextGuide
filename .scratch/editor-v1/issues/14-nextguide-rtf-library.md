# 14: `nextguide-rtf` pragmatic subset library

**What to build:** A workspace library **`nextguide-rtf`** converts between opaque RTF bytes and a toolkit-agnostic edit model for the pragmatic subset (bold/italic/underline/paragraphs/simple lists), with helpers for untouched-byte policy, soft-fail unparseable input, and minimal empty RTF — testable without GTK on the public API.

**Blocked by:** None (can start immediately)

**Status:** resolved

- [x] Crate exists in the workspace; public API has no GTK types
- [x] Subset convert round-trips are covered by library tests
- [x] Untouched-bytes / soft-fail / empty-RTF behaviours match the Editor v1 spec
- [x] No regression to Format v1 codec crates beyond adding the new member

## Answer

Added workspace crate `crates/nextguide-rtf` (also a `default-member`). Public API: `Document`/`Block`/`Run`/`SpanStyle`, `load` → `LoadBody::{Editable, Unparseable}`, `write`, `empty_rtf`, `bytes_for_save` (untouched vs rewrite). Pure Rust, zero deps, no GTK types. Tests cover subset round-trips, Guide-style samples, soft-fail, empty RTF, and untouched-bytes policy.
