---
name: wiki-propose
description: Create or complete a focused SpecWiki Lite proposal and metadata for one executable documentation change. Use after exploration has established scope, ownership, and a single-change delivery boundary.
---

# Wiki Propose

Operate only in a Codex repository. Use the Lite CLI for status and validation; do not recreate its metadata schema inside the Skill.

## Preconditions

- The target is a standalone change or child with `deliveryShape: single-change`.
- The change is new or at `exploration`, `proposal`, or `delivery`.
- Parent/child ids, order, and dependencies are already settled.

## Inputs

- Confirmed problem and scope
- Relevant `.wiki` pages and repository evidence
- Existing `research/*.md`, parent `split.md`, and child `meta.yaml` when present
- `spec-wiki-lite show <change-id> --json`

## Workflow

1. Define the problem, goals, non-goals, affected surfaces, measurable success criteria, and risks.
2. Cite upstream or external material with its source, target landing point, and adoption mode.
3. Preserve unknown metadata fields when changing stage or artifact status.
4. Write only proposal-level decisions; defer page layout and implementation mechanics to design.

## Outputs

- `.spec/changes/<change-id>/proposal.md`
- `.spec/changes/<change-id>/meta.yaml` with `stage: proposal` and the proposal artifact present
- A successful `spec-wiki-lite validate <change-id> --strict`

## Pause Conditions

- The change is a parent rather than an executable standalone/child change.
- New evidence changes delivery shape, child boundaries, order, or dependencies; return to `wiki-explore`.
- Success criteria cannot be verified without an unresolved product decision.

## Next Stage

Use `wiki-design` after the proposal is accepted. Do not edit final Wiki pages during proposal.
