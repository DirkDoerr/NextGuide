# 14: `nextguide-rtf` pragmatic subset library

**What to build:** A workspace library **`nextguide-rtf`** converts between opaque RTF bytes and a toolkit-agnostic edit model for the pragmatic subset (bold/italic/underline/paragraphs/simple lists), with helpers for untouched-byte policy, soft-fail unparseable input, and minimal empty RTF — testable without GTK on the public API.

**Blocked by:** None (can start immediately)

**Status:** ready-for-agent

- [ ] Crate exists in the workspace; public API has no GTK types
- [ ] Subset convert round-trips are covered by library tests
- [ ] Untouched-bytes / soft-fail / empty-RTF behaviours match the Editor v1 spec
- [ ] No regression to Format v1 codec crates beyond adding the new member
