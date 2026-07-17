# spec-wiki

`spec-wiki` provides a local Repo Wiki shared by people and Agents. It scans a repository, builds the repo-local knowledge runtime under `.wiki/`, and installs managed host assets. Codex is the only reference host; Claude and CodeBuddy are compatible hosts.

## Quick Start

```bash
spec-wiki init --host codex --repo-root .
spec-wiki status --repo-root .
spec-wiki query "payment flow" --repo-root .
spec-wiki update --repo-root .
```

`init` is the single initialization entry point. It bootstraps the selected host assets, initializes the runtime, and returns a landing status. Repeat `--host` or use a comma-separated `--hosts` value to select multiple hosts.

## Commands

Default help highlights the main product path:

```text
spec-wiki init [--host <host> | --hosts <host,host>] [--repo-root <path>] [--no-interactive]
spec-wiki status [--repo-root <path>]
spec-wiki query <term...> [--repo-root <path>]
spec-wiki update [--repo-root <path>] [--bridge-stdio]
```

`spec-wiki --help-all` also lists the implemented advanced and governance commands:

```text
spec-wiki sync [--repo-root <path>]
spec-wiki rebuild [--repo-root <path>] [--bridge-stdio]
spec-wiki changes [--repo-root <path>]
spec-wiki change <change-id> [--repo-root <path>]
spec-wiki validate <change-id> [--repo-root <path>]
spec-wiki archive <change-id> [--dry-run | --apply | --resume <operation-id>] [--repo-root <path>]
```

Archive defaults to `--dry-run`; `--apply` performs the write and `--resume <operation-id>` recovers a partial operation. Exit codes are `0` for success, `2` for partial init, invalid validation, or an archive not-ready/recovery outcome, `64` for usage errors, and `1` for domain, workflow, protocol, I/O, or invalid archive failures.

## Runtime Contract

The runtime writes formal knowledge, projected pages, metadata, and recoverable cache data under `.wiki/`. Query accepts a non-empty term and returns canonical readiness, governance, `route_groups`, and `answer`; route-local scores are not compared across groups.

The JavaScript API keeps `wikiInit`, `wikiStatus`, `wikiQuery`, `wikiUpdate`, `wikiSync`, and `wikiRebuild`. All hosts receive repo-local `wiki-*` skills that invoke the top-level CLI; host assets do not define a second command or Runtime contract.

Long-lived architecture and contribution guidance starts at [.wiki/INDEX.md](./.wiki/INDEX.md). The versioned product surface is defined by the [v0.2.0 release contract](./.wiki/04-对外方法/02-v0.2.0发布合同.md).

## Platform And License

Windows x64 is the v0.2.0 release contract target. The manifest, staged package, build, tests, or dry-run do not by themselves prove a registry release. This project is licensed under GNU GPL v3.0.
