# Refactor CodeGraph Stage Analysis Integration

## Problem

The current integration repeats a generic CodeGraph paragraph in every Skill and does not produce stage-specific analysis evidence. It also performs external installation during ordinary `init`.

## Goals

- Make `wiki-explore` and `wiki-design` execute explicit CodeGraph analysis workflows and persist concise evidence in existing artifacts.
- Make `init` read-only by default for CodeGraph, with explicit `--codegraph`, `--no-codegraph`, and non-interactive behavior.
- Give propose/plan/apply/review precise, phase-specific CodeGraph use without introducing a second graph or context store.
- Localize the workflow and evidence templates in Chinese and English while keeping machine fields stable.

## Non-goals

- No Lite-owned index, graph database, daemon, or runtime.
- No changes to historical archive evidence.
- No automatic CodeGraph invocation from continue or archive routing.

## Success Criteria

- Explore records status/context/explore or trace evidence in `research/codegraph.md`, including facts, inferences, unknowns, fallback, impact, and test leads.
- Design records impact, ownership, callers/callees, trace, affected tests, rollback, and source verification constraints.
- Init performs no external CodeGraph side effect by default; explicit `--codegraph` does, `--no-codegraph` skips, and JSON never waits for input.
- All zh/en phase Skills and templates contain phase-specific rules and no repeated generic block.
- Tests, lint, typecheck, build, pack, strict validation, and diff checks pass.

## Impact Scope

- `packages/spec-wiki-lite/src/cli.ts`
- `packages/spec-wiki-lite/src/orchestration/codegraph/**`
- `packages/spec-wiki-lite/src/orchestration/init/**`
- bilingual `assets/skills/**` and synced `.agents/skills/**`
- workflow and package tests

## References

- source: `.upstream/codegraph`, CodeGraph official guidance
- source: Specoding, agent-harness, OpenSpec community patterns
- target: Lite CLI orchestration, `.spec` research/design artifacts, Codex Skills
- adoption: behavior and structure rewritten for Lite; no upstream runtime copied
