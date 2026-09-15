# SpecWiki Lite

SpecWiki Lite 是面向 Codex 的纯 TypeScript 仓库 Wiki 与分阶段文档工作流。稳定项目知识写入 `.wiki/`，change 证据写入 `.spec/`，八个仓库级工作流 Skill 写入 `.agents/skills/`。

## 安装与初始化

```bash
npm install -g spec-wiki-lite
spec-wiki-lite init --host codex
spec-wiki-lite status
```

默认生成中文 Wiki 与 Skill 正文。需要英文时使用 `spec-wiki-lite init --host codex --language en`。无论语言如何，`wiki-*` Skill 标识、reference 文件名、CLI 和 `.spec` 机器字段均保持英文稳定。

运行环境为 Node.js `>=20.19.0`。当前版本是 `0.2.0`，不限制操作系统或 CPU 架构。

## 公开命令

```text
spec-wiki-lite init [path] [--host codex] [--language zh|en] [--force] [--no-codegraph] [--no-aoci] [--json]
spec-wiki-lite status [--json]
spec-wiki-lite show <change-id> [--artifact <artifact>] [--json]
spec-wiki-lite validate <change-id> [--strict] [--json]
spec-wiki-lite update [--force] [--tools] [--json]
spec-wiki-lite archive <change-id>
```

`init` 写入 `.wiki/config.yaml`，同步所选语言的 Wiki/Skill 资产，并默认准备固定版本的 CodeGraph `1.6.0` 与 AOCI-CODE `0.1.0-rc12`。CodeGraph 负责当前符号、调用链、影响范围和受影响测试；AOCI 负责长期系统语义、职责、约束、Code Cognition，以及声明数据库 source 后的 Database Cognition。`--no-codegraph` / `--no-aoci` 只延后当前一次初始化，核心资产仍成功，但返回 exit `2` 且项目保持 not ready。`update --tools` 才修复或升级外部工具；普通 `update` 只同步资产。`status` 使用官方只读 JSON/Guide 门禁，不安装、不配置、不索引。

两个工具都不属于 Lite runtime dependency，二进制、数据库、daemon、socket、日志和本机状态均不进入 tarball。Lite 不复制 AOCI 状态机，不维护第二套代码索引或知识图谱；AOCI 数据库能力只读取 Schema 元数据，不读取业务行，也不输出凭据。

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

发布制品只包含 Node CLI、`dist`、双语 Wiki/Skill 模板、README 与 LICENSE。每种语言包含 8 个 Skill 与 20 个登记 references，共 28 个 Skill 文件。本次 change 不执行 npm publish、打 tag或合并 `lite` 回 `main`。
