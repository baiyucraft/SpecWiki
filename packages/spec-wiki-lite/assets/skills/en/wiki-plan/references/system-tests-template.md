# <change-id> System Tests

## Environment

- runtime/platform: <version>
- fixture/data: <repeatable data>
- external dependency: <mock / local / none>

## ST-01 <Normal Scenario>

- Type: normal
- Given: <state>
- Action: <user or system action>
- Assertions: <observable result>
- Evidence: <command/output/path>

## ST-02 <Failure Scenario>

- Type: failure
- Given: <invalid input or fault>
- Action: <action>
- Assertions: <fail-closed behavior, error, and zero side effects>
- Evidence: <command/output/path>

## ST-03 <Boundary Scenario>

- Type: boundary
- Given: <repeat, empty, maximum, conflict, or platform boundary>
- Action: <action>
- Assertions: <stable behavior>
- Evidence: <command/output/path>

## Success Criteria Mapping

| Success criterion | ST | Evidence |
| --- | --- | --- |
| <criterion> | ST-01 | <evidence> |
