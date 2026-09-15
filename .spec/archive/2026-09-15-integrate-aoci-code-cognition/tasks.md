# integrate-aoci-code-cognition tasks

implementation-mode: tdd

## Task overview

Tasks follow the design capability blocks: pinned/safe tooling, AOCI and CodeGraph adapters, readiness/CLI, bilingual workflow assets, documentation/distribution, and final self-bootstrap.

## 1. Pinned tool infrastructure

- [x] 1.1 Red: add UT-01/UT-02 tests for manifests, platform mapping, argv safety, timeout, and injected execution.
- [x] 1.2 Green: implement the compatibility manifest and shared safe command seam.
- [x] 1.3 Refactor: centralize tool result, warning, platform, and version types.

### CheckList

- [x] Red failure is caused by missing target behavior.
- [x] Green and refactor focused tests pass.
- [x] No shell string construction or unbounded output.

## 2. AOCI verified installation and official lifecycle

- [x] 2.1 Red: add UT-03/UT-05/UT-06 tests for release selection, hash/extraction safety, init/scan/doctor, official gates, and database condition.
- [x] 2.2 Green: implement AOCI installer, bootstrap, inspection, and redacted database mapping.
- [x] 2.3 Refactor: isolate download/filesystem/process seams and atomic recovery.

### CheckList

- [x] Six platform assets and hashes covered.
- [x] Invalid archive bytes are never executed.
- [x] No database network call in ordinary status.
- [x] No credential value can enter public output.

## 3. Required CodeGraph and project readiness

- [x] 3.1 Red: add UT-04/UT-07 tests for default install, defer, exact version, JSON health, exit codes, and update --tools.
- [x] 3.2 Green: refactor CodeGraph adapter and wire dual-tool init/status/update.
- [x] 3.3 Refactor: remove legacy --codegraph mode and directory-only readiness.

### CheckList

- [x] Missing/deferred/stale/mismatched tools produce exit 2 and nextActions.
- [x] Core asset failure remains exit 1.
- [x] Existing exact healthy installs are idempotently reused.

## 4. Bilingual Skills, Wiki, and ownership contracts

- [x] 4.1 Red: add UT-08 content/inventory tests for AOCI/CodeGraph division and bootstrap ordering.
- [x] 4.2 Green: update zh/en eight Skills, references, Wiki templates, and repo-local synchronized assets.
- [x] 4.3 Refactor: remove generic/repeated upstream workflow text and preserve stable identifiers.

### CheckList

- [x] AOCI and CodeGraph evidence are separate and stage-specific.
- [x] AOCI AGENTS/config/formal assets remain upstream-owned.
- [x] Wiki remains curated documentation rather than an index mirror.

## 5. 0.2.0 distribution and aggregate verification

- [x] 5.1 Red: add UT-09 package/distribution/tarball expectations.
- [x] 5.2 Green: bump package contract and update README/Wiki/capability documentation.
- [x] 5.3 Refactor: align all help, examples, errors, tests, and current authority pages.
- [x] 5.4 Run package/root tests, lint, both typechecks, build, pack, smoke, strict validation, and git diff check.
- [x] 5.5 Perform real CodeGraph/AOCI self-bootstrap, record restart/alignment boundary, full review, verification, and archive.

### CheckList

- [x] Tarball contains no external runtime or local state.
- [x] All executable aggregate gates pass.
- [x] Wiki updates are complete and current.

## Case-to-task mapping

| System tests | Tasks / UT |
| --- | --- |
| ST-01, ST-02 | 1, 3 / UT-02, UT-04, UT-07 |
| ST-03, ST-04 | 1, 2, 3 / UT-01..05 |
| ST-05, ST-06 | 2, 3 / UT-05..07 |
| ST-07, ST-08 | 4 / UT-08 |
| ST-09 | 5 / UT-09 |

## Execution order

1. Tool manifest and process seam.
2. AOCI installer/lifecycle/database status.
3. CodeGraph required-mode and public readiness.
4. Localized Skills and Wiki contracts.
5. Version/distribution, real self-bootstrap, review, and archive.

## Deferred items

- npm publish, tag, merge, automated source build fallback, cloud Secret providers, and non-Codex hosts.
