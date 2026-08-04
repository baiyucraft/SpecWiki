# <change-id> Task Plan

implementation-mode: <tdd|direct>

## Task Overview

<Explain the task split using capability blocks, success criteria, and system-tests.md.>

## Implementation Mode

tdd | direct

`tdd`: write and confirm failing unit tests, implement the smallest behavior, then refactor. `direct`: use a structured implementation checklist for concrete modules/files/interfaces/configuration/data flows, adding unit tests or focused alternatives before each large task is complete; prior Red evidence is not required.

## 1. <Large task: capability block / success target>

### direct mode

- [ ] 1.1 Create or modify <concrete module or file> to support <behavior / design point>
- [ ] 1.2 Connect <call path, interface, configuration, or data flow> to <usage scenario>
- [ ] 1.3 Add or adjust <test file or focused verification> for <behavior / design point>

### tdd mode

- [ ] 1.1 Red: write the failing test for <behavior / design point> (UT-001) and confirm the expected failure
- [ ] 1.2 Green: implement the smallest behavior for UT-001
- [ ] 1.3 Refactor: clean up the implementation while keeping UT-001 passing

### CheckList

direct:

- [ ] Unit tests or a focused verification alternative cover the behavior
- [ ] Relevant verification passes
- [ ] Lint / typecheck / static analysis / formatter checks pass for this large task
- [ ] Commenting standards are checked

tdd:

- [ ] Red failure is confirmed
- [ ] Tests pass after the smallest implementation
- [ ] Tests still pass after refactoring
- [ ] Focused quality checks pass
- [ ] Commenting standards are checked

## Case-to-task Mapping

| System test case | Large task | Small task / verification |
| --- | --- | --- |
| ST-001 | 1. <large task> | 1.1 / 1.2 / UT-001 |

## Execution Order

- <Task dependency order, prerequisites, and suggested execution path>

## Deferred Items

- <Deferred work, reason, and trigger; write “None” when empty>

> Both modes require tests, review, verification, and archive gates. Lite does not use a readiness field or an extra confirmation step.
