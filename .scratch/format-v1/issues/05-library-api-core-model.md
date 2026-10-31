Type: grilling
Status: resolved
Blocked by: 01

## Question

What public library API should Format v1 expose for loading and saving a Guide under the Core model?

Cover the in-memory shape (Guide / Node / opaque RTF), entry points (load path/bytes, save path/bytes), and what is deliberately not exposed in Format v1 (no editor operations, no RTF mutate API unless decided otherwise).

## Answer

**Codec + Core read model** (Rust library):

- Entry points: `load` / `save` from path and from bytes.
- In-memory: `Guide` → ordered tree of `Node`s; each `Node` has `title: String`, opaque `rtf_body` bytes, ordered `children`.
- Walk/inspect only — no public tree-edit API, no RTF parse/mutate in Format v1.
- Preserve-through attribute bags stay private to the codec (not on the Core-model API).
- **Dummy root:** the public tree includes The Guide’s on-disk dummy root; callers may skip it. Hiding it is deferred to UI concerns.
