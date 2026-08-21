# CodeGraph Explore Procedure

This is the explore-specific analysis procedure, not a generic tool list.

1. Call `codegraph_status` first and record index availability and time.
2. Use `codegraph_context` to map the task area, entry points, and symbols.
3. Use `codegraph_explore` for relevant symbol source; treat returned source as already read.
4. When endpoints are known, call `codegraph_trace` for the complete path instead of rebuilding it with repeated searches.
5. Use `codegraph_callers` / `codegraph_callees` only to confirm ownership where needed.
6. Write a structured summary to `.spec/changes/<change-id>/research/codegraph.md` with impact, test leads, facts/inferences, unknowns, and fallback reasons.

Use the equivalent CLI when MCP is unavailable. If neither is available, read targeted source/tests and record the fallback. Never store large raw output.
