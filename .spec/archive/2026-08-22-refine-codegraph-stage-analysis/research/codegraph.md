# CodeGraph Research Evidence

phase: exploration
service-boundary: stage-analysis-and-init-orchestration

## Analysis Goal

Confirm the Lite init call path, CodeGraph runner ownership, affected tests, and the boundary for phase-specific Skill evidence.

## Index

- status: available
- version: 1.5.0
- generated-at: 2026-08-21T18:38:09.924Z
- project: `E:/project/!byAI/spec-wiki`
- state: complete; reindex not recommended
- observed pending changes: 10 modified files

## Queries

| tool | query | result |
| --- | --- | --- |
| `codegraph_status` | `codegraph status --json .` | healthy Node SQLite index; 114 files, 545 nodes, 1274 edges |
| `codegraph_explore` | `runBootstrapInit runCodeGraphIntegration ParsedCommand` | runner, bootstrap, CLI ownership and current source |
| `codegraph_impact` | `runCodeGraphIntegration` | runner, runner test, and public index are affected |
| `codegraph_affected` | changed CLI/runner files | 8 affected test files, including Lite and root contract tests |
| `codegraph_trace` | unavailable in installed CLI | MCP-only tool; used design fallback to source and explore output |

## Confirmed Facts

- `packages/spec-wiki-lite/src/cli.ts` parses `init` and delegates to `runBootstrapInit`.
- `runBootstrapInit` synchronizes package assets before invoking `runCodeGraphIntegration`.
- `runCodeGraphIntegration` owns command execution, warning normalization, CLI detection, Codex MCP detection, project-index detection, and force/auto/skip behavior.
- `runner.test.ts` covers argv-safe version/install/MCP/index calls, skipped mode, detection-only mode, existing-index reuse, and exception degradation.
- `scripts/tests/workflow-contract.test.ts` and package asset/status/sync tests cover localized Skill and reference ownership.

## Impact And Test Leads

- impact: CLI parsing, bootstrap orchestration, CodeGraph runner, localized registry/assets, package and root contract tests.
- affected tests: `packages/spec-wiki-lite/src/lite-red.test.ts`, runner/init tests, asset/status/wiki tests, `scripts/tests/current-surface.test.ts`, `skill-assets.test.ts`, and `tarball-smoke.test.ts`.
- boundary: CodeGraph remains external and read-only; `.spec` artifacts hold concise human-readable evidence only.

## Facts Vs Inferences

- confirmed: the current implementation has a direct CLI -> bootstrap -> runner path and no Lite-owned graph persistence.
- inferred: phase-specific references reduce repeated generic instructions and make explore/design evidence reviewable.

## Unknowns And Fallback

- unavailable tool: `codegraph_trace` is not exposed by the installed CLI; MCP availability depends on the host.
- fallback: used `codegraph_explore`, `impact`, `affected`, targeted source reads, and tests.
- residual risk: MCP-only dynamic call paths require host-level verification; external CodeGraph failures must remain warnings and be cross-checked by tests/source.

Only structured facts are recorded here; raw CodeGraph output is intentionally omitted.
