---
name: wiki-continue
description: Route a SpecWiki Lite change from authoritative status to the correct next workflow Skill, including bootstrap and parent-child dependencies.
---

# Wiki Continue

This is the Cross-stage routing entry point. It does not replace a stage-specific Skill. Work only in a Codex repository and treat the Lite CLI plus `.spec/changes/**` as authoritative.

## Preconditions

- The repository was initialized with `spec-wiki-lite init --host codex`.
- The user named a change, exactly one active change can be selected safely, or no active change exists while Wiki bootstrap is pending.

## Authoritative Inputs

Run and read, in order:

```text
spec-wiki-lite status --json
spec-wiki-lite show <change-id> --json
spec-wiki-lite validate <change-id> --strict --json
```

File existence is supporting evidence only. It never overrides metadata consistency, blocking issues, parent-child relationships, or full/pass report gates.

## Selection and Dependencies

- Use a user-named id when provided; never continue a change already under `.spec/archive/**`.
- Pause when multiple active changes cannot be selected without guessing.
- A parent never enters design, apply, or review. Select the earliest child whose `order` is ready and whose `dependsOn` changes are archived.
- Child metadata must match the parent's children, order, and dependencies.
- Pause on stage/artifact contradictions; do not silently rewind or repair stage.

## Bootstrap Routing

- No active change and `wiki.bootstrapPending: true`: route to `wiki-explore` to create the focused change that completes the Wiki home.
- No active change and bootstrap complete: report that there is nothing to continue.
- An active change always follows stage routing; bootstrap does not override it.

## Stage Routing

`implementation-mode` accepts only `tdd` or `direct`: a valid value routes directly to `wiki-apply`, while a missing or invalid value routes back to `wiki-plan`.

| Current state | Target Skill |
| --- | --- |
| `exploration`, scope/split/research incomplete | `wiki-explore` |
| `exploration`, standalone or child ready for proposal | `wiki-propose` |
| `proposal` / `delivery`, proposal incomplete | `wiki-propose` |
| `proposal` / `delivery`, proposal complete | `wiki-design` |
| `design`, design incomplete | `wiki-design` |
| `design` / `cases` | `wiki-plan` |
| `tasks`, missing or invalid `implementation-mode` | `wiki-plan` |
| `tasks`, valid mode and implementation authorized in the current request | `wiki-apply` |
| `implementation`, tasks or focused checks incomplete | `wiki-apply` |
| `implementation` / `review` | `wiki-review` |
| `verification`, full/pass evidence missing | `wiki-review` |
| `verification`, strict validation passes | `wiki-archive` |
| `archive` | `wiki-archive` |

## Outputs

Report the selected change, current stage, status summary, blocking issues, target Skill, reason, and exact next action, then continue the same change with that Skill.

## Pause Conditions

- Selection is ambiguous or the id is unsafe.
- Parent-child metadata, dependencies, stage, or artifacts disagree.
- Strict validation has an unresolved blocking issue.

## Constraints

- Route at most one target Skill per decision.
- Do not edit `meta.yaml.stage`, create stage artifacts, sign reports, or move change directories here.
- Never replace strict validation or full/pass evidence with subjective judgment.
## CodeGraph Stage Action

Continue routing never runs CodeGraph automatically. Only analyze code impact when needed, using the destination Skill's procedure. Lite CLI stage routing, strict validation, and `.spec` remain authoritative.
