---
review-result: pass
scope: full
---

# integrate-aoci-code-cognition review report

## Review conclusion

- review-result: pass
- scope: full
- The implementation matches the approved proposal, design, cases, and task plan.
- No Lite-core blocking defects were found in the CLI, tool adapters, asset ownership, readiness aggregation, or distribution boundary.

## Review scope

- Pinned CodeGraph and AOCI tool seams, argv safety, timeout/output bounds, and platform manifest.
- AOCI download/hash/extraction safety, versioned installation, lifecycle commands, database gate, and redacted status.
- CodeGraph exact-version detection, MCP configuration, project index reuse/sync, and readiness semantics.
- Bilingual Wiki/Skill assets, stage-specific CodeGraph/AOCI evidence contracts, and package tarball contents.
- CLI exit-code behavior, deferred tool modes, status/readiness reporting, and strict change validation.

## Evidence reviewed

- Package Vitest: 16 files, 89 passed, 1 skipped.
- Root contract/tarball Vitest: 5 files, 18 passed.
- `pnpm --filter spec-wiki-lite build`: pass.
- `pnpm build`: pass.
- `pnpm run pack`: pass; staged tarball contains only package runtime, assets, README, and LICENSE.
- Package typecheck, root typecheck, lint, and `git diff --check`: pass.
- CodeGraph self-check after `codegraph sync`: version 1.6.0, 125 indexed files, 716 nodes, 1828 edges, zero pending changes.
- AOCI Windows x64 binary: pinned version 0.1.0-rc12 installed and `doctor` passed; database source count is 0.

## Findings

### Blocking findings

- None.

### Non-blocking residual risk

- AOCI `verify` and `check` currently report `governance_aligned: false` because this repository has an empty cognition skeleton and the current Codex session has not reloaded the newly configured AOCI MCP. The adapter reports this truthfully through `governanceAligned`, `restartRequired`, warnings, and `nextActions`; it does not mark the project ready or fabricate cognition evidence.
- AOCI's `doctor --json` currently emits human-readable diagnostics despite the flag. The implementation therefore gates on exit status for doctor and does not parse that output as JSON.
- AOCI's full repository drift report includes historical/upstream paths outside Lite's ownership. Those paths remain upstream-owned and are not rewritten by this change.

## Contract checks

- No legacy `--codegraph` positive mode remains in the public CLI.
- `--no-codegraph` and `--no-aoci` are one-run deferrals; `update --tools` is the explicit repair path.
- External commands use argument arrays with `shell:false`; user paths are not interpolated into shell strings.
- `.codegraph`, `.aoci` runtime state, databases, sockets, daemons, and logs are excluded from the package tarball.
- Historical `.spec/archive/**` content is not rewritten.
- AOCI semantic evidence and CodeGraph structural evidence remain separate and stage-specific.

## Review decision

The Lite implementation is suitable for archive. The AOCI alignment/session boundary is an explicitly documented external follow-up and does not invalidate the Lite change because core initialization, tests, package gates, and strict artifact contracts pass.
