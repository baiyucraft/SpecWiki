# integrate-aoci-code-cognition TDD unit tests

## UT-01 Compatibility manifest

- Test: orchestration/tools/manifest.test.ts
- Modify: orchestration/tools/manifest.ts
- Mapping: ST-03/ST-04 and supply-chain boundary.
- Red: no pinned dual-tool manifest or platform asset lookup exists.
- Green: expose immutable CodeGraph npm identity and six AOCI asset/hash records.
- Refactor: centralize normalized platform/architecture/version types.

## UT-02 Safe command runner

- Test: orchestration/tools/command.test.ts
- Modify: orchestration/tools/command.ts and existing CodeGraph adapter.
- Mapping: ST-01/ST-02/ST-08 and command-injection boundary.
- Red: runner is CodeGraph-specific and cannot represent shared timeouts/results.
- Green: use argv arrays, shell:false, hidden Windows process, bounded output, timeout, and injectable execution.
- Refactor: keep platform shims adapter-local where required.

## UT-03 AOCI verified installer

- Test: orchestration/aoci/installer.test.ts
- Modify: orchestration/aoci/installer.ts.
- Mapping: ST-03/ST-04.
- Red: no downloader, hash verifier, safe extractor, lock, or atomic publish exists.
- Green: verify fixture archive identity and install exact executable safely.
- Refactor: isolate download/extract/filesystem dependencies for deterministic tests.

## UT-04 CodeGraph required adapter

- Test: orchestration/codegraph/runner.test.ts
- Modify: orchestration/codegraph/runner.ts.
- Mapping: ST-01/ST-02/ST-04/ST-05.
- Red: auto mode is detection-only and existing directory can bypass version/MCP/health checks.
- Green: default install exact 1.6.0, configure Codex, initialize/index, and parse status --json.
- Refactor: remove legacy positive force mode and normalize tool state.

## UT-05 AOCI bootstrap and inspection

- Test: orchestration/aoci/runner.test.ts
- Modify: orchestration/aoci/runner.ts.
- Mapping: ST-01/ST-04/ST-05.
- Red: no AOCI init/scan/doctor or verify/check/Guide integration exists.
- Green: implement official command sequence and fail-closed tolerant JSON mapping.
- Refactor: preserve raw unknown upstream fields only inside diagnostic evidence, not the stable public schema.

## UT-06 Database conditional gate

- Test: orchestration/aoci/database.test.ts
- Modify: orchestration/aoci/status.ts.
- Mapping: ST-06.
- Red: no declared-source/access/cognition distinction exists.
- Green: zero sources pass; declared sources require non-network access readiness and aligned cognition.
- Refactor: redact error summaries and forbid credential values in output.

## UT-07 Project status and CLI

- Test: core/status.test.ts and lite-red.test.ts
- Modify: core/status.ts, cli.ts, runInit.ts, bin.ts, index.ts.
- Mapping: ST-01/ST-02/ST-04/ST-05.
- Red: ready ignores tool health; status always exits 0; --codegraph remains positive opt-in; no --no-aoci/update --tools.
- Green: publish tools schema, nextActions, and exit 0/2/1 behavior.
- Refactor: share tool orchestration between init, update --tools, and status without side-effect leakage.

## UT-08 Localized workflow assets

- Test: scripts/tests/workflow-contract.test.ts and skill-assets.test.ts.
- Modify: bilingual Skill/reference/Wiki assets and registry.
- Mapping: ST-07/ST-08.
- Red: assets contain only CodeGraph phase rules and one stale default-install statement.
- Green: add AOCI-specific evidence/templates and exact stage division in both languages.
- Refactor: keep common concepts consistent while preserving localized prose and stable identifiers.

## UT-09 Distribution

- Test: distribution.test.ts and tarball-smoke.test.ts.
- Modify: package metadata, build/pack scripts, README, Wiki release contract.
- Mapping: ST-09.
- Red: package remains 0.1.0 and smoke does not model mandatory tools.
- Green: verify 0.2.0 tarball and fake-tool lifecycle.
- Refactor: keep network and machine configuration outside aggregate tests.

## Coverage boundaries

- Tests do not connect to real production databases or expose credentials.
- Upstream internal governance branches are not reimplemented or exhaustively tested; adapters test the public output contract and fail closed on ambiguity.
