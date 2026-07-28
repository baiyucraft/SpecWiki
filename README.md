# SpecWiki Lite

SpecWiki Lite is a TypeScript-only repository Wiki and staged documentation workflow for Codex. It keeps durable project knowledge in `.wiki/`, change evidence in `.spec/`, and eight repo-local workflow skills in `.agents/skills/`.

## Install

```bash
npm install -g spec-wiki-lite
spec-wiki-lite init --host codex
spec-wiki-lite status
```

Requires Node.js `>=20.19.0`. The initial package version is `0.1.0` and has no OS or CPU restriction.

## Commands

```text
spec-wiki-lite init [path] [--host codex] [--force]
spec-wiki-lite status [--json]
spec-wiki-lite show <change-id> [--artifact <artifact>] [--json]
spec-wiki-lite validate <change-id> [--strict] [--json]
spec-wiki-lite update [--force] [--json]
spec-wiki-lite archive <change-id>
```

`init` and `update` synchronize package-owned baselines and Skills. Scaffold pages and user pages are preserved. `status` checks Wiki structure, Skill installation, and active changes without scanning source code or creating a persistent index.

## Repository Model

```text
.wiki/
├── INDEX.md
├── 00-conventions/
│   ├── INDEX.md
│   └── 00-page-template.md
├── 01-project/
│   ├── INDEX.md
│   └── 00-overview.md
├── 02-development/
│   ├── INDEX.md
│   ├── 00-getting-started.md
│   └── 01-testing.md
├── 03-architecture/
│   ├── INDEX.md
│   └── 00-system-overview.md
└── 04-reference/
    └── INDEX.md
.spec/changes/<change-id>/     active workflow evidence
.spec/archive/<date>-<id>/     immutable archived evidence
.agents/skills/wiki-*/         Codex workflow skills
```

The initial Wiki contains prompts rather than invented project facts. Every scaffold page is created only when missing and is never overwritten by `update`, including `update --force`. The managed convention index is the only Wiki baseline that `--force` may refresh.

The Wiki uses `INDEX.md` for navigation. Ordinary pages use stable Markdown links and YAML frontmatter with `title`, `description`, `updated`, and `owner`.

## Development

```bash
pnpm install
pnpm test
pnpm lint
pnpm build
pnpm run pack
```

The distribution contains only the Node CLI, `dist`, templates, Skills, README, and LICENSE. This repository does not publish from the `lite` branch as part of the current change.

Chinese documentation: [README-CN.md](./README-CN.md).
