---
name: wiki-archive
description: Archive a change exactly once through the Lite CLI after strict full-pass gates, while verifying parent-child and durable Wiki state.
---

# Wiki Archive

Let `spec-wiki-lite archive` exclusively own directory moves and parent-child synchronization. Never move a change manually, edit archive markers, or fabricate timestamps.

## Preconditions

- An ordinary or child change is at `verification` or `archive` with completed tasks.
- `review-report.md` has `review-result: pass` and `scope: full`.
- `test-report.md` has `verification-result: pass` and `scope: full`.
- A parent is eligible only after every child is physically archived and metadata matches split markers.

## Workflow

1. Run `spec-wiki-lite status --json`.
2. Run `spec-wiki-lite validate <change-id> --strict --json`; stop on any blocker.
3. Confirm `.spec/archive/YYYY-MM-DD-<change-id>` does not exist.
4. Determine durable Wiki status truthfully:
   - `wiki-updates-made`: current-authority pages were updated;
   - `wiki-updates-required`: required knowledge is missing, so return to apply/review;
   - `wiki-updates-not-needed`: provide evidence that no durable knowledge changed.
5. Run `spec-wiki-lite archive <change-id>` exactly once.
6. Verify the active directory is gone, the dated archive exists, and evidence was not rewritten.
7. For a child, verify the active parent received `archiveStatus`, `archivedAt`, `archivedTo`, and the split marker.
8. Run status again and record the archive path plus the next dependency-ready child.

## Outputs

- Dated archive path.
- Post-archive status, parent-child synchronization, and truthful Wiki-update state.
- The ordinary/child change is absent from active changes.

## Pause Conditions

- Strict validation fails, reports are partial/non-pass, or tasks are incomplete.
- Archive target conflicts or parent-child metadata disagrees.
- `wiki-updates-required` remains unresolved.

## Constraints

- Invoke archive exactly once; do not retry blindly after a failure.
- Do not bulk-rewrite historical `.spec/archive/**`.
- Archive never substitutes for implementation or durable Wiki updates.
## CodeGraph Stage Action

Archive never runs CodeGraph automatically. Only inspect durable documentation or impact scope when needed, recording a concise summary in existing review/verification evidence; Lite CLI and strict validation remain the archive authority.
