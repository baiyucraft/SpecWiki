---
name: wiki-continue
description: Inspect SpecWiki Lite active changes and route the selected change to the correct wiki workflow skill. Use when a user asks to continue, resume, or determine the next step for a documentation change.
---

# Wiki Continue

Operate only in a Codex repository. Treat `spec-wiki-lite` output and the files under `.spec/changes` as the workflow authority.

## Preconditions

- The repository has been initialized with `spec-wiki-lite init --host codex`.
- A change id is named by the user, exactly one active change can be selected without guessing, or no active change exists while Wiki bootstrap is pending.

## Inputs

- `spec-wiki-lite status --json`
- `spec-wiki-lite show <change-id> --json`
- `spec-wiki-lite validate <change-id> --strict --json`
- The selected change's `meta.yaml` and reported artifacts

## Bootstrap Routing

- When there are no active changes and `status.wiki.bootstrapPending` is true, route to `wiki-explore`.
- `wiki-explore` should inspect repository facts and create the focused change needed to replace the bootstrap task with a final Wiki home and completed section pages.
- When there are no active changes and bootstrap is complete, report that no change is available to continue.
- Active changes always use stage routing below; bootstrap does not override their declared stage.

## Stage Routing

| Stage | Route |
| --- | --- |
| `exploration` | Use `wiki-explore` while scope or split artifacts are incomplete; otherwise use `wiki-propose`. |
| `proposal` / `delivery` | Use `wiki-propose` until the proposal and delivery boundary are complete; then use `wiki-design`. |
| `design` | Use `wiki-design` while design decisions are incomplete; otherwise use `wiki-plan`. |
| `cases` | Use `wiki-plan`. |
| `tasks` | Use `wiki-plan` until the plan is implementation-ready; then use `wiki-apply`. |
| `implementation` | Use `wiki-apply` until tasks and local checks are complete; then use `wiki-review`. |
| `review` | Use `wiki-review`. |
| `verification` | Use `wiki-review` if full/pass evidence is missing; otherwise use `wiki-archive`. |
| `archive` | Use `wiki-archive` to finish archive handling. |

## Outputs

- State the selected change id, current stage, blocking issues, target skill, and the exact next action.
- Delegate artifact edits to the target `wiki-*` skill; do not repair metadata while routing.

## Pause Conditions

- Multiple active changes are equally eligible and the user did not select one.
- Metadata is invalid, the id is unsafe, or required artifacts contradict the declared stage.
- A parent/child dependency prevents the selected change from advancing.

## Next Stage

Continue with the routed `wiki-*` skill. Do not skip stages or create another host projection.
