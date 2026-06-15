## Why

当前 `v0.1.0` 已经把公开范围收敛到 `index + codebuddy/codex/claude`，但 runtime、CLI、宿主 skill 和发布产物之间仍存在几处明显的 contract 漂移：`init/update/status/query` 对 `missing` 的解释不闭合，`wiki-query/wiki-update` 的 description 大于真实能力边界，CLI 与 core 的错误/bridge/query 语义也没有完全对齐。继续在这个状态上叠功能，只会放大宿主误触发和用户预期偏差。

这轮不扩能力上限，只把 `v0.1.0` 已经正式承诺的边界收硬：把 index-only runtime 的外部状态说清楚，把 query 的 trust/exit contract 收紧，把宿主技能文案与发布产物统一到单一真相源，并把暂不实现的 intent-aware query 明确转入设计文档。

## What Changes

- 为 `v0.1.0` 增补显式的 `index_only` 外部状态与 release-scope 投影，避免 `init` 已完成但仍返回 `missing`
- 收紧 `status/query` 的 preflight 与 query trust 语义，使其更贴近“本次结果是否可消费”
- 明确 `update` 在 `v0.1.0` 下是“重做当前 index-only runtime”的公开语义，不再把它写成完整 runtime refresh
- 收紧 `wiki-query` / `wiki-update` skill description，只描述当前稳定公开能力，不再提前承诺 `owner / entrypoint / impact`
- 删除发布包中的占位 skill 模板，统一宿主 skill 的真相源
- 修正 CLI/runtime forwarding 的 contract：失败退出码与 `--bridge-stdio` 暴露边界
- 把 intent-aware query、owner/impact/entrypoint 稳定 schema、宿主 trigger 语料测试体系写入 `.wiki/06-设计文档/01-Runtime设计.md` 与 `.wiki/06-设计文档/02-Agents设计.md`

## Capabilities

### New Capabilities

- `global-cli-bootstrap`: 定义 `spec-wiki` 全局 CLI 与三类宿主 bootstrap 资产在 `v0.1.0` 下的公开入口、description 与发布边界

### Modified Capabilities

- `repo-wiki-runtime`: 收紧 index-only runtime 的外部状态、query trust 与短流程 query contract
- `repo-wiki-workflow`: 为 `v0.1.0` 的 `init/update` 增补 release-scope 语义，不再把 index-only 终态伪装成 `missing`
- `adapter-distribution`: 发布产物不再携带与真实 bootstrap 逻辑漂移的占位 skill 模板

## Impact

- Rust: `crates/wiki-runtime/src/workflows/{init,status,query,update,release_scope}.rs`、相关 transport 与 runtime_profile
- TypeScript: `packages/spec-wiki/src/cli.ts`、`packages/spec-wiki/src/runtime/{forwardCore,invokeCore}.ts`、`packages/spec-wiki/src/agents/shared/commandAssets.ts`
- 发布与测试：`packages/spec-wiki/package.json`、`scripts/build-dist.mjs`、`scripts/tests/distribution.test.ts`、runtime/CLI/skill 相关测试
- 设计文档：`.wiki/06-设计文档/01-Runtime设计.md`、`.wiki/06-设计文档/02-Agents设计.md`
