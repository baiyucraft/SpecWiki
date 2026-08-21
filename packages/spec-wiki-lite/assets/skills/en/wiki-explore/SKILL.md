---
name: wiki-explore
description: Explore an unclear documentation or workflow request, collect scope-changing evidence, and choose a single change or parent-child delivery shape.
---

# Wiki Explore

Own only `exploration`: clarify the problem, inspect necessary repository facts, record research, and decide delivery shape. Do not pre-write proposal, design, tests, or implementation.

## Preconditions

- The request lacks a verifiable boundary or may contain independently accepted outcomes.
- The target change is absent or remains at `stage: exploration`.

## Inputs

- User goals, constraints, and confirmed decisions.
- `.wiki/INDEX.md`, relevant Wiki pages, code, configuration, tests, and historical archives.
- `spec-wiki-lite status --json` and existing `meta.yaml`, `split.md`, or `research/**`.

## Workflow

1. Ask only questions that block scope, ownership, or safety. Record safely inferred uncertainty as risks or unknowns.
2. Identify the problem, affected readers, SSOT owner, non-goals, and observable acceptance boundary.
3. Read repository facts on demand; never create a code index, knowledge graph, or hidden database.
4. When evidence changes scope, delivery shape, dependencies, or risk, read `references/research-template.md` and write `research/<topic>.md`.
5. Choose `single-change` when one result can be reviewed and archived independently. Choose `multi-change` only for distinct acceptance boundaries or dependencies.

## Single-change Stub

Create a standalone stub only when exploration research needs a stable location:

```yaml
id: <change-id>
stage: exploration
deliveryShape: single-change
artifacts:
  proposal:
    status: missing
  metadata:
    status: present
```

Do not create `proposal.md`; `wiki-propose` must complete it later.

## Parent and Child Stubs

The parent uses `deliveryShape: multi-change`, `multiChange.role: parent`, children/order/dependsOn, and `split.md`. Each child receives only an exploration metadata stub with `deliveryShape: single-change`, `multiChange.role: child`, and a missing proposal status.

Prefer child ids shaped as `<parent-id>-<topic>`. Split, parent metadata, and child metadata must agree exactly. A parent never enters implementation.

## Outputs

- Optional `research/<topic>.md`.
- A standalone exploration stub, or parent `split.md`, parent metadata, and child stubs.
- The earliest dependency-ready child and the next `wiki-propose` action.

## Pause Conditions

- A product choice, SSOT conflict, or destructive boundary cannot be inferred safely.
- A child lacks an independent acceptance boundary.
- Existing parent-child ids, order, or dependencies disagree.

## Next Stage

Use `wiki-propose` for the single change or earliest ready child. Keep the parent coordination-only.
## CodeGraph Stage Actions (required)

1. Call `codegraph_status` and record index status and time.
2. Use `codegraph_context` to map project areas and feature entry points.
3. Use `codegraph_explore` for key symbol source; when endpoints are known use `codegraph_trace` for the complete call path.
4. Use `codegraph_callers` / `codegraph_callees` only to confirm ownership.
5. Write a structured summary to `.spec/changes/<change-id>/research/codegraph.md`: goal, queries, entries/symbols/relations, impact, test leads, facts/inferences, unknowns, and fallback.

Use the equivalent CLI when MCP is unavailable. If neither is available, read targeted source/tests and record the fallback. Do not paste large raw output.
