---
name: wiki-design
description: Design Wiki information architecture, page ownership, navigation, and validation for an accepted SpecWiki Lite proposal. Use when a single change is at proposal, delivery, or design stage.
---

# Wiki Design

Operate only in a Codex repository. Keep `.wiki` as the final knowledge surface and avoid index, graph, cache, or projection designs.

## Preconditions

- `proposal.md` is complete and accepted.
- The change is `single-change`, is not a parent, and is at `proposal`, `delivery`, or `design`.
- `spec-wiki-lite validate <change-id> --strict` has no proposal-stage blocking issue.

## Inputs

- `proposal.md`, `meta.yaml`, and relevant research
- Existing `.wiki/INDEX.md`, target pages, adjacent indexes, and source-of-truth files
- Current CLI and Skill contracts when the change affects workflow behavior

## Workflow

1. Define target pages, navigation edges, frontmatter ownership, and source-of-truth boundaries.
2. Define link, orphan, duplicate authority, and rollback behavior.
3. Describe interfaces and file operations precisely enough to test without prescribing incidental code structure.
4. Record provenance for every adopted external design.
5. Do not edit product Wiki pages while the design is still being decided.

## Outputs

- `.spec/changes/<change-id>/design.md`
- `meta.yaml` advanced to `stage: design`, preserving unknown fields
- A successful strict validation for the design stage

## Pause Conditions

- Design work changes proposal goals, non-goals, or success criteria.
- Target page ownership conflicts with an existing source of truth.
- A parent change or unresolved dependency is selected.

## Next Stage

Use `wiki-plan` to create executable tests and tasks.
