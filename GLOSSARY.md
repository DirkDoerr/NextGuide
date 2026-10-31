# NextGuide

Cross-platform successor ideas around The Guide's hierarchical notes and `.gde` files. This glossary covers product language only.

## Language

**The Guide**:
The original Windows two-pane rich-text outliner (SourceForge / theguide.sourceforge.net) whose behaviour and file format NextGuide targets.
_Avoid_: Guide app, WinGuide (unless referring to a specific package name)

**Guide**:
A NextGuide / The Guide document: a tree of nodes, each with associated rich text, stored as a `.gde` file.
_Avoid_: notebook, document (when the Guide sense is meant), file (prefer Guide when talking about the logical document)

**Node**:
One entry in a Guide's tree; it has a title (and other attributes) and may have rich-text body content.
_Avoid_: page, topic, item (when Node is meant)

**Round-trip**:
A Guide written by NextGuide opens correctly in The Guide 2.0, and a Guide written by The Guide 2.0 opens correctly in NextGuide, for the supported feature set.
_Avoid_: import-only, compatibility (when the two-way bar is meant)

**Core model**:
The v1-supported Guide surface: tree structure, node titles, and each node's RTF body. Other The Guide 2.0 attributes (icons, colors, checkboxes, hyperlinks, remembered expansion) are outside the Core model.
_Avoid_: full Guide, minimal Guide (prefer Core model)

**Format v1**:
The first shippable NextGuide product slice: a library that loads and saves Guides as `.gde` for the Core model, plus a CLI that proves Round-trip. No editor UI. Codec is greenfield (written from primary sources), targeting The Guide **v2** `.gde` only. RTF bodies are an opaque payload for Round-trip, not a parsed edit model. The Round-trip CLI binary is **`nextguide-cli`** (renamed so **`nextguide`** can mean the editor).
_Avoid_: MVP app, NextGuide app (when Format v1 is meant)

**Editor v1**:
The first NextGuide editor UI product slice: a cross-platform desktop app (Windows, macOS, Linux) that opens, navigates, and edits Guides for the Core model with The Guide’s functionality (not a Windows UI clone), Round-trip honesty via Format v1, and a pragmatic rich-text subset for Node bodies. Mobile is out of this slice. UI stack is **GTK4 + Relm4** (no web shell); the app binary is **`nextguide`**. Body editing uses **`nextguide-rtf`** with GTK tags; untouched RTF bytes stay opaque until edited.
_Avoid_: NextGuide app (when Editor v1 is meant), MVP UI, The Guide clone

**Structural Round-trip**:
A load→save→load cycle that preserves the Core model (tree topology, titles, opaque RTF bodies), without requiring identical `.gde` bytes.
_Avoid_: byte-identical (when only the Core model must match)

**Byte-stable Round-trip**:
A load→save cycle whose output `.gde` bytes equal the input under Format v1’s canonical writer (used for owned golden fixtures).
_Avoid_: bit-exact (prefer Byte-stable Round-trip)

**Preserve-through**:
Format v1 codec behaviour that keeps non-Core and unknown `.gde` attributes opaque across load and save so The Guide does not lose them, without exposing them on the Core-model API.
_Avoid_: passthrough, strip-on-write (when Preserve-through is meant)

**Dummy root**:
The synthetic root node The Guide stores in a v2 `.gde` (typically titled `"dummy"`), under which user-visible nodes hang. Format v1’s Core-model API exposes it as part of the on-disk tree.
_Avoid_: hidden root, invisible root

**Node path**:
The address of a Node in the Core-model tree: a sequence of 0-based child indices from the Dummy root (`[]` is the Dummy root itself).
_Avoid_: node id, pointer, handle (when the path address is meant)
