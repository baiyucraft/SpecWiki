# General Review Standard

## Scope

Use for every full or partial SpecWiki Lite review. Read proposal, design, system tests, tasks, metadata, diff, and test evidence together.

## Blocking / P0

- Security: injection, path escape, command execution, authorization bypass, sensitive-data leakage, or unsafe deserialization.
- Correctness: null/boundary/error branches, state transitions, idempotency, partial-failure inconsistency, or resource leaks.
- Artifact drift: implementation contradicts goals, non-goals, success criteria, design, or task/test mapping.
- Interface/compatibility: CLI, schema, path, error, public output, or ownership conflicts with accepted contracts.
- Test gap: critical failure, safety, migration, rollback, or success criteria lack credible evidence.

## Non-blocking / P1/P2

- Material performance regression, unbounded loading, or repeated I/O.
- Mixed responsibilities, duplication, hidden state, or hard-to-test coupling.
- Weak observability, error context, or compatibility notes that do not block current correctness.

## Suggested Tooling

Use formatter, lint, type checker, static analyzer, coverage, dependency scanner, and diff checks for mechanical issues. Human review focuses on behavior, safety, contracts, and evidence.

## Non-goals and False Positives

- Do not block on personal naming or formatting preference.
- Do not review unrelated historical debt or third-party source.
- Missing a non-required tool is not itself a defect; decide whether evidence is actually insufficient.
