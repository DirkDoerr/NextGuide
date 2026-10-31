Status: ready-for-agent

# Format v1 — Guide `.gde` library and CLI

## Problem Statement

I have Guides written by The Guide (and will create new ones) stored as `.gde` files. I need a modern, installable tool and library that can read and write The Guide **v2** format for the Core model so I can Round-trip real Guides without an editor UI yet — and prove that Round-trip in CI — on Windows, macOS, and Linux, without depending on The Guide’s Windows app or adapting `libguide` as our implementation.

## Solution

Ship **Format v1**: a greenfield Rust library (`nextguide-gde`) that loads and saves Guides as v2 `.gde` for the Core model (tree, titles, opaque RTF bodies), with Preserve-through of non-Core attributes, plus a thin `nextguide` CLI (`dump` / `check`) that proves Structural Round-trip (and Byte-stable Round-trip for our goldens). Easy-to-install single binaries; minimal dependencies; Dummy root visible in the API; fail closed on bad input.

## User Stories

1. As a NextGuide developer, I want a library that loads a The Guide v2 `.gde` into a Core-model tree, so that I can inspect Guides without The Guide’s Windows UI.
2. As a NextGuide developer, I want a library that saves a loaded Guide back to `.gde`, so that Round-trip is possible in code and tests.
3. As a NextGuide developer, I want `load` and `save` from filesystem paths, so that tools and tests can use normal files.
4. As a NextGuide developer, I want `load` and `save` from byte buffers, so that I can test without touching the filesystem and embed the codec later.
5. As a NextGuide developer, I want each Node to expose a title string, so that I can see the tree’s labels.
6. As a NextGuide developer, I want each Node’s RTF body as opaque bytes, so that Round-trip does not depend on parsing rich text.
7. As a NextGuide developer, I want ordered children on each Node, so that tree topology matches the Guide.
8. As a NextGuide developer, I want the Dummy root included in the public tree, so that the API matches on-disk reality.
9. As a NextGuide developer, I want walk/inspect access only (no public tree-edit API) in Format v1, so that the library stays a codec, not an editor.
10. As a NextGuide developer, I want no public RTF parse or mutate API in Format v1, so that we do not pretend to own a rich-text model yet.
11. As a Guide owner, I want non-Core attributes (icons, colors, UI state, etc.) Preserve-through load/save, so that opening the file again in The Guide does not strip them.
12. As a NextGuide developer, I want Preserve-through data kept off the Core-model API, so that the public surface stays small.
13. As a Guide owner, I want unknown attribute ids skipped by length on load and preserved when possible, so that newer or odd files do not hard-fail solely for extra attrs.
14. As a NextGuide developer, I want Structural Round-trip as the CI gate (`load→save→load` preserves topology, titles, and RTF bytes), so that format work is proven without requiring byte-identical The Guide files.
15. As a NextGuide developer, I want Byte-stable Round-trip for Guides written by our canonical writer / owned goldens, so that our encoder stays locked down.
16. As a CI job, I want `nextguide check` to exit 0 on Structural Round-trip success and non-zero on failure, so that pipelines can gate merges.
17. As a CI job, I want `nextguide check --byte-stable` for golden fixtures, so that byte stability is opt-in and explicit.
18. As a developer debugging a Guide, I want `nextguide dump` to print the Core-model tree (titles and useful body lengths), so that I can see structure quickly.
19. As a CLI user, I want the binary named `nextguide`, so that the product name matches the tool I install.
20. As a CLI user, I want exit code 0 for success, 1 for load/check failure, and 2 for usage/args errors, so that scripts can distinguish failure kinds.
21. As a CLI user, I want `dump` and `check` to fail entirely when load fails, so that I never act on a partial Guide.
22. As a library caller, I want typed errors distinguishing I/O, unsupported generation (e.g. non-v2), and corrupt/truncated/invalid structure, so that I can handle cases differently.
23. As a library caller, I want `load` never to return a partial Guide, so that fail-closed behaviour is reliable.
24. As a Guide owner, I want Format v1 to reject The Guide v1 `.gde` as unsupported (not silently mis-parse), so that I get a clear error.
25. As a Guide owner, I want truncated or garbage files to fail as corrupt, so that I am not given a fake tree.
26. As a NextGuide developer, I want the codec written greenfield from The Guide 2.0 primary sources (`guide.c` / `guide.h`), so that we own the implementation (ADR-0001).
27. As a NextGuide developer, I want Kaitai and similar descriptions usable only as non-normative cross-checks, so that we do not codegen or vendor them as the codec.
28. As a NextGuide developer, I want a Cargo workspace with `nextguide-gde` (library) and `nextguide` (binary), so that the CLI stays thin and the library stays reusable.
29. As a future mobile/desktop app developer, I want the Core-model codec in a separable library crate, so that it can later link in-process (ADR-0002).
30. As a packager, I want to ship one executable per OS/arch with minimal dependencies, so that install is “drop the binary.”
31. As a NextGuide developer, I want a hard minimal-dependencies rule (prefer std for binary I/O; add crates only when clearly needed), so that the dependency graph stays small.
32. As a NextGuide developer, I want sample Guides under `data/` (including `ImportSample.gde`) as canonical fixtures, so that humans and tests share one sample home.
33. As a NextGuide developer, I want library tests to reference `data/` rather than duplicating goldens in the crate tree, so that fixtures stay single-sourced.
34. As a NextGuide developer, I want Structural Round-trip tests against `ImportSample.gde`, so that real The Guide–shaped input is covered early.
35. As a NextGuide developer, I want at least one owned golden that Byte-stable Round-trips under our writer, so that the encoder’s stability is tested.
36. As a NextGuide developer, I want little-endian v2 layout with 32-bit fake pointer widths matching historical The Guide 2.0 files, so that Round-trip with existing Guides works.
37. As a NextGuide developer, I want UTF-8 length-prefixed titles handled as The Guide stores them, so that titles Round-trip correctly.
38. As a documentation reader, I want Format v1 documented as library + CLI with no editor UI, so that scope is obvious.
39. As a product owner, I want editor UI and desktop/mobile apps out of this spec, so that Format v1 can finish without waiting on UX.
40. As a product owner, I want full The Guide 2.0 attribute *API* parity out of scope while still Preserve-through on disk, so that Core model stays small.
41. As a license-conscious maintainer, I want our shipped codec under the repo’s MIT license, written from Apache-2.0 primary sources without shipping adapted `libguide`.
42. As an agent implementer, I want this spec marked ready-for-agent with decisions already settled on the wayfinder map, so that I can implement without re-litigating product calls.
43. As a NextGuide developer, I want `check` to reuse the same Round-trip helpers the library tests use, so that CLI and CI do not diverge.
44. As a NextGuide developer, I want clear stderr messages on `check`/`dump` failure, so that humans can diagnose without a debugger.
45. As a NextGuide developer, I want unsupported vs corrupt distinguished in errors, so that “wrong generation” is not confused with “truncated file.”
46. As a future UI developer, I want the public Dummy root left visible, so that UI can decide how to hide it later without the codec lying.
47. As a NextGuide developer, I want preorder-compatible tree reconstruction from parent/child relationships in the file, so that topology matches The Guide.
48. As a CI maintainer, I want Round-trip proof without running The Guide GUI in CI, so that pipelines stay portable.
49. As a NextGuide developer, I want optional manual verification in The Guide called out as a human gate, not a CI dependency.
50. As a NextGuide developer, I want the research inventory of primary sources respected as the normative byte-level reference, so that implementation debates resolve against `guide.c`.

