# <change-id> TDD Unit Tests

## UT-01 <Behavior>

- Test: `<test-file>`
- Modify: `<production-file>`
- Maps to: ST-<id> / success criterion / safety boundary
- Red: <Relevant failure caused by missing behavior, not infrastructure>
- Green: <Smallest complete implementation>
- Refactor: <Structural improvement guarded by coverage>

## UT-02 <Failure or Boundary>

- Test: `<test-file>`
- Modify: `<production-file>`
- Input: <invalid/boundary>
- Assertions: <error, rollback, idempotency, or resource release>

## Coverage Boundary

- Do not test implementation detail that has no observable contract impact.
- Use fixtures/mocks for external dependencies; never connect to production.
