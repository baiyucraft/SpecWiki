# spec-wiki

`spec-wiki` provides a local Repo Wiki shared by people and Agents. It scans a repository, builds the repo-local knowledge runtime under `.wiki/`, and installs managed integrations for Codex, Claude, and CodeBuddy.

## Quick Start

```bash
spec-wiki init --host codex --repo-root .
spec-wiki status --repo-root .
spec-wiki query "payment flow" --repo-root .
spec-wiki update --repo-root .
```

`init` is the single initialization entry point. It bootstraps the selected host assets, initializes the runtime, and returns a landing status. Use repeated `--host` flags or a comma-separated `--hosts` value when multiple hosts are needed.

## Commands

Default help highlights the main product path:

```text
spec-wiki init [--host <host> | --hosts <host,host>] [--repo-root <path>] [--no-interactive]
spec-wiki status [--repo-root <path>]
spec-wiki query <term...> [--repo-root <path>]
spec-wiki update [--repo-root <path>] [--bridge-stdio]
```

`spec-wiki --help-all` also lists the implemented advanced commands:

```text
spec-wiki sync [--repo-root <path>]
spec-wiki rebuild [--repo-root <path>] [--bridge-stdio]
spec-wiki changes [--repo-root <path>]
spec-wiki change <change-id> [--repo-root <path>]
spec-wiki validate <change-id> [--repo-root <path>]
```

Use `--json` for machine-readable JSON or NDJSON. `--bridge-stdio` implies machine mode and is available only for streaming commands. Exit codes are `0` for success, `2` for a partial unified init or invalid change validation, `64` for usage errors, and `1` for workflow or protocol failures.

## Runtime Contract

The runtime writes formal knowledge, projected pages, metadata, and recoverable cache data under `.wiki/`. Query routing follows `index -> knowledge -> page fallback`; structured governance references are included without copying change artifact bodies into the Wiki.

The JavaScript API keeps the existing `wikiInit`, `wikiStatus`, `wikiQuery`, `wikiUpdate`, `wikiSync`, and `wikiRebuild` identities. Host assets keep `wiki-*` skill names and Claude `/wiki:*` entry identities while invoking the top-level CLI.

Long-lived architecture and contribution guidance starts at [.wiki/INDEX.md](./.wiki/INDEX.md).

## Platform And License

The current release target is Windows x64. This project is licensed under GNU GPL v3.0.
