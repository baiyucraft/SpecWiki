---
name: wiki-plan
description: Convert an accepted design into normal, failure, and boundary system tests, optional TDD unit tests, and an executable task plan.
---

# Wiki Plan

Own only `design`, `cases`, and `tasks`. Before writing tasks, select exactly one implementation mode: `tdd` or `direct`; do not choose for the user.

## Preconditions

- Proposal and design are complete, consistent, and free of strict-validation blockers.
- The change is an executable standalone or child, not a parent.
- Every success criterion has an observable outcome.

## Inputs

- Proposal, design, metadata, and research.
- Existing test conventions, commands, affected files, and available environments.
- `references/system-tests-template.md`, `references/unit-tests-template.md`, and `references/tasks-template.md`.
- For browser interaction, read `references/browser-automation.md`.

## System Tests

- Use stable `ST-*` ids for normal, failure, boundary, and regression scenarios.
- Define environment, data, action, assertions, fail-closed behavior, and evidence form.
- System evidence may be CLI, API, file inspection, integration automation, browser automation, or structured manual verification.

## TDD Unit Tests

- Generate `unit-tests.md` only for `tdd`, using `UT-*` with Test/Modify, Red rationale, the minimal Green behavior, and Refactor guards.
- For `direct`, still generate system tests and verification mappings, but do not require prior Red failure evidence.
- Map every UT to an ST, success criterion, or safety boundary; avoid implementation-detail-only coverage.

## Tasks

1. If the current request does not specify a mode, ask once: “Choose `tdd` or `direct` for this change.” Do not ask again when it is already explicit.
2. Record the stable field `implementation-mode: tdd` or `implementation-mode: direct` in tasks.
3. `tdd` uses Red → Green → Refactor and requires relevant Red failure evidence; `direct` uses Implement → Verify → Refactor and does not require Red evidence.
4. `direct` follows a structured implementation checklist: split by capability block/success target into numbered large tasks, give every large task a `### CheckList`, and start small tasks with verbs that name concrete modules, files, interfaces, configuration, or data flows.
5. Map each task to ST/UT, success criteria, and commands; both modes still require tests, review, verification, and archive gates.

## Browser Automation

Browser tooling is optional evidence. Use it only when the project starts safely, data is controlled, permissions are sufficient, and side effects are acceptable. Prefer existing project tooling. When unavailable, record the fallback reason and use component tests, API/CLI assertions, or structured manual evidence. A screenshot alone is never a pass assertion.

## Outputs

- `system-tests.md`
- `unit-tests.md` when `tdd` is selected; it may be omitted for `direct`
- `tasks.md` with a valid `implementation-mode`, task overview, numbered large tasks, checklists, case mapping, execution order, and deferred items
- Stage tasks and successful strict validation when planning is complete

## Pause Conditions

- A success criterion cannot map to evidence.
- Design lacks an interface, ownership, safety, or rollback decision.
- Implementation is not authorized.

## Next Stage

Use `wiki-apply` when tasks are complete, `implementation-mode` is valid, and the current user has explicitly authorized implementation. Missing or invalid mode routes back to this skill.

## AOCI Stage Action

Map `AOCI-derived semantic constraints` into system tests, tasks, and the pre-archive governance check: identify preserved responsibilities/invariants, the official Maintain timing, and the Database Cognition gate when a source is declared. AOCI does not replace ST/UT design or become another Lite task state machine.

## CodeGraph Stage Action

When target files or symbols are known, call `codegraph_affected` as needed and map affected tests into ST/UT and tasks. It is only a test lead, never a replacement for test design, failure coverage, or acceptance assertions; record the concise result in task evidence.
