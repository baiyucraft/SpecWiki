# integrate-aoci-code-cognition design

## Solution overview

SpecWiki Lite 0.2.0 introduces a shared external-command layer and two independent tool adapters. CodeGraph remains the precise structural analyzer. AOCI-CODE becomes the persistent governed semantic layer. Lite owns installation orchestration and readiness aggregation only; each upstream tool remains authoritative for its data and lifecycle.

The CLI performs deterministic bootstrap work synchronously: assets, pinned tool installation, MCP configuration, CodeGraph indexing, AOCI init/scan/doctor. It does not claim complete AOCI cognition before the Codex session can access the newly configured MCP. Instead it returns official next actions and keeps readiness false until AOCI verify/check/Guide report alignment.

## Public interfaces and stable contracts

- Package version: spec-wiki-lite 0.2.0.
- init accepts --no-codegraph and --no-aoci. Both are one-run deferrals and never persist a disabled state.
- The former positive --codegraph flag is removed because CodeGraph is now on by default.
- update accepts --tools to install, repair, or upgrade both tools to versions pinned by the current Lite release.
- status invokes both official status surfaces and returns exit code 2 when the project is not ready.
- init returns exit code 2 when core assets succeeded but either required tool remains incomplete; hard asset/path/config failures return 1.
- Stable output moves external state under tools.codegraph and tools.aoci and includes nextActions.

Pinned compatibility manifest:

- CodeGraph: npm @colbymchenry/codegraph@1.6.0 plus its registry integrity identity.
- AOCI-CODE: v0.1.0-rc12 and the six official archive names/SHA-256 values for win32/linux/darwin and x64/arm64.
- AOCI user install root: SPEC_WIKI_LITE_TOOLS_DIR when explicitly set for tests/administration; otherwise the platform user data directory under spec-wiki-lite/tools/aoci/0.1.0-rc12/.
- MCP configuration always references the resolved stable absolute executable.

## Ownership and data flow

- Lite manifest -> safe downloader/hash/extractor -> user-local AOCI executable -> official init/scan/doctor -> AOCI-owned formal cognition and machine state.
- Pinned npm CodeGraph -> official install/init/status -> CodeGraph-owned local index.
- AOCI Overview + CodeGraph queries + source/tests -> .spec evidence -> curated durable conclusions in .wiki.
- Lite asset sync continues to own only .wiki baselines and .agents/skills/wiki-*.
- AOCI owns its AGENTS marker block. Lite must never replace AGENTS.md as a whole or edit inside the AOCI markers.
- Database source configuration, credential references, Evidence, Baseline, and cognition stay exclusively in AOCI files. .wiki/config.yaml remains the Wiki/Skill language SSOT.

## Normal flow

1. init validates path/config and synchronizes localized Wiki/Skills.
2. CodeGraph adapter detects exact version; if absent or incompatible it installs the pinned npm version, configures Codex globally, and initializes/rebuilds the project index.
3. AOCI adapter selects the exact platform asset, downloads archive and SHA-256-verified bytes, extracts into a temporary sibling, verifies --version, then atomically publishes the version directory.
4. AOCI adapter runs init with the Wiki locale and Codex host, scan, and doctor.
5. Tool inspection obtains codegraph status --json plus AOCI verify/check/Guide JSON. Output records restartRequired and upstream next actions when full cognition is not yet possible.
6. wiki-continue obtains AOCI rules and complete Overview after MCP is available, follows Guide/Maintain until aligned, then routes Wiki bootstrap or the active .spec stage.
7. During changes, AOCI provides semantic/global context and CodeGraph provides exact structural evidence. After code/tests stabilize, apply/review follows AOCI Maintain and proves alignment.

## Failure, boundaries, and rollback

