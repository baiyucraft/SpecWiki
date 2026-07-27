---
name: wiki-review
description: Perform final review and verification for an implemented SpecWiki Lite change, then write machine-readable full/pass evidence when all gates succeed. Use at implementation, review, or verification stage.
---

# Wiki Review

Operate only in a Codex repository. Review the whole change against its own artifacts and the Lite CLI contract; do not treat a partial check as archive evidence.

## Preconditions

- The selected change is not a parent and is at `implementation`, `review`, or `verification`.
- Implementation tasks and checklists are complete.
- The implementation diff and all declared artifacts are available.

## Inputs

- Proposal, design, system tests, optional unit tests, tasks, metadata, and implementation diff
- Relevant Wiki pages, package assets, test output, and repository quality commands
- `spec-wiki-lite validate <change-id> --strict --json`

## Workflow

1. Advance to `stage: review` while evidence is being collected.
2. Review correctness, artifact consistency, navigation, frontmatter, links, ownership, path safety, and regression risk.
3. Run every declared unit/system test plus repository lint, build, package, and diff gates that apply.
4. Map results back to `ST-*`, success criteria, and remaining risks.
5. Advance to `stage: verification` only after both formal reports are complete and full/pass.

## Outputs

- `review-report.md` beginning with `review-result: pass` and `scope: full` only when no blocking issue remains
- `test-report.md` beginning with `verification-result: pass` and `scope: full` only when required evidence passes
- `meta.yaml` at `stage: verification` after a successful full review
- A successful strict validation at verification stage

## Pause Conditions

- Any blocking defect, failed required test, missing evidence, or artifact drift remains.
- Review scope is partial; record partial/skipped evidence and do not advance to verification.
- A newly discovered issue invalidates earlier pass evidence; fix it through `wiki-apply` and rerun the full review.

## Next Stage

Use `wiki-archive` only after strict validation confirms full/pass evidence.
