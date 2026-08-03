# Python Review Standard

Use with the general standard, focusing on exceptions, typing, async, paths, resources, security, and tests.

## Blocking / P0

- No bare `except:`, empty pass, or oversized try that swallows critical failure; preserve exception context.
- External input, configuration, environment, files, and serialized data need runtime validation; public contracts need type information.
- Async code must not run blocking I/O or `time.sleep` directly; handle cancellation, timeout, locks, and shared state.
- Use context managers for files, connections, locks, and temporary files; use structured path APIs and prevent escape.
- Avoid mutable defaults, collection mutation during iteration, untrusted pickle, and sensitive-data logging.
- Pytest covers exceptions, boundaries, async, resource release, and mocked external dependencies.

## Non-blocking / P1/P2

- Deep nesting, complex comprehensions, implicit API, weak log context, or dependency-locking risk.

## Tooling and False Positives

- Automate ruff/formatting, mypy/pyright, pytest, coverage, security, and dependency scanning.
- Do not block mechanically on local `Any` or absence of one particular model library.
