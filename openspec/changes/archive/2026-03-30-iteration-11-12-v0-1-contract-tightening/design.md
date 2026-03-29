## Context

当前仓库已经在实现上把 `v0.1.0` 收敛为“facts/index 驱动的结构化 query + 宿主薄消费”，但对外 contract 仍带着完整 runtime 时期的影子：

- `SPEC_WIKI_V0_1_INDEX_ONLY=1` 会让 `init/update` 在 facts snapshot 后短路，但 `InitReport.state` 和 `StatusReport.state` 仍使用 `missing`
- `query_trust` 只由 runtime preflight 推导，无法表达“虽然 runtime 不完整，但本次 query 已返回可消费结果”
- 宿主 skill 的 `description` 仍把 `owner / entrypoint / impact` 写成已正式开放的能力
- 发布包里还带着未被 bootstrap 使用的占位 skill 模板，形成“两套真相源”

设计边界仍以 [DESIGN-3.0.md](E:/project/!byAI/spec-wiki/DESIGN-3.0.md) 和 [DESIGN-RUNTIME.md](E:/project/!byAI/spec-wiki/DESIGN-RUNTIME.md) 为准：`Facts -> Knowledge Planning -> Research -> Compose -> Assemble` 是完整主链，但 `v0.1.0` 只正式承诺 facts/index 与宿主薄消费。

参考源码结论：

- `GitNexus` 的查询能力值得借鉴的重点在厚索引与 intent-aware query substrate，而不是当前版本就把宿主 description 写成已支持 `entrypoint/impact`。源码可见 [pipeline.ts](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/pipeline.ts) 和 [symbol-table.ts](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/symbol-table.ts)。
- `deepwiki-open` 更适合作为 query/session 消费层参考，不适合反推当前 `v0.1.0` core contract。

## Goals / Non-Goals

**Goals:**

- 让 `v0.1.0` 的外部状态机对宿主可解释且自洽
- 让 query 的 trust、CLI 错误码与宿主 skill description 回到当前真实能力边界
- 删除发布与 bootstrap 之间的漂移真相源
- 把尚未正式开放的 intent-aware query 能力明确转入设计文档，而不是继续藏在宿主文案里

**Non-Goals:**

- 不在本轮把 `query(term)` 升级成正式的 intent-aware external payload
- 不在本轮补完整 knowledge/page runtime
- 不在本轮改变 `wiki-index` 已有内部 intent 枚举或新增新的宿主工具
- 不在本轮回收所有历史实现与所有旧测试，只处理本轮 contract 直接相关的路径

## Decisions

### 1. 用显式 `index_only` 外部状态替代“facts 已就绪但仍报告 missing”

把 `missing` 重新收回为“尚未初始化 / facts snapshot 不可用”的语义。`v0.1.0` 的 index-only 短路路径改为显式返回 `index_only`，并通过 runtime metadata 持久化 release scope，让 `status/query/init/update` 在 facts snapshot 已完成但 page/runtime 未生成时返回统一状态。

这样做的原因：

- 宿主不再需要用 `state + facts_ready` 猜真实阶段
- `init` 完成后返回 `missing` 的自相矛盾可以直接消除
- 这个改动只收紧对外语义，不要求本轮补 page/runtime

备选方案：

- 保留 `missing`，只靠 `query_readiness` 区分。问题是宿主和测试会持续出现“missing 但 ready”的双重语义。
- 直接把 index-only 也叫 `fresh`。问题是这会把完整 runtime 的 `fresh` 与 facts-only 的 `fresh` 混成一类，误导更大。

### 2. `query_trust` 必须至少反映“本次有没有可消费结果”

保留现有 `ready / stale_but_queryable / blocked` 三值，但不再机械等于 runtime preflight。若 runtime preflight 是 `blocked`，但本次 query 已返回 page fallback 或 mixed 结果，则将 trust 下调到 `stale_but_queryable`，同时保留 `recommended_action` 提示后续修复动作。

这样做的原因：

- 宿主可以展示结果，同时明确结果处于降级路径
- 不需要在本轮引入更大的 trust taxonomy

备选方案：

- 新增 `partial` 或 `fallback_only`。更细，但会扩大对外协议面。
- 保持当前实现。这样继续存在“有结果但 blocked”的不可解释组合。

### 3. `update` 本轮不强行改成非破坏式，但必须改实公开语义

当前 `SPEC_WIKI_V0_1_INDEX_ONLY` 路径下的 `update` 实际是“重做当前 index-only runtime”，而不是完整 runtime refresh。本轮先收口文案、skill 和测试契约；不承诺把行为改成完整增量刷新。

这样做的原因：

- 直接改行为会碰到完整 runtime 和 facts-only runtime 的残留清理边界，风险更高
- 当前阶段更需要先把 contract 写实，避免宿主继续误导用户

备选方案：

- 本轮直接实现“保留 page/runtime，仅刷新 facts/index”。这会留下“旧页面仍在但不再受正式承诺”的更大歧义。

### 4. CLI passthrough 需要最小协议感知来给出正确退出码

`spec-wiki wiki <action>` 继续原样透传 stdout/stderr，但 CLI 需要在不改写 payload 的前提下识别短流程 JSON 响应和长流程 terminal event 的 `ok=false` 终态，并将退出码映射为非零。这里的协议感知只服务 shell/CI contract，不负责在 CLI 层重组业务数据。

这样做的原因：

- 当前 shell/CI 只能看到子进程退出码，`ok=false` 但 exit 0 会把逻辑失败伪装成成功
- `invokeCore` 仍然保留结构化 response contract；CLI 只是补齐 Unix 语义

### 5. 宿主 skill 与发布产物必须只有一个真相源

保留 `commandAssets.ts` 作为实际 skill 文案真相源；发布包不再携带未被 bootstrap 使用的占位 skill 模板。CodeBuddy hook 的附加上下文和触发词也同步收紧到当前真实 query 边界。

这样做的原因：

- `skill-creator` 只看 `name + description` 触发技能，漂移模板会直接误导未来分发与复用
- 目前发布包里的 `__DESCRIPTION__` 占位稿已经不是“低优先素材”，而是错误合同

## Risks / Trade-offs

- [已有测试和脚本依赖 `missing + facts_ready`] → 通过统一改成 `index_only` 并同步更新测试，避免继续累积双语义
- [`update` 行为仍可能删除既有 page/runtime] → 本轮明确把这件事写进 action note / skill / tests，不再伪装为完整 refresh
- [CLI passthrough 开始解析协议终态来确定退出码] → 保持 stdout 原样透传，只做最小解析，不在 CLI 层重写业务数据
- [删除发布占位模板后 staging 产物结构变化] → 同步调整 `package.json` 与 distribution tests，确保发布包只包含真实会被消费的文件

## Migration Plan

1. 先补 OpenSpec delta，明确 `index_only`、query trust 和宿主 asset 边界
2. 实现 runtime / CLI / skill / distribution 改动
3. 更新相关测试与 bootstrap 断言
4. 在 `DESIGN-RUNTIME.md` / `DESIGN-AGENTS.md` 记录 deferred 项：intent-aware query、稳定 owner/impact schema、trigger 语料测试体系

## Open Questions

- 本轮先保留 `query(term)` 外部入口，后续 intent-aware query 是扩展现有 action 还是新增 structured payload
- `index_only` 是否只作为 `v0.1.0` 过渡状态存在，还是未来继续保留为长期 release scope
