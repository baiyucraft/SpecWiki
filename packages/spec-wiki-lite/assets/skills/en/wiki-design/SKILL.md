---
name: wiki-design
description: Design interfaces, ownership, failure recovery, and verification boundaries for an accepted Wiki, Skill, CLI, or workflow proposal.
---

# Wiki Design

Make the design testable without pre-writing system tests, tasks, or implementation. `.wiki` remains the durable knowledge surface; do not introduce an index, graph, cache, or projection layer.

## Preconditions

- `proposal.md` is accepted and strict validation has no proposal blocker.
- The change is a standalone or child `single-change`, not a parent.
- Current stage is `proposal`, `delivery`, or `design`.

## Inputs

- Proposal, metadata, and existing research.
- Relevant Wiki, source, tests, configuration, CLI/Skill contracts, and SSOT owners.
- When needed, use `references/research-template.md` for evidence that changes design.

## Workflow

1. Read `references/design-template.md`.
2. Define target pages/assets, navigation, frontmatter, ownership, interfaces, file/data flow, and stable machine fields.
3. Define path safety, invalid input, repeat/concurrent execution, partial failure, atomicity, rollback, and user-content protection.
4. Design verification entry points, observable outputs, test seams, and success-criteria mapping without writing concrete ST/UT cases.
5. Record source, target landing area, and adoption mode for external material, distinguishing migration, rewrite, and inspiration.
6. Preserve unknown metadata, write `design.md`, set `stage: design`, and run strict validation.

## Boundaries

- Do not change proposal goals, non-goals, or success criteria; return to `wiki-propose` when needed.
- Do not edit final Wiki pages or product code.
- Do not promote incidental code structure into a public contract unless it defines safety, compatibility, or test boundaries.

## Outputs

- `.spec/changes/<change-id>/design.md`
- Metadata with design present and stage design
- Successful strict validation evidence

## Pause Conditions

- SSOT ownership conflict remains unresolved.
- The proposal lacks a product decision required for interface, safety, rollback, or verification.
- Design changes delivery shape or parent-child boundaries.

## Next Stage

Use `wiki-plan` for normal, failure, and boundary cases plus TDD tasks.
## CodeGraph Code Context

- When the project root contains `.codegraph/`, prefer CodeGraph for code location, context construction, and impact analysis.
- When MCP is available, prefer `codegraph_context`, `codegraph_explore`, `codegraph_search`, `codegraph_callers`, `codegraph_callees`, `codegraph_impact`, `codegraph_affected`, and `codegraph_status`; when only the CLI is available, use the corresponding `codegraph` command.
- If the index is absent, stale, or failing, fall back to ordinary source reading, tests, and project tools; never skip required safety, tests, or implementation verification.
- CodeGraph is an external read-only analysis tool. Lite does not create a second index, knowledge graph, database, or Wiki intermediary; databases, daemons, sockets, and logs are excluded from the package.
- Use callers, callees, and impact to identify ownership, interface boundaries, and rollback effects.
