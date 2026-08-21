# integrate-codegraph-into-wiki-skills 设计

## 方案概述

增加独立的 CodeGraph orchestration，隔离外部命令、副作用和错误捕获。Lite 先执行现有 Wiki/.spec/Skills asset sync，再执行 CodeGraph 集成；CodeGraph 失败只进入 warning，不改变 Lite 核心 ready 判定。

## 接口与数据结构

`runBootstrapInit` 增加 `codegraph?: boolean` 选项，默认 true；CLI 增加 `--no-codegraph`、`--json`（init 允许 JSON 输出）。

CodeGraph 结果固定为：

- `requested: boolean`
- `cli: { available: boolean; version?: string; installed: boolean }`
- `codexMcp: { configured: boolean }`
- `project: { initialized: boolean; path: ".codegraph" }`
- `warnings: Array<{ stage: "install" | "mcp" | "project"; message: string; recovery: string }>`

外部命令通过 `spawn` 参数数组执行：`codegraph --version`、`npm install -g @colbymchenry/codegraph@latest`、`codegraph install --target=codex --location=global --yes --no-permissions`、`codegraph init <repoRoot>`。不拼接 shell 命令，不把路径交给 shell 解析。

`ProjectStatusReport` 增加只读 `codegraph: { initialized: boolean; path: ".codegraph" }`。status 只检查路径，不安装、配置或索引。

## 失败与回滚

- Wiki asset sync 失败：保持现有 init failed 行为，不执行 CodeGraph。
- CodeGraph install/mcp/project 任一步失败：记录 warning，继续后续步骤和返回 `outcome: ready`。
- `--no-codegraph`：返回 requested false、空 warning，不执行任何外部命令。
- 不删除用户已有 `.codegraph`；项目索引失败不回滚已有 CodeGraph 数据库。
- 外部命令使用可注入 runner，单元测试不访问真实 npm、Codex 配置或网络。

## Skill 合同

8 个中英文 Skill 都加入统一提示：`.codegraph` 存在时优先 CodeGraph；MCP 优先 `codegraph_context/explore/search/callers/callees/impact/affected/status`；CLI 可用时使用对应命令；无索引/失败时回退源码读取和项目测试；CodeGraph 不替代安全、测试、review 或 Lite 的 `.spec` 权威。

阶段提示按 proposal 中的分配落入对应 Skill；continue/archive 只在需要代码影响或沉淀核对时使用。

## Ownership 与发布边界

- CodeGraph 不是 package dependency；发布包只含 Lite 的 Node CLI、assets、dist、README、LICENSE。
- `.codegraph` 由 CodeGraph 自己创建并通过其 `.gitignore` 排除数据库；不注册为 Lite asset。
- `.spec/archive/**` 不改写。
