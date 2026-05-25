---
name: wiki-init
description: "Use when the repository has not been bootstrapped for spec-wiki and you need to initialize the wiki runtime."
user-invocable: true
---

# wiki-init

## When To Use
- Initialize the wiki runtime for a repository that has not been bootstrapped yet.

## Run
- Run `spec-wiki wiki init` from the repository root.
- Relay the final result or error as-is. Do not rewrite business semantics in the host layer.

## Interpret
- Report the CLI end state without overstating runtime completeness.
- `v0.2.0` treats a minimal knowledge runtime as the formal success contract. Do not rewrite it as an index-only bootstrap.

## Guardrails
- Route the entry point back to `spec-wiki` CLI only. Do not rewrite Wiki business logic in the host layer.
- If CLI or runtime returns an error, pass the clear error through directly. Do not silently rewrite the state.
