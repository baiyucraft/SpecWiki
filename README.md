# SpecWiki Lite

SpecWiki Lite is a TypeScript-only repository Wiki and staged documentation workflow for Codex. It keeps durable project knowledge in `.wiki/`, change evidence in `.spec/`, and eight repo-local workflow skills in `.agents/skills/`.

## Install

```bash
npm install -g spec-wiki-lite
spec-wiki-lite init --host codex
spec-wiki-lite status
```

Initialization uses Chinese Wiki content and paths by default. Use `spec-wiki-lite init --host codex --language en` for English.

Requires Node.js `>=20.19.0`. The initial package version is `0.1.0` and has no OS or CPU restriction.

## Commands

```text
spec-wiki-lite init [path] [--host codex] [--language zh|en] [--force]
spec-wiki-lite status [--json]
spec-wiki-lite show <change-id> [--artifact <artifact>] [--json]
spec-wiki-lite validate <change-id> [--strict] [--json]
spec-wiki-lite update [--force] [--json]
spec-wiki-lite archive <change-id>
```

`init` writes `.wiki/config.yaml` and synchronizes localized baselines and Skills. `update` reads `wiki.language` from that config. Scaffold pages and user pages are preserved. `status` checks Wiki structure, bootstrap completion, Skill installation, and active changes without scanning source code or creating a persistent index.

## Repository Model

```text
.wiki/
├── INDEX.md
├── config.yaml
├── 00-文档约定/
│   ├── INDEX.md
│   ├── 00-边界与SSOT规则.md
│   ├── 01-页面模板.md
│   └── 02-SpecWiki-Lite工作流.md
├── 01-快速上手/
│   └── INDEX.md
├── 02-开发指南/
│   ├── INDEX.md
│   └── 00-代码注释规范.md
├── 03-模块指南/
│   └── INDEX.md
└── 04-对外方法/
    └── INDEX.md
.spec/changes/<change-id>/     active workflow evidence
.spec/archive/<date>-<id>/     immutable archived evidence
.agents/skills/wiki-*/         Codex workflow skills
```

The initial root page is a one-time bootstrap task. Codex replaces it with a project-specific home after inspecting repository facts. Until then, `status` reports `bootstrapPending: true` and project `ready: false`.

Scaffold pages are never overwritten. The boundaries/SSOT, page template, and workflow pages are managed baselines that only `update --force` may refresh. Language migration only replaces unchanged packaged assets and fails before writes when user edits conflict.

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
