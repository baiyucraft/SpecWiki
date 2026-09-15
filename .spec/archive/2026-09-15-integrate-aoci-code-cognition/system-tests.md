# integrate-aoci-code-cognition system tests

## Test environment

- Runtime/platform: Node.js >=20.19 on Windows/Linux/macOS platform fixtures.
- Fixtures: temporary Git repositories, fake npm/codegraph/aoci executables, local release archives, injected downloads and command runners.
- External dependencies: no real database or production credential; one isolated real self-bootstrap after mocked aggregate gates.

## ST-01 Default dual-tool bootstrap

- Type: normal
- Precondition: empty temporary Git project, no tools installed.
- Action: run spec-wiki-lite init --json.
- Assertions: localized assets are created; pinned CodeGraph and AOCI install paths are selected; official init commands are invoked safely; result exposes tools and nextActions; readiness is false until AOCI cognition aligns.
- Evidence: CLI/integration tests and JSON snapshot assertions.

## ST-02 Deferred external work

- Type: boundary
- Precondition: empty temporary project.
- Action: run init --no-codegraph --no-aoci --json.
- Assertions: neither external runner/download is called; core assets are created; both tools report deferred/incomplete; command exits 2 and status.ready is false.
- Evidence: injected spy runners and CLI exit-code assertions.

## ST-03 Pinned supply-chain installation

- Type: normal/failure
- Precondition: fixture archives for all six platform/architecture identities.
- Action: install AOCI through the adapter.
- Assertions: exact asset and SHA are selected; verified bytes extract atomically to stable user path; wrong hash, traversal, symlink, corrupt archive, timeout, or unsupported platform never executes or replaces a prior valid install.
- Evidence: installer unit tests and temporary filesystem inspection.

## ST-04 Tool version and upgrade behavior

- Type: boundary
- Precondition: missing, exact, older, newer, and partially installed tools.
- Action: init, status, then update --tools.
- Assertions: exact compatible versions are reused; mismatches are reported not ready; update --tools repairs to pinned versions; ordinary update has no tool side effects; AOCI pending Recovery prevents binary replacement.
- Evidence: adapter/CLI tests.

## ST-05 Official readiness aggregation

- Type: normal/failure
- Precondition: fake CodeGraph status and AOCI verify/check/Guide outputs for aligned, missing Baseline, stale, recovery, malformed JSON, and unavailable states.
- Action: run status --json.
- Assertions: readiness uses official outputs rather than directory existence; failures are fail-closed with stable reason and nextActions; status exits 0 only when fully ready, otherwise 2.
- Evidence: status tests and JSON contract tests.

## ST-06 Conditional Database Cognition

- Type: normal/boundary
- Precondition: AOCI fixtures with zero or one declared database source.
- Action: inspect status and archive gates.
- Assertions: zero sources do not block; a declared source requires credential access readiness, Evidence/Baseline, and Database Cognition alignment; no ordinary status command connects to the database or exposes credentials.
- Evidence: AOCI adapter command log and redacted fixture outputs.

## ST-07 Stage-specific Skill collaboration

- Type: normal
- Precondition: zh and en asset trees.
- Action: inspect all eight Skills and referenced templates.
- Assertions: AOCI provides Overview/global semantics and post-change maintenance; CodeGraph provides exact structural evidence; explore/design produce separate concise evidence; bootstrap starts after AOCI cognition is reliable; no Skill duplicates AOCI's state machine.
- Evidence: workflow/skill asset content tests.

## ST-08 Ownership and user-content protection

- Type: failure/boundary
- Precondition: existing AGENTS content with and without AOCI marker, custom Wiki pages, modified scaffold, and local host config.
- Action: init/update/retry.
- Assertions: Lite never overwrites AOCI formal assets, its AGENTS block, local .codex config, custom pages, or unregistered Skill files; only package-owned assets are synchronized.
- Evidence: asset sync and integration fixtures.

## ST-09 Distribution and self-bootstrap

- Type: regression
- Precondition: staged spec-wiki-lite 0.2.0 tarball.
- Action: install into temporary projects, run deferred and fake-tool full paths, then perform one real current-repository bootstrap.
- Assertions: tarball contains only Node CLI/dist/assets/docs/license; no external runtime or state; real CodeGraph/AOCI versions and checksum are verified; init/scan/doctor complete and restart boundary is truthfully reported.
- Evidence: pack manifest, tarball smoke, real command logs.

## Success-criterion mapping

| Success criterion | Tests |
| --- | --- |
| Default pinned dual-tool setup | ST-01, ST-03, ST-04 |
| Defer without false readiness | ST-02, ST-05 |
| Official status and conditional database gates | ST-05, ST-06 |
| Bilingual stage-specific Skills and Wiki bootstrap | ST-07 |
| Ownership, safety, and distribution boundaries | ST-03, ST-08, ST-09 |
| Aggregate quality gates | ST-09 |
