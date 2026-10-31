# 17: Edit titles and bodies + dirty/save

**What to build:** Users can edit the selected Node’s title in the tree and its body with the pragmatic rich-text subset; the Guide becomes dirty; Save writes Core-model mutations (subset RTF rewrite only for touched bodies; Preserve-through retained on those Nodes).

**Blocked by:** 16 — Open Guide — two-pane tree + body + save unchanged

**Status:** ready-for-agent

- [ ] Title and body edits update the in-memory Guide via the mutation API / `nextguide-rtf`
- [ ] Dirty indicator appears; Save clears dirty and persists `.gde`
- [ ] Untouched sibling bodies keep original RTF bytes; edited bodies emit subset RTF
- [ ] Library/UI path does not regress Structural Round-trip for the edited Guide’s Core model
