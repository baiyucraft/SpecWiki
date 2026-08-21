# Design: Stage-Oriented CodeGraph Analysis

## Ownership and Interfaces

`runner.ts` owns external command execution and detection only. CLI parsing owns user intent (`auto`, `force`, `skip`) and interactivity. Skills own human-readable artifact evidence; Lite does not persist raw graph output.

## CodeGraph-derived design constraints

- Entry and call paths: `init` -> `runBootstrapInit` -> `runCodeGraphIntegration`; explore/design Skills write evidence into the current change. This is supported by `research/codegraph.md` and the CodeGraph explore/impact/affected results.
- Ownership and dependency boundaries: CodeGraph remains an external read-only analyzer; `.spec` remains the workflow SSOT.
- Impact radius: CLI parsing, runner orchestration, localized skill registry/assets, and contract tests.
- Affected tests: runner, init/CLI, workflow content, tarball smoke, aggregate gates.
- Rollback boundary: failed external commands become warnings; Wiki/.spec sync is independent and remains atomic.
- Graph evidence vs source verification: graph results are concise leads, not final correctness claims; tests and targeted source reads remain authoritative.
- Unresolved items: actual MCP availability is environment-dependent and must be recorded as fallback evidence.

### Evidence cross-check

- Graph evidence confirms the runner/test/public-index blast radius; targeted source reads confirm the exact mode branches and argument arrays.
- The installed CLI does not expose `codegraph_trace`; the Skill therefore requires the MCP tool when available and records CLI/source fallback when it is not.

## Failure and Safety

- Default init detects only; `--codegraph` enables install/MCP/project init; `--no-codegraph` skips all external calls.
- Existing `.codegraph` is reused. Status never mutates global or project state.
- Arguments use `spawn` arrays with `shell: false`; project paths are never shell-concatenated.

## Verification

The contract tests assert tool ordering, localized evidence fields, warning degradation, no raw output persistence, and no duplicate generic CodeGraph block.
