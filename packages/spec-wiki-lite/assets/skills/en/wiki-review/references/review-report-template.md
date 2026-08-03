review-result: pending
scope: partial

# <change-id> Review Report

## Review Scope

- artifacts: <proposal/design/tests/tasks/meta>
- implementation diff: <commit/range/path>
- selected standards: general / frontend / go / java / python
- exclusions: <explicit exclusion and reason>

## Findings

| Priority | Location | Issue | Impact | Fix/return stage |
| --- | --- | --- | --- | --- |
| P0/P1/P2 | `<path:line>` | <reproducible issue> | <risk> | apply/design/plan |

## Artifact Consistency

| Artifact / success criterion | Implementation and evidence | Result |
| --- | --- | --- |
| <item> | <path/command> | pass / fail / skipped |

## Security, Ownership, and Rollback

- path/input safety: <result>
- user-content protection: <result>
- failure atomicity/rollback: <result>

## Residual Risks

- <risk; write “none” when absent>

## Conclusion

- Only for full scope with no blocking finding, change the first lines to `review-result: pass` and `scope: full`.
- Otherwise keep fail/pending/partial and state the next action.
