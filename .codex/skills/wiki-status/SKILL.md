---
name: wiki-status
description: "Use when you need to know whether the wiki is ready, stale, blocked, or needs refresh."
user-invocable: true
---

# wiki-status

## When To Use
- Check whether the wiki is ready, stale, blocked, or what should happen next.

## Run
- Run `spec-wiki wiki status` from the repository root.
- Explain the current runtime state in user-facing language.
- Prefer `state`, `query_readiness`, and `recommended_action` from runtime output when explaining whether the wiki is queryable, stale, and what should happen next.

## Interpret
- Make it clear whether the repo has been bootstrapped and which runtime state it is in.
- Point to the next action when needed.
- `status` inspects runtime state only. It does not imply full knowledge/page completion.

## Guardrails
- Route the entry point back to `spec-wiki` CLI only. Do not rewrite Wiki business logic in the host layer.
- If CLI or runtime returns an error, pass the clear error through directly. Do not silently rewrite the state.
- `status` is only an inspection entry point. Do not misstate it as proof that the full knowledge/page runtime is complete.
