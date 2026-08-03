# Go Review Standard

Use with the general standard, focusing on Go errors, context, concurrency, resources, and tests.

## Blocking / P0

- Do not discard critical errors; preserve context and machine-usable semantics.
- Propagate `context.Context` across I/O, RPC, DB, and long work; release resources after cancel/timeout.
- Goroutines need wait/cancel/ownership; shared maps/slices/structs need synchronization; channel-close ownership must be clear.
- Files, bodies, rows/transactions, locks, and temporary resources close or roll back on every path.
- Cover errors, cancellation, concurrency, resource release, and mocked external dependencies.

## Non-blocking / P1/P2

- Oversized interfaces, broad `any`, unclear package boundaries, material allocation/string waste, or weak log context.

## Tooling and False Positives

- Automate gofmt, go vet, staticcheck, race, coverage, and dependency scanning.
- Do not require an interface for every dependency or block on a tiny unallocated slice.