## Implementation Decisions

- Build a Cargo workspace with two crates: library `nextguide-gde` (Core-model codec) and binary `nextguide` (CLI depending on the library).
- Language: Rust; hard minimal-dependencies rule (ADR-0002). Prefer the standard library for binary I/O; add third-party crates only when the cost of not doing so is clear.
- Greenfield codec from The Guide 2.0 primary sources; do not adapt `libguide` or forks as the implementation (ADR-0001). Normative reference: `guide.c` / `guide.h` (see research inventory). Kaitai etc. may be read for cross-check only — no codegen, no vendoring as codec, not normative.
- Target on-disk generation: The Guide **v2** `.gde` only. Non-v2 is **unsupported**.
- Historical layout assumptions for compatibility: little-endian; 32-bit fake pointer / id widths as in official 32-bit The Guide 2.0 writers.
- Public library API: `load`/`save` for path and bytes; in-memory `Guide` with ordered `Node` tree; each `Node` has `title`, opaque `rtf_body` bytes, ordered `children`; walk/inspect only. No public tree-edit API; no RTF interpret/mutate API.
- Dummy root is part of the public tree.
- Preserve-through: non-Core and unknown file/node attributes kept opaquely inside the codec across load/save; not exposed on the Core-model API.
- Hybrid Round-trip equality: Structural Round-trip is the default proof; Byte-stable Round-trip for canonical writer output and owned goldens.
- Expose library helpers for Structural and Byte-stable Round-trip checks so the CLI and tests share one implementation.
- Typed error enum with `Display`: at least I/O, unsupported, corrupt/truncated/invalid. `load` returns `Result` and never a partial `Guide`.
- CLI binary name `nextguide`; subcommands `dump` and `check`; `check --byte-stable` selects Byte-stable mode.
- CLI exit codes: `0` success, `1` load/check failure, `2` usage/args.
- Fixtures live under the repo’s `data/` tree; tests reference them rather than duplicating binaries inside crates.
- Ship installable executables per OS/arch; keep the library separable for later in-process linking (desktop/mobile UI out of this spec).

