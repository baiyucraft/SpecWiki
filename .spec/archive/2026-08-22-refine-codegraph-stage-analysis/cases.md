# System and Boundary Cases

## Normal

- ST-01: default `init --json` detects CodeGraph without external installation, MCP configuration, or project indexing.
- ST-02: `init --codegraph --json` performs explicit installation/configuration/indexing and returns structured warnings.
- ST-05: zh/en explore and design Skills require ordered CodeGraph analysis and artifact evidence.

## Failure

- ST-03: `init --no-codegraph --json` performs no external calls and reports `requested: false`.
- ST-06: missing or failing CodeGraph falls back to source/tests and records residual risk rather than blocking Lite initialization.

## Boundary

- ST-04: an existing `.codegraph` is reused without duplicate install or index work.
- ST-07: command arguments remain argv-safe for paths containing shell metacharacters.
- ST-08: tarball excludes CodeGraph databases, daemons, sockets, and logs.
