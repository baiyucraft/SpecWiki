# spec-wiki

`spec-wiki` gives agents a lightweight, local Repo Wiki for a codebase on Windows x64.

In `v0.1.0`, it focuses on two practical jobs:

- bootstrap repo-local integrations for `Codex`, `Claude`, and `CodeBuddy`
- build an index-first runtime so you can query files, modules, symbols, and call paths before doing deeper code reading

## Why Use It

When an agent jumps straight into a large repo, it usually wastes tokens on blind search.

`spec-wiki` helps by giving the agent a fast structural map first:

- where the important code lives
- which files and symbols are related
- whether the local wiki runtime is ready, stale, or needs refresh

The goal of `v0.1.0` is not full repository documentation yet. The goal is a reliable first-pass repo map.

## Current Scope

`v0.1.0` currently guarantees:

- Windows x64 runtime support
- public CLI actions: `init`, `status`, `update`, `query`
- index-only runtime initialization and refresh
- index-first query results

`v0.1.0` does not yet treat full knowledge/page generation as a formal release contract.

## Quick Start

### 1. Bootstrap your agent host

```bash
spec-wiki init --tool codex --repo-root .
```

This writes managed assets for a supported host inside the repo.

Supported hosts:

- `codex`
- `claude`
- `codebuddy`

### 2. Build the local repo index

```bash
spec-wiki wiki init --repo-root .
```

This scans the repository and creates the local wiki cache.

### 3. Check whether the runtime is ready

```bash
spec-wiki wiki status --repo-root .
```

Use this before asking an agent to rely on the repo wiki.

### 4. Query the repo map

```bash
spec-wiki wiki query --repo-root . --term "payment flow"
```

You can also use positional text:

```bash
spec-wiki wiki query payment flow
```

### 5. Refresh after source changes

```bash
spec-wiki wiki update --repo-root .
```

## Two Different `init` Commands

This is the most important CLI distinction.

### `spec-wiki init`

This is the bootstrap command.

It installs repo-local host assets such as skills, hooks, or settings.

It does not build the wiki runtime.

### `spec-wiki wiki init`

This is the runtime command.

It scans the repository and initializes the local repo wiki cache.

It does not install host bootstrap assets.

## Install Or Run From Source

If you are running from this repository:

```bash
pnpm install
pnpm build
node dist/spec-wiki/bin/spec-wiki.js --help
```

After installation, you can use the binary directly:

```bash
spec-wiki --help
```

## Command Reference

### Bootstrap

```bash
spec-wiki init [--tool <host> | --tools <host1,host2>] [--repo-root <path>] [--no-interactive]
```

### Runtime

```bash
spec-wiki wiki init [--repo-root <path>] [--bridge-stdio]
spec-wiki wiki status [--repo-root <path>]
spec-wiki wiki update [--repo-root <path>] [--bridge-stdio]
spec-wiki wiki query [--repo-root <path>] --term <text>
spec-wiki wiki query [--repo-root <path>] <query text>
```

Notes:

- `--bridge-stdio` only applies to long-running actions such as `init` and `update`
- `query` requires either `--term` or positional query text

## What Gets Created

The runtime currently writes a local `.wiki/` directory for the repository.

In `v0.1.0`, the main runtime artifact you should rely on is:

```text
.wiki/
└─ .cache/
   └─ wiki-cache.db
```

## Query Contract In v0.1.0

For `query`, the stable fields to rely on are:

- `query_mode`
- `query_trust`
- `recommended_action`
- `matched_pages`
- `provenance_summary`

Recommended usage:

1. run `query` to narrow the search space
2. identify the most relevant files, modules, or symbols
3. read code directly when implementation detail matters

`query` is meant to reduce search cost, not replace code reading.

## What It Is Good For

- giving agents a fast structural map of a repository
- reducing blind file reads and wasted tokens
- deciding whether the repo wiki needs `init` or `update`
- finding the next files or symbols worth inspecting

## License

This project is licensed under `GNU GPL v3.0`.

## TODO

- formal knowledge/page runtime support
- richer research and answer assembly
- broader platform support
