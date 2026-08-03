---
name: wiki-plan
description: Convert an accepted design into normal, failure, and boundary system tests, optional TDD unit tests, and an executable task plan.
---

# Wiki Plan

Own only `design`, `cases`, and `tasks`. The plan must cover every success criterion and support Red/Green/Refactor execution.

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

- When automation is appropriate, use `UT-*` and identify Test/Modify, relevant Red failure, minimal Green behavior, and Refactor guard.
- Map every UT to an ST, success criterion, or safety boundary; avoid implementation-detail-only coverage.

## Tasks

1. Red: add a relevant test that fails because the target behavior is absent; record evidence.
2. Green: implement the smallest complete path in dependency order.
3. Refactor: consolidate types, errors, duplication, documentation, and aggregate verification.
4. Map each task to ST/UT, success criteria, and commands.
5. Set `implementation-ready: true` only when the user has authorized implementation; otherwise pause with false.

## Browser Automation

Browser tooling is optional evidence. Use it only when the project starts safely, data is controlled, permissions are sufficient, and side effects are acceptable. Prefer existing project tooling. When unavailable, record the fallback reason and use component tests, API/CLI assertions, or structured manual evidence. A screenshot alone is never a pass assertion.

## Outputs

- `system-tests.md`
- `unit-tests.md` when TDD applies
- `tasks.md` with an implementation-ready signal
- Stage tasks and successful strict validation when planning is complete

## Pause Conditions

- A success criterion cannot map to evidence.
- Design lacks an interface, ownership, safety, or rollback decision.
- Implementation is not authorized.

## Next Stage

Use `wiki-apply` only with `implementation-ready: true`.
