---
name: wiki-apply
description: Implement an authorized SpecWiki Lite task plan with TDD, update task evidence, and advance the change to review readiness. Use for a single change at tasks or implementation stage.
---

# Wiki Apply

Operate only in a Codex repository. Edit source files and final `.wiki` Markdown directly; do not build a persistent repository index or knowledge graph.

## Preconditions

- The change is `single-change`, is not a parent, and is at `tasks` or `implementation`.
- `tasks.md` is complete and `implementation-ready` is true.
- `spec-wiki-lite validate <change-id> --strict` has no planning-stage blocker.

## Inputs

- `proposal.md`, `design.md`, `system-tests.md`, optional `unit-tests.md`, and `tasks.md`
- Affected source, Wiki, tests, and package assets
- Repository coding, documentation, and verification conventions

## Workflow

1. Set `stage: implementation` while preserving unknown metadata fields.
2. Execute tasks in order. In TDD mode, capture a relevant Red failure before production edits.
3. Make the smallest Green change, run focused tests, then refactor without losing coverage.
4. Update task and checklist state immediately after its evidence passes.
5. Run the declared focused and aggregate checks before declaring implementation complete.

## Outputs

- Implemented code, assets, Wiki pages, and automated tests
- Updated `tasks.md` with completed evidence
- `meta.yaml` remaining at `stage: implementation`
- Clean focused validation results ready for final review

## Pause Conditions

- A task requires changing proposal scope or an unresolved design decision.
- A security, path-safety, data-loss, or external-state risk is not covered by authorization.
- Required tests cannot run and no credible alternative evidence exists.

## Next Stage

Use `wiki-review` after every task and checklist is complete. Do not write final review or test reports in apply.
