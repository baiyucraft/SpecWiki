# System Tests

- ST-01: default `init --json` initializes Lite assets without invoking npm, MCP install, or project indexing.
- ST-02: `init --codegraph --json` invokes version detection and missing-tool installation/configuration/indexing with structured warnings.
- ST-03: `init --no-codegraph --json` performs no external calls and reports `requested: false`.
- ST-04: existing `.codegraph` is reused without duplicate installation.
- ST-05: zh/en explore and design Skills require phase-specific tool ordering and evidence fields.
- ST-06: tarball excludes CodeGraph database, daemon, socket, and logs.
