# spec-wiki

`spec-wiki` gives agents a lightweight, local Repo Wiki for a codebase on Windows x64.

In `v0.2.0`, it focuses on two practical jobs:

- bootstrap repo-local integrations for `Codex`, `Claude`, and `CodeBuddy`
- build a minimal formal knowledge runtime so you can query files, modules, symbols, formal knowledge pages, and call paths before doing deeper code reading

## Why Use It

When an agent jumps straight into a large repo, it usually wastes tokens on blind search.

`spec-wiki` helps by giving the agent a fast structural map first:

- where the important code lives
- which files and symbols are related
- whether the local wiki runtime is ready, stale, or needs refresh

The goal of `v0.2.0` is a reliable first-pass repo map backed by a minimal formal knowledge runtime.

## Current Scope

`v0.2.0` currently guarantees:

- Windows x64 runtime support
- public CLI actions: `init`, `status`, `update`, `query`, `sync`, `rebuild`
- minimal formal knowledge runtime artifacts in `.wiki/.knowledge/**`, `.wiki/pages/**`, `wiki.metadata.json`, and recoverable `.wiki/.cache/**`
- knowledge runtime initialization and refresh
- query routing through `index -> knowledge -> page fallback`
- explicit page-writeback through `sync` and explicit full runtime rebuild through `rebuild`

The public release contract is now the minimal knowledge runtime, not a facts-only shortcut.

## Project Knowledge

Long-lived project knowledge, architecture notes, scenario boundaries, and contribution guidance live in [.wiki/INDEX.md](./.wiki/INDEX.md). The repository root keeps only product and agent entry points.

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

### 2. Build the local knowledge runtime

```bash
spec-wiki wiki init --repo-root .
```

This scans the repository and creates the local knowledge runtime and cache.

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

### 6. Sync managed `.wiki` page edits

```bash
spec-wiki wiki sync --repo-root .
```

Use this only after editing managed `.wiki` pages. It syncs page-layer changes back into runtime state, metadata, and cache.

### 7. Force a full rebuild when you need one

```bash
spec-wiki wiki rebuild --repo-root . --bridge-stdio
```

Use this only when you explicitly need a full runtime rebuild. It does not replace normal `update`.

## Two Different `init` Commands

This is the most important CLI distinction.

### `spec-wiki init`

This is the bootstrap command.

It installs repo-local host assets such as skills, hooks, or settings.

It does not build the wiki runtime.

### `spec-wiki wiki init`

This is the runtime command.

It scans the repository and initializes the local knowledge runtime and cache.
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
spec-wiki wiki sync [--repo-root <path>]
spec-wiki wiki rebuild [--repo-root <path>] [--bridge-stdio]
```

Notes:

- `--bridge-stdio` only applies to long-running actions such as `init`, `update`, and `rebuild`
- `query` requires either `--term` or positional query text
- `sync` only writes managed `.wiki` page edits back into runtime state; it does not replace `update`

## What Gets Created

The runtime currently writes a local `.wiki/` directory for the repository.

In `v0.2.0`, the runtime artifacts you should rely on are:

```text
.wiki/
|- .knowledge/
|- pages/
|- wiki.metadata.json
`- .cache/
   `- wiki-cache.db
```

## Query Contract In v0.2.0

For `query`, the stable fields to rely on are:

- `query_mode`
- `query_trust`
- `recommended_action`
- `matched_pages`
- `provenance_summary`

`provenance_summary` now carries stable route tags:

- `index_hit`
- `knowledge_hit`
- `page_fallback`

Recommended usage:

1. run `query` to narrow the search space
2. identify the most relevant files, modules, or symbols
3. read code directly when implementation detail matters

`query` is meant to reduce search cost, not replace code reading.

## Workflow Boundaries In v0.2.0

- `spec-wiki init` bootstraps host assets such as Codex, Claude, and CodeBuddy skills
- `spec-wiki wiki init` builds the repo-local knowledge runtime
- `status` reports runtime readiness and recommended next action
- `update` refreshes the knowledge runtime after source changes
- `sync` applies managed `.wiki` page edits back into runtime state, metadata, and cache
- `rebuild` is the explicit full runtime rebuild entry point

## What It Is Good For

- giving agents a fast structural map of a repository
- reducing blind file reads and wasted tokens
- deciding whether the repo wiki needs `init` or `update`
- finding the next files or symbols worth inspecting

## License

This project is licensed under `GNU GPL v3.0`.

## TODO

- richer research and answer assembly
- broader platform support
