---
name: wiki-apply
description: Execute an authorized SpecWiki Lite task plan with a valid implementation mode, mode-specific evidence, scope-drift handling, and safe failure recovery.
---

# Wiki Apply

Implement only confirmed tasks. You may edit source, tests, package assets, and final `.wiki` pages directly, but never build an index, knowledge graph, or hidden intermediate layer.

## Preconditions

- The change is a standalone or child `single-change` at `tasks` or `implementation`.
- `tasks.md` is complete with `implementation-mode: tdd` or `implementation-mode: direct`.
- The user authorized implementation and strict validation has no planning blocker.

## Inputs

- Proposal, design, system tests, unit tests, tasks, and metadata.
- Affected source, Wiki, tests, assets, and repository constraints.
- Focused and aggregate commands declared by the task plan.

## Authorization and Scope

- Execute only authorized tasks. “Finish” does not authorize external publishing, destructive migration, or undeclared product expansion.
- Return to `wiki-propose` for goal/non-goal/success changes, `wiki-design` for key design changes, or `wiki-plan` for case/task-only changes.
- Pause for unapproved path-safety, data-loss, or external-state risk.

## Implementation modes

1. Set `stage: implementation` while preserving unknown metadata.
2. `tdd`: run a relevant failure proving the target behavior is absent; infrastructure failure is not Red evidence; then perform Green and Refactor.
3. `direct`: implement the smallest complete behavior, run Verify, then Refactor; prior Red evidence is not required.
4. Check each task and record command/result immediately after evidence passes; failed tasks remain open.
5. Both modes must run the declared focused, aggregate, lint, typecheck, build, pack, and diff gates.

## Failure Recovery

- On sync or migration failure, verify atomic rollback and user-file protection.
- Diagnose failed tests, fix, and rerun. Never claim completion when required tests cannot run and no credible alternative exists.
- Scope drift requires updating the earlier artifact and strict validation, not an implementation-only workaround.

## Outputs

- Completed code, assets, Wiki, and automated tests.
- Current `tasks.md` evidence.
- Stage implementation, ready for final review.

## Pause Conditions

- New work exceeds proposal, design, or tasks.
- An uncovered security, path, data-loss, or external-state risk appears.
- Required tests cannot run and no credible alternative exists.

## Next Stage

Use `wiki-review` after all tasks and focused/aggregate checks pass. Do not sign formal reports here.

## AOCI Stage Action

Before implementation, read the accepted AOCI semantic constraints instead of overturning durable ownership from local code alone. After code and tests stabilize, follow the official AOCI Maintain/Guide flow and verify Code Cognition; also align Database Cognition when a source is declared. Pause and record the blocker when Guide enters Recovery, the MCP is unavailable in this session, or implementation changes an unplanned durable responsibility. Never hand-edit `aoci*.txt` to bypass the gate.

## CodeGraph Stage Action

Before editing target symbols, run `codegraph_impact` to confirm the design impact radius. After editing, use `codegraph_affected` to select tests and run the project test command. If impact crosses the design boundary, stop implementation and return to `wiki-design`. CodeGraph assists location only; it cannot replace safety checks or test conclusions.
