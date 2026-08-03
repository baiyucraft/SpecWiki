---
name: wiki-propose
description: Create proposal.md and consistent metadata for one independently acceptable SpecWiki Lite change without entering design or implementation early.
---

# Wiki Propose

Own only the `proposal` and `delivery` stages for a concrete standalone or child change. Return to `wiki-explore` when scope still needs splitting.

## Preconditions

- The id is canonical kebab-case with no active or archived conflict.
- Delivery shape is settled as `single-change`.
- A child's parent, order, and dependencies match parent metadata and split.

## Inputs

- Confirmed problem, goals, non-goals, and constraints.
- Existing exploration stub, parent split, `research/**`, relevant Wiki pages, and repository evidence.
- `spec-wiki-lite show <change-id> --json` and strict validation results.

## Workflow

1. Reuse exploration research. Add focused research only for proposal gaps, using `references/research-template.md`.
2. Read `references/proposal-template.md` and define problem, goals, non-goals, success criteria, impact scope, delivery shape, risks, and references.
3. Success criteria must be observable, testable, and capable of producing review/verification evidence.
4. Record source, target landing area, and adoption mode for external or upstream material.
5. Preserve unknown metadata fields; then set proposal present and `stage: proposal`.
6. Run `spec-wiki-lite validate <change-id> --strict --json`.

## Parent-Child Consistency

- A parent is not an executable proposal target; operate only on a child or standalone change.
- A child proposal may not expand the parent split or alter dependencies.
- Return to `wiki-explore` when new evidence changes delivery shape, children, or order.

## Outputs

- `.spec/changes/<change-id>/proposal.md`
- Updated `meta.yaml` with proposal present and stage proposal
- Successful strict validation evidence

## Pause Conditions

- The selected change is a parent.
- Success criteria depend on an unresolved product decision.
- Scope cannot remain single-change or parent-child metadata disagrees.

## Next Stage

Use `wiki-design` after acceptance. Do not edit final Wiki pages or implementation here.
