Type: grilling
Status: resolved

## Question

When a The Guide v2 `.gde` contains attributes outside the Core model (icons, colors, checkboxes, hyperlinks, UI state, etc.), what must Format v1 do on load and on save?

Decide among preserve-through (opaque passthrough so Round-trip does not strip them), strip on write, reject/fail load, or another precise policy — and how that interacts with the fixture equality bar.

## Answer

**Preserve-through:** load keeps non-Core (and unknown) file- and node-level attributes as opaque data; save writes them back. The Core-model API does not have to expose them.

This must not weaken the hybrid equality bar: Structural Round-trip remains the CI gate for all fixtures; Byte-stable Round-trip applies only to the canonical writer / owned goldens. The Guide–written fixtures are not required to Round-trip byte-identically through Format v1.
