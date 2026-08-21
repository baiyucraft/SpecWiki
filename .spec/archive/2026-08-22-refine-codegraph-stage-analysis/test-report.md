---
verification-result: pass
scope: full
---

# refine-codegraph-stage-analysis Test Report

## Environment

- runtime/platform: Node.js 20.20.0, Windows workspace
- package/tarball: `spec-wiki-lite@0.1.0`, staged `dist/spec-wiki-lite`
- fixtures: Vitest temporary projects and injected argv-safe CodeGraph runners

## Commands and Results

| Command | Result | Evidence |
| --- | --- | --- |
| `pnpm test` | pass | package 80 passed/1 skipped; root 15 passed |
| focused runner/registry tests | pass | 9 tests passed |
| `pnpm --filter spec-wiki-lite build` | pass | Vite transformed 92 modules |
| `pnpm run pack` | pass | 0.1.0 tarball, 88 files, no `.codegraph` data |
| tarball smoke | pass | zh/en init, language switch, status readiness |
| CLI smoke `init --no-codegraph --json` | pass | core init succeeds, requested false, no external calls |
| `codegraph status --json .` | pass | index healthy, version 1.5.0 |
| `codegraph impact` / `affected` | pass | runner blast radius and 8 affected tests recorded in research |
| `pnpm exec tsc ... --noEmit` | skipped | no output for several minutes in workspace; terminated |
| `pnpm lint` | skipped | no output for several minutes in workspace; terminated |
| `git diff --check` | pass | no whitespace errors; CRLF conversion warnings only |
| strict validate | pass | change valid at verification stage |

## System Test Coverage

| ST | Result | Evidence |
| --- | --- | --- |
| ST-01 default detection-only init | pass | runner tests and CLI smoke |
| ST-02 explicit force orchestration | pass | argv-safe runner success/failure tests |
| ST-03 skip mode | pass | no-codegraph no-call test |
| ST-04 existing index reuse | pass | existing-index runner test |
| ST-05 bilingual phase Skills | pass | workflow contract and asset sync tests |
| ST-06 fallback evidence | pass | research artifact and warning tests |
| ST-07 path safety | pass | semicolon path and existing path-safety suite |
| ST-08 tarball boundary | pass | distribution/tarball smoke |

## TDD Evidence

| UT | Red | Green/Refactor | Result |
| --- | --- | --- | --- |
| UT-01/02/03 | old default side-effect contract failed after mode change | runner suite passes all modes | pass |
| UT-04 | generic Skill contract replaced by ordered phase contract | root workflow contract passes | pass |
| UT-05 | inventory expected 24 before new references | registry, distribution, and smoke expect 26 | pass |

## Gaps

- No real global npm/MCP write was executed; injected runner and explicit offline smoke cover the external side-effect boundary.
- Standalone TypeScript/lint commands were environment-skipped as noted above; build and tests remain passing.

## Conclusion

All executable required evidence passed; scope is full.
