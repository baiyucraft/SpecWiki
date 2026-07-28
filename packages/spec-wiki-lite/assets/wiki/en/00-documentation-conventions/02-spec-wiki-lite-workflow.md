---
title: SpecWiki Lite Workflow
description: Change stages, Skills, and archive boundaries for Wiki work.
updated: 2026-07-29
owner: spec-wiki-lite
---

# SpecWiki Lite Workflow

> SpecWiki Lite managed baseline

```text
explore -> propose -> design -> plan -> apply -> review -> archive
```

- `.spec/changes/<change-id>/` stores active change artifacts.
- `.spec/archive/YYYY-MM-DD-<change-id>/` stores completed evidence.
- `.agents/skills/wiki-*` contains the Codex stage entry points.
- `spec-wiki-lite validate --strict` is the deterministic advancement and archive gate.
- `.wiki` contains only knowledge that remains valuable after the change is complete.
