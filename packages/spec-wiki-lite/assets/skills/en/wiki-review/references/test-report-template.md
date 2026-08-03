verification-result: pending
scope: partial

# <change-id> Test Report

## Environment

- runtime/platform: <version>
- package/tarball: <version or path>
- fixtures: <test data>

## Commands and Results

| Command/verification action | Result | Evidence summary |
| --- | --- | --- |
| `<command>` | pass / fail / skipped | <output/path/reason> |

## System Test Coverage

| ST | Type | Result | Evidence |
| --- | --- | --- | --- |
| ST-01 | normal/failure/boundary | pass/fail/skipped | <evidence> |

## Unit Test and TDD Evidence

| UT / suite | Red | Green/Refactor | Result |
| --- | --- | --- | --- |
| UT-01 | <failure> | <passing command> | pass/fail |

## Success Criteria Coverage

| Success criterion | ST/UT/command | Result |
| --- | --- | --- |
| <criterion> | <evidence> | pass/fail/skipped |

## Failures, Unverified Items, and Evidence Gaps

- failures: <none / detail>
- unverified: <none / reason and impact>
- evidence gaps: <none / return stage>

## Conclusion

- Only when all required evidence passes with full scope, change the first lines to `verification-result: pass` and `scope: full`.
