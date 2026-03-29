## 1. OpenSpec

- [x] 1.1 为 `repo-wiki-runtime`、`repo-wiki-workflow`、`adapter-distribution` 和 `global-cli-bootstrap` 补充 `v0.1.0` contract 收口要求
- [x] 1.2 更新 `DESIGN-RUNTIME.md` 与 `DESIGN-AGENTS.md`，把 intent-aware query、稳定 owner/impact schema 与 trigger 测试体系转入后续设计

## 2. Runtime / CLI / Skill

- [x] 2.1 为 index-only release scope 引入显式 `index_only` 外部状态，并统一 `init/status/query/update` 的状态投影
- [x] 2.2 收紧 query contract：修正 query trust 与结果可用性的关系
- [x] 2.3 修正 CLI forwarding contract：失败退出码与 `--bridge-stdio` 暴露边界
- [x] 2.4 收紧宿主 skill description、CodeBuddy hook 触发词与共享语义说明
- [x] 2.5 删除发布包里的占位 skill 模板，并同步更新分发脚本与断言

## 3. 测试与注释

- [x] 3.1 补充并更新 runtime/CLI/skill/distribution 测试，覆盖 `index_only` 状态、query trust、错误退出码、bridge 参数与 hook 触发边界
- [x] 3.2 检查本轮新增和修改注释，确保符合 `COMMENTING.md`
