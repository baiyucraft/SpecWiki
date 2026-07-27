---
name: wiki-archive
description: Archive a fully verified SpecWiki Lite change, or a parent whose children are already archived, using the Lite CLI's fail-closed archive operation. Use only after review and verification are complete.
---

# Wiki Archive

Operate only in a Codex repository. Let `spec-wiki-lite archive` own directory moves and parent synchronization; never move change folders or edit archive markers manually.

## Preconditions

- An ordinary or child change is at `verification` or `archive` with full/pass review and test reports.
- A parent has every child physically archived with matching metadata and split markers.
- `spec-wiki-lite validate <change-id> --strict` succeeds.

## Inputs

- The complete active change directory
- Review and test report frontmatter
- Parent/child metadata, split markers, and dated archive directories when applicable
- Current Wiki status and any durable documentation updates required by the change

## Workflow

1. Run `spec-wiki-lite status --json` and strict validation.
2. Confirm the dated archive target does not already exist.
3. Run `spec-wiki-lite archive <change-id>` exactly once.
4. For a child, verify the active parent received `archiveStatus`, `archivedAt`, `archivedTo`, and the archived split marker.
5. Run status again and record the returned archive path.

## Outputs

- `.spec/archive/YYYY-MM-DD-<change-id>/` containing the unchanged evidence set
- No active directory for the archived change
- Synchronized active parent evidence for child archives
- Final archive path and post-archive status summary

## Pause Conditions

- Strict validation fails, evidence is partial, or the change is at an earlier stage.
- The archive target exists or parent/child metadata, dependencies, or markers disagree.
- Required durable Wiki updates are missing.

## Next Stage

There is no implementation stage after archive. Continue with the next dependency-ready child, or report the change closed.
