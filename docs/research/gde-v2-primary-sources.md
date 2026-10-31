# Primary sources for The Guide v2 `.gde` format

**Date:** 2026-10-31  
**Ticket:** `.scratch/format-v1/issues/04-primary-sources-inventory.md`  
**Scope:** Inventory of materials that define The Guide **v2** `.gde` on-disk format and Core-model-relevant behaviour (tree structure, node titles, opaque RTF bodies). Third-party descriptions are listed only as secondary pointers.

## Verdict

There is **no separate official format specification document**. The v2 `.gde` format is defined by **implementation**: `libguide`’s load/store code in The Guide 2.0 sources (Apache License 2.0), plus first-party release notes that state the format changed for v2.x. In this repo, `data/ImportSample.gde` is a local v2 fixture (magic `GDE`, little-endian version `2`).

## 1. Official / source-owned materials

### 1.1 Project home and SourceForge

| Item | Location | Role | License |
|------|----------|------|---------|
| Project home | https://theguide.sourceforge.net/ | Product description; points to downloads and release notes | Site content; software Apache 2.0 (see About) |
| About / license statement | https://theguide.sourceforge.net/about.html | States The Guide is released under **Apache 2.0**; author Mahadevan R (`mdevan`) | Apache 2.0 for the software |
| SourceForge project | https://sourceforge.net/projects/theguide/ | Hosts releases; project metadata lists **Apache License V2.0** | Apache License V2.0 |
| Downloads index | https://theguide.sourceforge.net/download.html | Canonical release packages | — |
| Stable 2.0 sources (canonical tarball) | SourceForge Files → `winguide/2.0/` → **`winguide-2.0-src.zip`** (13-Dec-2008, ~1.5 MB) — https://sourceforge.net/projects/theguide/files/winguide/2.0/ | **Primary source tree** for Format v1 codec work | Contained `LICENSE` / `NOTICE` (Apache 2.0) |
| Stable 2.0 binaries | Same folder: `winguide-2.0-win32.zip`, `winguide-2.0-win32.exe` | Reference writer/reader behaviour | Apache 2.0 |

**Note on access (research session):** Direct HTTPS download of `winguide-2.0-src.zip` from SourceForge returned HTTP 403 from this environment. The same 2.0 tree is widely mirrored on GitHub (below). Prefer verifying any mirror against the SourceForge zip when network access allows.

### 1.2 First-party docs that mention the format (not a byte layout)

| Item | Location | What it establishes |
|------|----------|---------------------|
| Release notes v2.0 | https://theguide.sourceforge.net/release.html | “The **`.gde` file format has changed**. The Guide v2.x series can read files of both format, and will write out in the new file format.” |
| v2.0 WIP page | https://theguide.sourceforge.net/20dev.html | Same format-change note during development; lists features that map to on-disk node attributes (icons, colors, remembered expand/selection, etc.) |

These documents confirm a **v1 vs v2 generation** of `.gde` and that writers of the 2.x series emit v2. They do **not** document field layouts.

### 1.3 Source tree layout (The Guide release 2.0)

Inspected via GitHub mirrors whose `README` identifies **“release 2.0”** and whose `LICENSE` is Apache 2.0 copyright **2005-08 Mahadevan R** (same notices as the official product):

- https://github.com/sandeep-datta/winguide (full app + `libguide` + `gdeutil`)
- https://github.com/enteropositivo/winguide (same core; minor portability edits in `guide.c`; also ships `winguide-2.1-setup` extras — treat as a **mirror with possible extras**, not a later official format)
- https://github.com/Cryru/TheGuide (another public copy of the 2.0 README/tree)

**Packages inside the 2.0 tree relevant to `.gde`:**

