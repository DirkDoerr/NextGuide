Type: research
Status: resolved

## Question

What primary sources define The Guide v2 `.gde` on-disk format and Core-model-relevant behaviour, and where do they live?

Inventory official/source-owned materials (The Guide source trees, headers, writers/readers, sample files in this repo, any first-party docs). Note licenses. Third-party descriptions may be listed only as secondary pointers, not as the definition of truth. Output a cited research note in-repo for later codec work.

## Answer

No separate official format spec exists: v2 `.gde` is defined by The Guide 2.0 `libguide` load/store (`guide.c` / `guide.h`), shipped as Apache-2.0 `winguide-2.0-src.zip` on SourceForge, with first-party release notes confirming the format change. Local fixture: `data/ImportSample.gde`. Full inventory: [docs/research/gde-v2-primary-sources.md](../../../docs/research/gde-v2-primary-sources.md).
