# CodeGraph Design Procedure

This is the design-specific evidence procedure:

1. Read the same change's `research/codegraph.md`; do not repeat a full scan.
2. Run `codegraph_impact` for proposed symbols to confirm impact radius.
3. Use `codegraph_callers` / `codegraph_callees` to confirm ownership, interface boundaries, and rollback effects.
4. Use `codegraph_trace` for cross-layer flows and `codegraph_affected` for tests when target files are known.
5. Write entry points, paths, dependency boundaries, impact, tests, rollback, graph/source differences, and unresolved items in the design template section.

Use CLI or targeted source verification when MCP is unavailable, recording fallback and residual risk. Never store large raw output.
