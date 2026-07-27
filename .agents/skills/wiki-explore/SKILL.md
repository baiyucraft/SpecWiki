---
name: wiki-explore
description: Explore an unclear documentation request, gather repository evidence, and decide whether it needs one SpecWiki Lite change or a parent with child changes. Use before proposal when scope, ownership, or delivery shape is unresolved.
---

# Wiki Explore

Operate only in a Codex repository. Read source files and `.wiki` pages directly; never create a persistent code index, knowledge graph, or alternate metadata store.

## Preconditions

- The request is not yet specific enough for measurable proposal criteria, or it spans independently verifiable outcomes.
- Any existing change is absent or at `stage: exploration`.

## Inputs

- The user request and stated constraints
- `.wiki/INDEX.md`, relevant Wiki pages, and repository files needed as evidence
- `spec-wiki-lite status --json`
- Existing `meta.yaml`, `split.md`, or `research/*.md` when continuing exploration

## Workflow

1. Identify the problem, affected readers, source-of-truth owners, and observable success boundary.
2. Record only evidence that changes scope, ownership, risks, or delivery shape.
3. Choose `single-change` when one result can be reviewed and archived independently.
4. Choose `multi-change` only when child outcomes have distinct acceptance boundaries or dependencies.
5. Keep the stage at `exploration`; do not edit final Wiki pages or pre-write implementation design.

## Outputs

- Optional `.spec/changes/<change-id>/research/<topic>.md` for durable evidence.
- For a standalone exploration: `meta.yaml` with a canonical id and `deliveryShape: single-change`.
- For a parent: `split.md`, parent `meta.yaml`, and child stub metadata with order and dependencies.

## Pause Conditions

- A material product choice, source-of-truth conflict, or destructive scope cannot be inferred safely.
- A requested child cannot be given an independent acceptance boundary.
- Existing parent and child metadata disagree on ids, order, or dependencies.

## Next Stage

Use `wiki-propose` for the standalone change or the earliest dependency-ready child. Keep parent changes out of implementation.
