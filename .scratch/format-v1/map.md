# Format v1 wayfinder map

Type: wayfinder:map

## Destination

A buildable product spec for **Format v1**: a greenfield library plus CLI that Round-trips The Guide **v2** `.gde` files for the **Core model** (tree, titles, opaque RTF bodies), with fixture-based proof. No editor UI.

## Notes

- Domain language: `GLOSSARY.md`
- Tracker: local markdown under `.scratch/format-v1/` (see `docs/agents/issue-tracker.md`)
- Codec posture: **greenfield** from primary sources — do not ship Format v1 by adapting `libguide` / forks as the implementation
- Implementation language: **Rust**, minimal-dependencies rule (see Decisions so far + ADR-0002)
- Skills: `grilling` + `domain-modeling` for grilling tickets; `research` for research tickets; `prototype` only if a ticket is labelled that way

## Decisions so far

- [Inventory primary sources for The Guide v2 .gde format](issues/04-primary-sources-inventory.md): No separate format spec — v2 `.gde` is defined by Apache-2.0 The Guide 2.0 `libguide` (`guide.c`/`guide.h`); inventory at `docs/research/gde-v2-primary-sources.md`
- [Choose Format v1 implementation language](issues/01-implementation-language.md): Rust with a hard minimal-dependencies rule (desktop binaries now; same codec linkable later for optional mobile)
- [Define Round-trip fixture equality](issues/02-round-trip-fixture-equality.md): Hybrid — Core-model structural Round-trip as CI gate; byte-stable Round-trip for canonical writer / owned goldens
- [Non-Core attributes policy](issues/03-non-core-attributes-policy.md): Preserve-through — opaque load/save of non-Core and unknown attrs; Core API need not expose them
- [Library API for Core model](issues/05-library-api-core-model.md): Codec + Core read model (load/save path+bytes; Guide/Node title+opaque RTF+children; no edit/RTF API; dummy root visible; Preserve-through private)
- [CLI Round-trip surface](issues/06-cli-round-trip-surface.md): `nextguide dump` + `nextguide check` (Structural; `--byte-stable` for Byte-stable); exit 0/non-zero
- [Rust package layout](issues/07-rust-package-layout.md): Workspace — `crates/nextguide-gde` (lib) + `crates/nextguide` (bin); fixtures stay in `data/`
- [Error model](issues/08-error-model.md): Fail closed; typed I/O / unsupported / corrupt; no partial Guide; CLI exits 0/1/2
- [Kaitai as reference](issues/09-kaitai-as-reference.md): Read-only cross-check allowed; no codegen/vendoring as codec; not normative vs guide.c
- [Scaffold Format v1 Cargo workspace](issues/10-scaffold-cargo-workspace.md): Workspace `crates/nextguide-gde` + `crates/nextguide`; fixtures via `data/`
- [Load v2 Guide into Core model](issues/11-load-core-model.md): `load_path`/`load_bytes` → Core tree; fail-closed typed errors
- [Save + Structural Round-trip](issues/12-save-structural-round-trip.md): `save_*` + `check_structural`; Preserve-through private; ImportSample passes
- [Byte-stable canonical golden](issues/13-byte-stable-golden.md): `data/canonical-golden.gde` + `check_byte_stable`
- [`nextguide dump`](issues/14-nextguide-dump.md): Core tree dump; exits 0/1/2
- [`nextguide check`](issues/15-nextguide-check.md): Structural / `--byte-stable` via library helpers

## Not yet specified

<!-- empty: frontier cleared for this destination -->

## Out of scope

- Editor UI and desktop / mobile apps (README platforms beyond the format layer)
- How Format v1 later feeds a desktop/mobile editor (integration shape deferred past this destination)
- The Guide **v1** `.gde` generation (read or write)
- Full The Guide 2.0 attribute parity beyond the Core model (icons, colors, checkboxes, hyperlinks, remembered expansion, etc.)
- Adapting `libguide` or its forks as the Format v1 implementation
