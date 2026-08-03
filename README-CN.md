# SpecWiki Lite

SpecWiki Lite 是面向 Codex 的纯 TypeScript 仓库 Wiki 与分阶段文档工作流。稳定项目知识写入 `.wiki/`，change 证据写入 `.spec/`，八个仓库级工作流 Skill 写入 `.agents/skills/`。

## 安装与初始化

```bash
npm install -g spec-wiki-lite
spec-wiki-lite init --host codex
spec-wiki-lite status
```

默认生成中文 Wiki 与 Skill 正文。需要英文时使用 `spec-wiki-lite init --host codex --language en`。无论语言如何，`wiki-*` Skill 标识、reference 文件名、CLI 和 `.spec` 机器字段均保持英文稳定。

运行环境为 Node.js `>=20.19.0`。初始版本是 `0.1.0`，不限制操作系统或 CPU 架构。

## 公开命令

```text
spec-wiki-lite init [path] [--host codex] [--language zh|en] [--force]
spec-wiki-lite status [--json]
spec-wiki-lite show <change-id> [--artifact <artifact>] [--json]
spec-wiki-lite validate <change-id> [--strict] [--json]
spec-wiki-lite update [--force] [--json]
spec-wiki-lite archive <change-id>
```

`init` 写入 `.wiki/config.yaml`，同步所选语言的 Wiki 基线以及完整 Skill/reference 文件树；`update` 从配置的 `wiki.language` 读取语言，配置变化会同时切换 Wiki 基线和 Skill 正文。用户页面与 scaffold 内容不会被覆盖；登记 Skill 文件每次 update 都恢复为 package 版本，Skill 目录中用户新增的未登记文件保留。`status` 逐文件比对目标语言的全部登记 Skill 资产，并检查 Wiki、bootstrap 和 active changes，不扫描源码，也不建立持久化代码索引。

## 目录模型

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
.spec/changes/<change-id>/     active workflow 证据
.spec/archive/<date>-<id>/     已归档历史证据
.agents/skills/wiki-*/
├── SKILL.md                   本地化 Codex 工作流 Skill
└── references/**              本地化 artifact 模板与 review standards
```

初始根页是一次性 Wiki 初始化任务。Codex 按需读取仓库事实并将其替换为正式首页；完成前 `status` 报告 `bootstrapPending: true`，项目 `ready: false`。

scaffold 页面永不覆盖；边界与 SSOT、页面模板和 Lite 工作流是 managed 基线，只有 `update --force` 可刷新。语言迁移只替换仍等于内置模板的登记资产，发现用户修改时在写入前失败。

Wiki 使用 `INDEX.md` 组织导航；普通页面通过稳定相对链接连接，并包含 `title/description/updated/owner` YAML frontmatter。

## 开发验证

```bash
pnpm install
pnpm test
pnpm lint
pnpm build
pnpm run pack
```

发布制品只包含 Node CLI、`dist`、双语 Wiki/Skill 模板、README 与 LICENSE。每种语言包含 8 个 Skill 与 16 个登记 references，共 24 个 Skill 文件。本次 change 不执行 npm publish、打 tag或合并 `lite` 回 `main`。
