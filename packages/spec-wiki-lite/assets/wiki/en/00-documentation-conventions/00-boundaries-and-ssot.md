---
title: Boundaries and SSOT
description: Boundaries between durable Wiki knowledge, change artifacts, temporary material, and code facts.
updated: 2026-07-29
owner: spec-wiki-lite
---

# Boundaries and SSOT

> SpecWiki Lite managed baseline
>
> Normal updates preserve local edits. Only `update --force` restores this packaged baseline.

| Kind | Location | Responsibility |
| --- | --- | --- |
| Durable knowledge | `.wiki/**/*.md` | Stable explanations, navigation, development rules, and public contracts |
| Change process | `.spec/changes/**` | Proposals, designs, tests, tasks, and reports |
| Historical evidence | `.spec/archive/**` | Read-only evidence for completed changes |
| Code facts | Source, configuration, and tests | Final authority for behavior, fields, defaults, and entry points |
| Temporary material | `.tmp/` or the project convention | Debug output and one-time research |

- Maintain each fact in one authoritative place and link to it elsewhere.
- Do not copy full change artifacts into the Wiki.
- Do not promote unconfirmed assumptions or one-time evidence to durable facts.
- Update local and parent `INDEX.md` pages after adding or moving pages.
