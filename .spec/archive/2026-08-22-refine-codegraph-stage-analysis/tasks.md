# Tasks

## Task Overview

implementation-mode: tdd

### 1. Refactor CodeGraph orchestration

- [x] 1.1 Red: add default detection-only, force, skip, existing-index, and JSON non-interactive tests.
- [x] 1.2 Green: implement intent-aware runner and safe reuse behavior.
- [x] 1.3 Refactor: normalize warnings, result schema, and command seams.

### CheckList

- [x] runner tests cover all modes
- [x] no shell injection path
- [x] warnings do not block core init

### 2. Stage-specific bilingual Skills and templates

- [x] 2.1 Red: add explore/design ordering and evidence contract tests.
- [x] 2.2 Green: add phase references, localized research/design fields, and remove generic blocks.
- [x] 2.3 Refactor: sync package and repo-local assets through registry.

### CheckList

- [x] zh/en references registered
- [x] explore/design artifact workflow explicit
- [x] other Skills phase-scoped

### 3. Verification and archive

- [x] 3.1 Run focused and aggregate tests, lint, typecheck, build, pack, and diff checks.
- [x] 3.2 Write full/pass review and verification evidence.
- [x] 3.3 Archive with Lite CLI and push `origin/lite`.

### CheckList

- [x] strict validate passes
- [x] archive path verified

## Case-to-task mapping

| Case | Task |
| --- | --- |
| ST-01..04 | 1 |
| ST-05 | 2 |
| ST-06 | 3 |

## Execution order

1. Red tests
2. Runner/CLI Green
3. Skill/template Green
4. Aggregate verification and archive

## Deferred items

- No Lite-owned graph persistence or new machine artifact id.
