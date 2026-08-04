# <change-id> Implementation Tasks

implementation-mode: <tdd|direct>

## tdd mode: Red

- [ ] R1 <Add test mapped to UT/ST/success criterion>
- [ ] R2 Run focused tests and record the relevant Red failure.

## Green

- [ ] G1 <Implement the smallest complete path>
- [ ] G2 <Implement fail-closed behavior, ownership, path safety, or rollback>
- [ ] G3 Run focused tests and record Green.

## Refactor and Verification

- [ ] F1 <Consolidate types, errors, duplication, and documentation>
- [ ] F2 Run package/root tests, lint, typecheck, build, pack, Wiki, and diff gates.
- [ ] F3 Update evidence and prepare full review.

## Success Criteria Mapping

| Success criterion | ST/UT | Task | Command/evidence |
| --- | --- | --- | --- |
| <criterion> | ST-01 / UT-01 | G1 | <command/path> |

> For `direct`, omit the Red section and use Implement → Verify → Refactor. Both modes still require tests, review, verification, and archive gates.
