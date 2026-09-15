# AOCI-CODE research summary

## Source and version

- Canonical source: https://github.com/aoci-spec/aoci-code
- Pinned target: v0.1.0-rc12, published as a prerelease on 2026-09-14
- License: FSL-1.1-MIT; Lite downloads and invokes it but does not redistribute its binary
- Release inventory: Windows/Linux/macOS for amd64/arm64 plus SHA256SUMS, SBOM, Sigstore, and provenance assets

## Confirmed capabilities

- Formal cognition is stored in Git-reviewable aoci.txt, aoci.meta.txt, and aoci.code.txt; an enabled database adds aoci.database.txt.
- .aoci contains local state, drafts, Ledger, transactions, and recovery evidence and is normally ignored.
- Codex uses project-level .codex/config.toml pointing to a stable absolute aoci executable path.
- init writes an empty Volumes skeleton and rules; scan establishes the Managed Baseline; an MCP-enabled Agent authors complete semantics through the official Guide/Maintain workflow.
- Repository alignment is determined by verify --json, check --json, and index agent guide --agent codex --json. A wrapper must not duplicate that state machine.
- Database Evidence supports PostgreSQL, MySQL, and constrained openGauss 6.0.5; it reads Schema catalogs, not business rows, and stores only an environment-variable reference for credentials.

## Product responsibility split

| Layer | Authority |
| --- | --- |
| AOCI-CODE | persistent system semantics, responsibilities, strong relations, constraints, Code/Database Cognition, governance alignment |
| CodeGraph | current-source symbols, call paths, impact radius, affected tests |
| .spec | change requirements, design, tasks, implementation, review evidence |
| .wiki | durable human/Agent documentation after cross-verification |
| source/config/tests | final truth for current behavior |

## Integration constraints

- Every project requires AOCI Code Cognition; Database Cognition becomes required only after a source is declared.
- AOCI owns its AGENTS managed block and host configuration; Lite verifies boundaries but does not overwrite those bytes.
- Wiki bootstrap first obtains reliable AOCI Overview, then validates claims with CodeGraph and targeted source/tests; it never copies either index mechanically.
- Per-session Overview/Attestation reliability is Agent session state, not a new Lite persistent schema.

## Unknowns and handling

- AOCI JSON may evolve within future pinned versions: use injected runners and tolerant parsing that preserves unknown fields; inability to establish required facts fails closed with summarized official next actions.
- MCP dynamic loading is host-dependent: init reports restartRequired when cognition cannot continue in the current session rather than claiming completion.