| Path (in 2.0 sources) | Role for format / Core model |
|-----------------------|------------------------------|
| `libguide/include/libguide/guide.h` | In-memory model: `guide_t` (tree + `_counter` + `sel_node`), `guide_nodedata_t` (**title** `wchar_t*`, **text** as byte string documented as UTF-8 — in practice RTF payload), non-Core attrs (`state`, `icon`, `first_line`, `color`, `bgcolor`, `uid`, `tc_state`) |
| `libguide/src/guide.c` | **Definition of on-disk v2 layout**: `guide_store`, `guide_load` / `guide_load_v2`, `_guide_read_header`, `_guide_read_node_v2`, attribute writers |
| `libguide/include/libguide/tree.h` (+ `tree.c`) | Tree ADT; preorder traversal used when writing nodes |
| `libguide/include/libguide/lut.h` (+ `lut.c`) | UID / fake-pointer remapping on load (not itself a file format) |
| `libguide/include/libguide/config.h` | Build/export macros |
| `gdeutil/gdeutil.c` | First-party CLI: load via `guide_load`, export to XML (shows expected node fields including RTF text) |
| `guide/GuideDoc.cpp` | MFC document I/O: calls `guide_load` / `guide_store` for `.gde` paths (application behaviour around the codec) |
| `LICENSE`, `NOTICE`, `README`, `CHANGES` | License (Apache 2.0), third-party attributions (Silk icons, Argtable), release identity |

**There is no dedicated “format.md” / RFC in the tree.** Comments in `guide.c` are the closest first-party prose for the byte layout.

### 1.4 On-disk v2 layout (as implemented in `guide_store` / `guide_load_v2`)

Endianness: native host integers as written by the historical **32-bit Win32** app (little-endian on x86). Official The Guide 2.0 ships as a **32-bit** application; node/parent “ids” are written with `sizeof(void*)` (4 bytes on that platform).

**File header** (`guide_store` / `_guide_read_header`):

```
'GDE'                          # 3 bytes magic
uint32 format_version          # must be 2
uint32 file_attr_count         # written as 2
# then file_attr_count times:
  uint32 attr_id
  uint32 attr_val_len
  attr_val                     # attr_val_len bytes
```

Known file attribute ids (writer emits both):

| attr_id | Meaning |
|---------|---------|
| 1 | `_counter` (`uint32`) |
| 2 | `sel_node` as fake pointer / id (`uint32` on 32-bit) |

**Each node** (preorder; comments in `_guide_storer_fn`):

```
node_id                        # sizeof(void*) — fake pointer id
parent_node_id                 # sizeof(void*); root parent is null/0
uint32 node_attr_count         # written as 7
# then node_attr_count times: attr_id, attr_val_len, attr_val
uint32 title_len
title_bytes                    # UTF-8 (no NUL included in len)
uint32 text_len
text_bytes                     # node body bytes (RTF); opaque for Core model
```

Known node attribute ids:

| attr_id | Field | Core model? |
|---------|-------|-------------|
| 1 | `state` (e.g. expanded bit) | No |
| 2 | `icon` | No |
| 3 | `first_line` | No |
| 4 | `color` | No |
| 5 | `bgcolor` | No |
| 6 | `uid` | No (identity helper; tree topology uses fake pointers) |
| 7 | `tc_state` | No |

Unknown attributes must be skipped by length (`guide_load_v2` default branch). Format v1 Core model cares about **parent/child topology**, **title**, and **text** bytes; other attrs are policy for a separate ticket.

**v1 detection** (out of Format v1 scope, but present in the same reader): `guide_load` treats files whose bytes at offset 4 match a fixed `dummy` pattern as format 1; v2 requires the 7-byte prefix `GDE\x02\0\0\0`.

### 1.5 Core-model behaviour hooks in first-party code

- **Tree:** `guide_t.tree`; nodes linked via parent fake-ids remapped through a LUT on load (`guide_load_v2`).
- **Titles:** stored UTF-8 length-prefixed; in memory `wchar_t*` (`guide_nodedata_t.title`).
- **Bodies:** `guide_nodedata_t.text` — header comment says UTF-8; sample files contain **`{\rtf1...`** Rich Text. For Format v1, treat as **opaque RTF payload** (do not parse for Round-trip).
- **Dummy root:** new guides create a root titled `"dummy"` with text `"dummy"` (`_guide_get_dummy_data`); real user trees hang under that structure as written by the app.

