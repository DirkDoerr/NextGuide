Type: grilling
Status: resolved
Blocked by: 01

## Question

How should the Format v1 Rust workspace be laid out?

Decide crate boundaries (e.g. library crate for the Core-model codec vs binary crate for the Round-trip CLI), repository paths, and how that layout supports shipping one executable per OS/arch while keeping the library reusable for a later UI — under the minimal-dependencies rule.

## Answer

**Cargo workspace with two crates:**

- `crates/nextguide-gde` — library (Core-model codec; kept separable for later `cdylib`/`staticlib` / UI linking)
- `crates/nextguide` — binary crate producing `nextguide`, depending on `nextguide-gde` (+ minimal CLI deps only)

**Fixtures:** `data/` remains the canonical human-facing sample home (e.g. `ImportSample.gde`). Crate tests reference `data/` — do not duplicate goldens under the crate tree.
