---
verification-result: pass
scope: full
---

# integrate-aoci-code-cognition verification report

## Verification conclusion

- verification-result: pass
- scope: full
- All declared Lite package/root gates passed.
- Real CodeGraph synchronization passed.
- Real AOCI installation and doctor checks passed; AOCI governance alignment remains an explicit residual boundary and is reported as not ready rather than hidden.

## Automated results

| Command | Result |
| --- | --- |
| `pnpm test` | pass — package 89 passed / 1 skipped; root 18 passed |
| `pnpm lint` | pass |
| `pnpm exec tsc -p tsconfig.json --noEmit` | pass |
| `pnpm --filter spec-wiki-lite exec tsc -p tsconfig.json --noEmit` | pass |
| `pnpm build` | pass |
| `pnpm run pack` | pass |
| `git diff --check` | pass; only normal CRLF normalization warning |
| `node packages/spec-wiki-lite/bin/spec-wiki-lite.js validate integrate-aoci-code-cognition --strict --json` | pass; valid before reports were added |

## Distribution verification

- Tarball: `spec-wiki-lite-0.2.0.tgz`.
- Package contains Node CLI, bundled `dist`, bilingual Wiki/Skill assets, README, and LICENSE.
- Package does not contain Rust/native runtime, AOCI executable, CodeGraph database, `.codegraph`, `.aoci`, daemon, socket, or log files.
- Tarball smoke initialized both locales and exercised deferred `--no-codegraph --no-aoci` mode.

## External tool verification

### CodeGraph

- `codegraph sync`: pass; 1 changed file synchronized.
- `codegraph status --json`: initialized `true`, version `1.6.0`, 125 files, 716 nodes, 1828 edges, pending changes all zero, index state `complete`.

### AOCI-CODE

- Pinned Windows x64 release `0.1.0-rc12` installed after checksum verification.
- `aoci --repo . init --locale zh-CN --agent codex`: pass.
- `aoci --repo . scan`: pass.
- `aoci --repo . doctor --json`: pass by exit status; output is human-readable diagnostics.
- `aoci --repo . database source list --json`: pass; 0 configured sources and no network access.
- `aoci --repo . verify --json`: exit 1 with `governance_aligned: false`.
- `aoci --repo . check --json`: exit 1 with repository drift findings.

The failed AOCI alignment checks are not represented as Lite test failures. They are recorded as the expected external session/governance boundary in the review report and in the implementation's readiness output.

## Scenario coverage

- Exact healthy tool reuse and idempotent initialization.
- Missing/deferred/stale/mismatched tool readiness and next actions.
- Safe archive extraction, SHA-256 mismatch rejection, path traversal and link-entry rejection.
- Invalid paths, shell-injection-safe argv handling, bounded output, and command timeout.
- Bilingual asset install, locale migration, user-file preservation, managed force behavior, and complete Skill/reference inventory.
- Strict validation, parent/child dependency checks, archive report gates, and tarball isolation.

## Known limitations

- AOCI MCP was configured in the user-level Codex config but cannot be hot-reloaded into the already-running session. A fresh Codex session plus the official AOCI Guide is required to establish durable cognition alignment.
- No database cognition test was required because the repository declares zero database sources.
