# SpecWiki Lite

SpecWiki Lite 是面向 Codex 的纯 TypeScript 仓库 Wiki 与分阶段文档工作流。稳定项目知识写入 `.wiki/`，change 证据写入 `.spec/`，八个仓库级工作流 Skill 写入 `.agents/skills/`。

## 安装与初始化

```bash
npm install -g spec-wiki-lite
spec-wiki-lite init --host codex
spec-wiki-lite status
```

运行环境为 Node.js `>=20.19.0`。初始版本是 `0.1.0`，不限制操作系统或 CPU 架构。

## 公开命令

```text
spec-wiki-lite init [path] [--host codex] [--force]
spec-wiki-lite status [--json]
spec-wiki-lite show <change-id> [--artifact <artifact>] [--json]
spec-wiki-lite validate <change-id> [--strict] [--json]
spec-wiki-lite update [--force] [--json]
spec-wiki-lite archive <change-id>
```

`init/update` 只同步受管基线和 Skills，用户页面与 scaffold 内容不会被覆盖。`status` 只检查 Wiki 静态结构、Skills 安装状态和 active changes，不扫描源码，也不建立持久化代码索引。

## 目录模型

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
.spec/changes/<change-id>/     active workflow 证据
.spec/archive/<date>-<id>/     已归档历史证据
.agents/skills/wiki-*/         Codex 工作流 Skills
```

初始 Wiki 页面只提供待填写结构，不虚构项目事实。所有 scaffold 页面只在缺失时创建，`update --force` 也不会覆盖；只有 managed convention 索引允许由 `--force` 刷新。

Wiki 使用 `INDEX.md` 组织导航；普通页面通过稳定相对链接连接，并包含 `title/description/updated/owner` YAML frontmatter。

## 开发验证

```bash
pnpm install
pnpm test
pnpm lint
pnpm build
pnpm run pack
```

发布制品只包含 Node CLI、`dist`、模板、Skills、README 与 LICENSE。本次 change 不执行 npm publish、打 tag 或合并 `lite` 回 `main`。
