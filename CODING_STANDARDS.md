# Coding standards

Rules for `/code-review` (Standards axis). Judgement calls only — mechanical checks belong in CI or tooling.

## Core model vs Preserve-through

Public Core-model types (`Guide`, `Node`, and their public methods) must not surface Preserve-through bags via `Debug`, `Display`, or serialization. Opaque load/save state stays `pub(crate)` and out of human-facing debug output.