## 2. In-repo first-party materials (NextGuide)

| Item | Path | Role | License |
|------|------|------|---------|
| Sample fixture | `data/ImportSample.gde` | Binary v2 sample; header begins `47 44 45 02 00 00 00` (`GDE` + version 2); contains UTF-8 titles and RTF bodies (e.g. `Page1`, `Child1`, `{\rtf1\ansi...`) | Repo **MIT** (`LICENSE`); fixture content is user/sample data, not The Guide sources |
| Domain language | `GLOSSARY.md` | Defines Core model / Format v1 / Round-trip terms | MIT |
| Product README | `README.md` | States intent to read/write the same format as The Guide | MIT |
| Greenfield codec ADR | `docs/adr/0001-greenfield-gde-codec.md` | Decided: do not adapt `libguide` as the Format v1 implementation | MIT |

No other `.gde` samples or format docs currently live in this repo.

## 3. Licenses (summary)

| Work | License | Notes |
|------|---------|-------|
| The Guide 2.0 (incl. `libguide`, `gdeutil`, MFC app sources) | **Apache License 2.0** | Confirmed by SourceForge project page, About page, and `LICENSE` / copyright headers in the 2.0 tree (`Copyright 2005-08 Mahadevan R`). `NOTICE` attributes Silk icons and Argtable. |
| NextGuide repository | **MIT** | `LICENSE` — Copyright (c) 2026 Dirk Dörr |
| Greenfield Format v1 codec | MIT (repo) | May be **written from** Apache-2.0 primary sources; shipping adapted `libguide` is out of scope by project decision |

## 4. Secondary pointers (not definition of truth)

Use only as navigation aids or cross-checks; **do not** treat as normative for Format v1:

| Item | Location | Why secondary |
|------|----------|---------------|
| Non-Windows fork of `libguide` / `gdeutil` | https://github.com/onderweg/the-guide-non-windows | Modified (POSIX mmap/unicode, **v1 support removed**, forced 32-bit id sizes). Useful for reading on macOS, not the original writer. |
| Kaitai descriptions | `extra/kaitai/gde32.kty`, `gde64.kty` in that fork | Third-party reverse description; aligns with `guide.c` but is derivative. |
| Blog: “Reviving The Guide” | https://blog.onderweg.eu/reviving-the-guide/ | Narrative of porting/`sizeof(void*)` pitfalls |
| Other GitHub copies | `enteropositivo/winguide`, `Cryru/TheGuide`, etc. | Convenience mirrors; authenticity should be checked against `winguide-2.0-src.zip` |

## 5. Implications for later codec work

1. **Treat `libguide/src/guide.c` (+ `guide.h`) from The Guide 2.0 as the normative byte-level reference** for v2 `.gde`.
2. Prefer the **SourceForge `winguide-2.0-src.zip`** as the pinned primary artifact; keep a verified local copy outside product history if needed.
3. Use `data/ImportSample.gde` as the first Round-trip fixture; expand fixtures from files written by official 2.0 binaries when available.
4. Keep **Kaitai / onderweg** as optional cross-checks only; Format v1 remains a **greenfield** implementation (see ADR `0001`).
5. Plan for **32-bit fake pointer** widths when matching historical files; document any intentional fixed-width choice in the greenfield codec.

## 6. Sources consulted

- https://theguide.sourceforge.net/ (home, download, release notes, 20dev, about)
- https://sourceforge.net/projects/theguide/ and Files `winguide/2.0/`
- Mirrored 2.0 trees: `sandeep-datta/winguide`, `enteropositivo/winguide` (inspected `libguide`, `gdeutil`, `guide/GuideDoc.cpp`, `LICENSE`, `NOTICE`, `README`, `CHANGES`)
- Local: `data/ImportSample.gde`, `LICENSE`, `GLOSSARY.md`, `docs/adr/0001-greenfield-gde-codec.md`
- Secondary: https://github.com/onderweg/the-guide-non-windows (README + `extra/kaitai/gde32.kty`)
