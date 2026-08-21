# SpecWiki Lite

SpecWiki Lite is a TypeScript-only repository Wiki and staged documentation workflow for Codex. It keeps durable project knowledge in `.wiki/`, change evidence in `.spec/`, and eight repo-local workflow skills in `.agents/skills/`.

## Install

```bash
npm install -g spec-wiki-lite
spec-wiki-lite init --host codex
spec-wiki-lite status
```

Initialization uses Chinese Wiki and Skill content by default. Use `spec-wiki-lite init --host codex --language en` for English. The stable `wiki-*` Skill ids, reference filenames, CLI, and `.spec` machine fields remain English in either locale.

Requires Node.js `>=20.19.0`. The initial package version is `0.1.0` and has no OS or CPU restriction.

## Commands

```text
spec-wiki-lite init [path] [--host codex] [--language zh|en] [--force] [--no-codegraph] [--json]
spec-wiki-lite status [--json]
spec-wiki-lite show <change-id> [--artifact <artifact>] [--json]
spec-wiki-lite validate <change-id> [--strict] [--json]
spec-wiki-lite update [--force] [--json]
spec-wiki-lite archive <change-id>
```

`init` writes `.wiki/config.yaml`, synchronizes the localized Wiki and Skill/reference tree, and by default prepares external CodeGraph: it installs the global CLI when missing, configures the Codex user-level MCP entry, and runs project-level `codegraph init`. Failures are reported as warnings and do not block Wiki/.spec initialization; use `--no-codegraph` for offline or restricted environments. `init --json` exposes each stage result. `update` reads `wiki.language` from config; changing it switches both Wiki baselines and Skill content. Scaffold pages and user pages are preserved. Registered Skill files are package-owned and repaired on every update, while unregistered user additions remain untouched. `status` only reports whether `.codegraph` exists; it never installs, configures MCP, or rebuilds an index.

CodeGraph is an external optional read-only analysis tool, not a Lite runtime dependency. Lite does not maintain a second code index, knowledge graph, or database; local `.codegraph` data is ignored and excluded from tarballs.

### Implementation mode

`wiki-plan` asks once whether the change should use `tdd` or `direct` unless the current request already specifies it. The choice is recorded as `implementation-mode` in `tasks.md`. `tdd` uses Red → Green → Refactor; `direct` uses Implement → Verify → Refactor. A complete tasks artifact with a valid mode routes directly to `wiki-apply`, while authorization, strict validation, review, verification, and archive gates remain required.

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
.agents/skills/wiki-*/
├── SKILL.md                   localized Codex workflow skill
└── references/**              localized artifact templates and review standards
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

The distribution contains only the Node CLI, `dist`, bilingual Wiki/Skill templates, README, and LICENSE. Each locale ships 8 Skills and 16 registered references (24 Skill files). This repository does not publish from the `lite` branch as part of the current change.

Chinese documentation: [README-CN.md](./README-CN.md).