## Testing Decisions

- Good tests assert **external behaviour** only: load/save results, Core-model observables (topology, titles, opaque RTF bytes), Round-trip helpers’ pass/fail, error categories, and CLI exit codes/stderr — not private parser internals or Preserve-through bag representation.
- **Primary seam:** `nextguide-gde` public API (load/save, Core-model read surface, Structural/Byte-stable helpers, typed errors).
- **Secondary thin seam:** `nextguide` CLI smoke tests (`dump`/`check` exit codes; `check` delegates to library helpers). Do not duplicate full codec matrices at the CLI layer.
- Prior art: none in-repo yet (greenfield). Establish library fixture tests first; CLI smokes second.
- Required coverage includes: Structural Round-trip on `data/ImportSample.gde`; Byte-stable Round-trip on at least one owned golden from the canonical writer; unsupported non-v2 / corrupt inputs fail closed with the right error category; CLI `check` aligns with library Structural results.

## Out of Scope

- Editor UI; desktop and mobile applications; how Format v1 later embeds into those apps beyond keeping a separable library.
- The Guide **v1** `.gde` read or write.
- Full The Guide 2.0 attribute *API* parity (icons, colors, checkboxes, hyperlinks, remembered expansion, etc.) beyond Preserve-through on disk.
- Adapting or shipping `libguide` / forks as the Format v1 implementation.
- Kaitai codegen or vendoring format descriptions as the codec.
- Requiring The Guide GUI inside CI.
- RTF editing, rendering, or semantic rich-text models.
- Public tree mutation APIs in Format v1.

## Further Notes

- Wayfinder map and resolved decision tickets live alongside this spec under `.scratch/format-v1/`; domain terms are in `GLOSSARY.md`.
- Primary-source inventory: `docs/research/gde-v2-primary-sources.md` (and the throwaway research branch archive if needed). Prefer verifying mirrors against SourceForge `winguide-2.0-src.zip` when network allows.
- Manual open-in-The-Guide checks remain a useful human gate after Byte-stable/Structural automation, not a substitute for it.
- After this spec: `/to-tickets` to split tracer-bullet implementation work with blocking edges.
