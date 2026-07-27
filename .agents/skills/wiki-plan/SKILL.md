---
name: wiki-plan
description: Convert an accepted SpecWiki Lite design into system tests, optional TDD unit tests, and ordered implementation tasks. Use for changes at design, cases, or tasks stage before implementation begins.
---

# Wiki Plan

Operate only in a Codex repository. Plan against the Lite CLI and `.wiki` files; do not introduce native runtime, index, or knowledge-layer work.

## Preconditions

- `proposal.md` and `design.md` are complete and mutually consistent.
- The change is `single-change`, is not a parent, and is at `design`, `cases`, or `tasks`.
- Every success criterion has a testable outcome.

## Inputs

- `proposal.md`, `design.md`, `meta.yaml`, and relevant research
- Existing test conventions and affected Wiki/CLI files
- `spec-wiki-lite validate <change-id> --strict`

## Workflow

1. Write normal, failure, and boundary system cases with stable `ST-*` ids.
2. When TDD is appropriate, define focused `UT-*` cases in `unit-tests.md`.
3. Build `tasks.md` in Red, Green, and Refactor order, with verification attached to each task group.
4. Map every success criterion and system case to a task and command or observable assertion.
5. Mark implementation ready only when the current user authorization already permits implementation.

## Outputs

- `system-tests.md`
- Optional `unit-tests.md`
- `tasks.md` with an `implementation-ready` signal
- `meta.yaml` at `stage: cases` while incomplete, then `stage: tasks` when planning is complete

## Pause Conditions

- A success criterion cannot be mapped to evidence.
- The design lacks an interface, ownership, safety, or rollback decision required by a test.
- Implementation authorization is absent; leave `implementation-ready` false.

## Next Stage

Use `wiki-apply` only after the plan is complete and implementation-ready.
