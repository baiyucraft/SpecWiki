# Java Review Standard

Use with the general standard, focusing on exceptions, nulls, concurrency, resources, layering, and tests.

## Blocking / P0

- No empty catch, swallowed exception, `Throwable`, or unjustified broad `Exception` handling.
- Collection return and null/Optional contracts are clear; wrapper comparison, mutation, and view semantics are correct.
- Thread pools are bounded, named, and have rejection policy; shared mutable state is synchronized.
- Streams, connections, statements, results, and transactions close or roll back reliably.
- Controller/Service/Repository, DTO/Entity/Domain, and transaction boundaries preserve business semantics.
- Cover exception, transaction, concurrency, collection boundary, and mocked external dependency paths.

## Non-blocking / P1/P2

- Weak log context, oversized classes/methods, complex streams/lambdas, injection tradeoffs, or dependency maintenance risk.

## Tooling and False Positives

- Automate formatter, SpotBugs, PMD, Error Prone, JUnit, coverage, and dependency scanning.
- Do not block mechanically on framework annotation or Javadoc preference.