- Invalid platform/architecture: fail the AOCI adapter with an actionable warning and readiness false; do not execute fallback or source builds.
- Download/hash/extraction failure: delete only the verified temporary directory; preserve any prior pinned installation and never execute unverified bytes.
- Repeated/concurrent installs: use versioned directories plus an exclusive lock and atomic rename; an already verified exact version is reused.
- Partial tool failure: preserve successful core assets and the other tool; return exit 2 with precise nextActions. Never report ready from directory existence alone.
- Version mismatch: status reports expected and actual. update --tools installs the pinned version; AOCI upgrade stops if official Guide reports pending Recovery or unresolved governance work.
- User content: preserve unknown Wiki config fields, user pages, unregistered Skill files, AOCI formal cognition, and existing non-AOCI AGENTS content.
- Path safety: project paths remain resolveSafePath guarded; external commands receive argv arrays with shell:false; archive extraction rejects absolute paths, drive prefixes, parent traversal, and symlink entries.
- Database: ordinary status performs no database network call. It reads official declarations/access/cognition status only. Explicit verify during review/archive is allowed only for a declared source and reports redacted upstream results.
- Session boundary: when MCP is not loaded, return restartRequired and exact follow-up instruction. Do not simulate MCP authoring through CLI.

## Verification design

- All process, downloader, filesystem-root, platform, and environment dependencies are injectable.
- Unit tests use fake runners and local fixture archives; no test mutates real Codex configuration, global npm state, or database services.
- CLI/tarball smoke uses fake tool binaries injected through environment and verifies exit 0/2/1 semantics.
- Content contracts assert bilingual AOCI/CodeGraph division, references, initial bootstrap order, and absence of duplicated state-machine instructions.
- Real self-bootstrap separately verifies the pinned Windows archive checksum, CodeGraph 1.6.0, init/scan/doctor, and the restart boundary.

## Wiki and long-term contract landing

- Overall design and Agents design: dual cognition architecture and stage responsibilities.
- CLI, configuration/artifacts, quick start, and FAQ: default installation, deferral, readiness, machine-local files, and restart workflow.
- Testing/acceptance: supply-chain, mocked external side effects, real self-bootstrap, and database redaction boundaries.
- Release contract becomes 0.2.0; capability baseline gains external cognition bootstrap/readiness requirements.

## Reference boundary

- Source: AOCI-CODE v0.1.0-rc12 official README, install, agent integration, cognition refresh, Overview delivery, and Database Evidence contracts.
- Target: Lite adapters, readiness aggregation, existing eight Skills, and Wiki bootstrap.
- Adoption: behavioral rewrite around public CLI/MCP contracts. No upstream source, runtime, template text, or internal workflow is copied.

## Rollback

- Reverting Lite restores the 0.1.0 public contract without deleting AOCI/CodeGraph user data.
- Tool rollback selects a previously verified versioned binary/package and reruns upstream doctor/status; it never rewrites aoci*.txt or resets Baselines merely because the executable changed.
- Failed initialization may be retried after the exact nextAction. No automatic cleanup removes user-authored cognition.

## CodeGraph-derived design constraints

- entry points and call paths: execute(init/update/status) -> runBootstrapInit/getProjectStatus; runBootstrapInit currently calls syncProjectAssets then runCodeGraphIntegration.
- ownership and dependency boundaries: external adapters sit below init/status; asset registry owns only localized Wiki/Skills; validate/archive continue to own .spec lifecycle.
- impact radius: CLI, init, status, exported types, runner, assets, docs, package version, distribution tests, and archive readiness.
- affected tests: CodeGraph reported 11 current suites including runner, init, status, Lite red, asset sync, Wiki inspection, current-surface, distribution, skill assets, tarball smoke, and workflow contract.
- rollback boundary: preserve successful assets and upstream-owned data; only verified temporary download/extraction paths are removable.
- graph evidence vs source verification: CodeGraph confirmed callers/callees and test reachability; targeted reads confirmed exact current arguments, readiness calculation, and ownership behavior.
- unresolved items: AOCI JSON field details remain adapter-local and are validated against the pinned real binary before Green completion.
