# integrate-codegraph-into-wiki-skills

## 问题

SpecWiki Lite 的 Wiki Skills 目前只提示普通源码读取；当前项目虽可单独使用 CodeGraph，但新项目初始化不会自动准备它，导致 Skills 无法稳定利用代码图谱，也没有统一的失败降级状态。

## 目标

- `spec-wiki-lite init` 默认安装/准备 CodeGraph、配置 Codex MCP 并初始化当前项目索引。
- 增加 `--no-codegraph` 和 `init --json`，支持离线、CI、受限权限和机器读取。
- CodeGraph 任一步失败不阻断 Wiki/.spec/Skills 初始化，结果中报告 warning。
- 中英文 8 个 `wiki-*` Skill 写入 CodeGraph 优先、CLI fallback 和普通源码读取 fallback 规则。
- status 报告项目是否存在 CodeGraph 索引，但不触发副作用。

## 非目标

- 不把 CodeGraph 加入 SpecWiki Lite runtime dependencies。
- 不复制 CodeGraph 源码、数据库、daemon 或 MCP runtime 到发布包。
- 不让 Lite 建立第二套代码索引、知识图谱或 Wiki 中间层。
- 不修改历史 `.spec/archive/**`。

## 成功标准

- init 成功时返回稳定 CodeGraph 结果：requested、CLI version/installed、Codex MCP configured、project initialized、warnings。
- 缺失 CLI、安装失败、MCP 配置失败、项目索引失败均可测试且不阻断核心 Wiki 初始化。
- `--no-codegraph` 不调用外部 CodeGraph/npm 命令。
- `status --json` 报告 CodeGraph 只读状态；ready 仅由 Lite 核心状态决定。
- 中英文 8 个 Skill 均包含 CodeGraph 工具选择和 fallback 规则，且 repo-local 与 package assets 同步。
- tarball 不包含 `.codegraph` 数据库或 CodeGraph runtime。

## 影响范围

- `packages/spec-wiki-lite/src/orchestration/codegraph/**`
- `packages/spec-wiki-lite/src/orchestration/init/runInit.ts`、`src/cli.ts`、`src/core/status.ts`
- `packages/spec-wiki-lite/assets/skills/{zh,en}/wiki-*/SKILL.md`
- README/Wiki、package/root tests、tarball smoke

## 参考资料

- source：`E:/project/!byAI/spec-wiki/.upstream/codegraph` 与 `https://github.com/colbymchenry/codegraph`
- target：Lite init orchestration、CLI/status、Codex Skills 和测试
- adoption：改写 CodeGraph 的安装/初始化/提示规则；不迁移源码或 runtime。
