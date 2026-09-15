# CodeGraph Evidence

## Objective and index state

- Objective: identify current integration entry points, ownership boundaries, call paths, and affected tests.
- Indexed on: 2026-09-15
- codegraph status: 114 files, 554 nodes, 1,436 edges, up to date.

## Queries and findings

| Tool | Query | Finding |
| --- | --- | --- |
| codegraph explore | external analysis integration, init, status, Skills | cli.ts -> runBootstrapInit -> runCodeGraphIntegration is the current external-tool path; getProjectStatus only checks .codegraph existence |
| codegraph explore | config, readiness, asset ownership | .wiki/config.yaml currently owns language only; registry/sync owns localized Skills; ready aggregates Wiki, Skills, and changes |
| codegraph explore | metadata, validation, archive | Lite validate/archive exclusively own stage artifacts and full/pass gates |

## Confirmed entry points and impact

- packages/spec-wiki-lite/src/cli.ts owns arguments, init/status/update output, and exit codes.
- orchestration/init/runInit.ts coordinates assets and external tools.
- orchestration/codegraph/runner.ts supplies the existing argv-safe runner seam.
- core/status.ts owns public readiness aggregation.
- core/assets/registry.ts owns the bilingual Skill/reference inventory.

## Test leads

- Directly affected suites: runner, init, CLI, status, registry/content contracts, distribution, and tarball smoke.
- External commands must continue to use argv arrays, shell:false, hidden Windows processes, and injectable runners.
- AOCI adds network download, hash, extraction, stable-path, and partial-failure recovery cases.

## Facts, inference, and fallback

- Confirmed: Wiki bootstrap instructs Codex to inspect repository facts; CodeGraph is currently used by staged .spec Skills rather than as a Wiki generator.
- Inferred: AOCI is appropriate as semantic prior for bootstrap and development, while source/tests remain final truth.
- Unknown: CodeGraph CLI lacks context/trace commands, so those remain MCP-preferred with explore/source fallback.
- Fallback: targeted reads of cli.ts, status.ts, Skill assets, and Wiki templates confirmed details not provided by the graph.
