---
title: Wiki Bootstrap Task
description: Instructs Codex to build the project Wiki from verified repository facts.
updated: 2026-09-15
owner: project
---

<!-- spec-wiki-lite:bootstrap-pending -->

# Wiki Bootstrap Task

This is a one-time bootstrap task, not the final Wiki home page. Codex should inspect the repository as needed, complete the sections, and replace this page with a project-specific home page without the bootstrap marker.

## Existing Sections

- [Documentation conventions](./00-documentation-conventions/INDEX.md)
- [Quick start](./01-quick-start/INDEX.md)
- [Development guide](./02-development-guide/INDEX.md)
- [Module guide](./03-module-guide/INDEX.md)
- [Public interfaces](./04-public-interfaces/INDEX.md)

## Bootstrap Steps

1. Read the official AOCI Guide and complete Overview first. When Code Cognition is not aligned, follow upstream Maintain/Recovery before continuing. AOCI supplies durable responsibilities, strong relations, and constraints.
2. Use CodeGraph for current source entry points, symbols, call paths, and impact, then verify with README, manifests, build/test configuration, and targeted source. Keep AOCI and CodeGraph evidence separate.
3. Decide whether the evidence establishes project purpose, stack, and run path. Ask only for blocking facts and avoid indiscriminate full scans.
4. Replace placeholders with cross-verified project facts. Explain why a section is not applicable instead of inventing content.
5. Add only useful quick-start, module, CLI, API, configuration, deployment, or troubleshooting pages. Keep an `INDEX.md` in every Markdown directory.
6. Replace this page with the final Wiki home including section navigation, SSOT rules, task navigation, module/package navigation, and documentation conventions.

## Output

- A final `.wiki/INDEX.md`
- Completed section indexes and necessary project pages
- Remaining questions that require user confirmation
