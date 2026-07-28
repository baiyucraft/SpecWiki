---
title: Wiki Bootstrap Task
description: Instructs Codex to build the project Wiki from verified repository facts.
updated: 2026-07-29
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

1. Decide whether the README, manifests, build and test configuration, and source entry points are sufficient to establish the project purpose, stack, and run path. Ask only for blocking facts.
2. Read repository evidence on demand rather than scanning everything indiscriminately.
3. Replace placeholders in each section index with verified project facts. Explain why a section is not applicable instead of inventing content.
4. Add only useful quick-start, module, CLI, API, configuration, deployment, or troubleshooting pages. Keep an `INDEX.md` in every Markdown directory.
5. Replace this entire page with the final Wiki home. Include section navigation, SSOT rules, task navigation, module or package navigation, and documentation conventions.

## Output

- A final `.wiki/INDEX.md`
- Completed section indexes and necessary project pages
- Remaining questions that require user confirmation
