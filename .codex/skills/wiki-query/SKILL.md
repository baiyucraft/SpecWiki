---
name: wiki-query
description: "Use when you need a fast structured map of where code lives and how files, modules, symbols, or call paths relate before deeper inspection."
user-invocable: true
---

# wiki-query

Use `wiki-query` when you need a fast structured map of how code works together before deeper inspection.

This is a working pattern, not a rigid output template.

## When To Use
- Use it when the user asks where code lives, what a module does, or which files, modules, symbols, or call paths relate to a concept.
- Use it proactively when the agent needs a fast structured map of how code works together before deciding which files to inspect next.

## How To Work
- Extract a concise query term from the request, or use the provided term directly.
- Run `spec-wiki wiki query --term "$ARGUMENTS"` from the repository root.
- Look at these stable fields first to decide whether the result is sufficient: `query_mode`, `query_trust`, `recommended_action`, `matched_pages`, `provenance_summary`.
- Treat the query result as a structured map for narrowing the search space. Prioritize relevant pages, files, symbols, or call-path clues instead of restating the JSON payload.

## After This
- If query already answers the question, respond directly from the hits.
- If query only gives direction, continue with `rg`, targeted file reads, symbol-level analysis, or implementation-level verification.
- If the result is insufficient, state the coverage gap clearly and suggest a better next action or a narrower query.

## Action Notes
- `v0.2.0` formally routes query through `index -> knowledge -> page fallback`.
- Read `provenance_summary` as route tags first: `index_hit`, `knowledge_hit`, `page_fallback`.

## Guardrails
- Route the entry point back to `spec-wiki` CLI only. Do not rewrite Wiki business logic in the host layer.
- If CLI or runtime returns an error, pass the clear error through directly. Do not silently rewrite the state.
- If the query result is insufficient, say so directly. Do not fill in symbol, module, or page content that does not exist.
- Do not rebuild a new Wiki state machine or page semantic layer from query results.
- Do not invent missing pages, modules, symbols, or knowledge projections.
- Do not treat wiki-query as a replacement for `rg`, targeted file reads, or full implementation review. Read code when precise implementation details matter.
