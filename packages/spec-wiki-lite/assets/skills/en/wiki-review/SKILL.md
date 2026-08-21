---
name: wiki-review
description: Perform full or partial review of a complete SpecWiki Lite change, map tests to criteria, and issue formal full/pass evidence only when justified.
---

# Wiki Review

Review the whole change against its artifacts. A focused check is partial evidence and never an archive gate.

## Preconditions

- The change is not a parent and is at `implementation`, `review`, or `verification`.
- Tasks and checklists are complete; implementation diff and required evidence are available.

## Inputs

- Proposal, design, system tests, unit tests, tasks, metadata, and full diff.
- `spec-wiki-lite validate <change-id> --strict --json`.
- `references/review-report-template.md`, `references/test-report-template.md`, and `references/review-standard.md`.
- Select frontend, Go, Java, or Python standards from the actual diff and domain, never from a config filename alone.

## Review Workflow

1. Set `stage: review` while preserving unknown metadata.
2. Review correctness, security, paths, failure atomicity, ownership, navigation, frontmatter, links, compatibility, and regression risk.
3. Add domain standards based on the real technology surface; every finding must describe a reproducible current-change risk.
4. Run all declared UT/ST plus repository lint, typecheck, build, pack, and diff gates.
5. Map results to success criteria, ST/UT, tasks, commands, and residual risks.
6. Only with full scope, no blocking finding, and all required evidence passing, write the templates with:

```text
review-result: pass
scope: full
```

and:

```text
verification-result: pass
scope: full
```

7. After both reports are complete, set `stage: verification` and run strict validation.

## Evidence Boundaries

- Record partial/skipped reasons, impact, and return stage; never present them as full.
- Screenshots, traces, and video are supporting evidence only; require a non-image assertion or alternative evidence.
- Tool-detectable formatting/lint issues do not replace human correctness review.
- A new defect invalidates earlier pass evidence; return to `wiki-apply`, fix, and rerun full review.

## Outputs

- `review-report.md`
- `test-report.md`
- Stage verification and successful strict validation only for full/pass evidence

## Pause Conditions

- A blocking defect, failed required test, missing evidence, or artifact drift remains.
- Review scope is partial.
- Implementation contradicts proposal or design.

## Next Stage

Use `wiki-archive` only after strict validation confirms full/pass evidence.
## CodeGraph Stage Action

Run `codegraph_impact` for final-diff key symbols and `codegraph_affected` for changed files, then cross-check the concise findings with tests, source facts, and the design impact radius. Graph output helps regression scope only and cannot replace a full review.
