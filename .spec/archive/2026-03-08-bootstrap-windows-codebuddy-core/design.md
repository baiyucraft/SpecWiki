## Context

当前仓库已经具备 `wiki-core` Rust binary、CodeBuddy Agent 调用层、基础扫描/生成/状态命令和 npm 打包链路，但第一阶段真正需要稳定的三件事还没有被系统化约束：

- 目标仓库 `.wiki/` 的运行时边界还不稳定，`wiki.metadata.json` 只是最小样例，尚未明确与参考样例的对齐范围。
- `init/status/update/query/sync/rebuild` 已有代码入口，但行为仍偏“最小可跑”，尚未形成可验证的阶段性契约。
- CodeBuddy Agent 已经能调用 core，但 Windows-only、thin Agent、binary 调用和错误透传这些约束还未沉淀成设计决策。

本 change 对应项目设计中的“迭代 1：Windows + CodeBuddy 跑通”，目标是先把 deterministic baseline 固化下来，为迭代 2 以后再进入更强的扫描、生成器、层级拆分和增量更新铺路。

约束条件：

- 当前只支持 Windows。
- 当前只支持 CodeBuddy Agent。
- 当前不引入 LLM、图生成、跨平台构建和复杂 DB cache。
- `wiki.metadata.json` 以 `tmp/reference-zh/meta/repowiki-metadata.json` 为主要参考，但允许第一阶段先落最小可验证字段集。

## Goals / Non-Goals

**Goals:**

- 定义第一阶段 Repo Wiki Runtime：`.wiki/*.md`、`wiki.metadata.json`、`.wiki/.cache/`
- 定义第一阶段六个工作流的最小行为和状态语义
- 明确 CodeBuddy Agent 与 `wiki-core` 的职责边界
- 让现有代码可以沿着统一 contract 收敛，而不是继续以样例逻辑生长

**Non-Goals:**

- 不在本 change 中实现完整生成器分层、层级化仓库理解或真正增量更新
- 不在本 change 中引入 LLM provider、Mermaid 图生成或 TOON 输出
- 不在本 change 中支持 Linux、macOS 或其他 IDE / CLI 宿主
- 不要求一次性完全复刻参考 metadata 的全部字段和嵌套结构

## Decisions

### 决策 1：第一阶段以 deterministic baseline 为主

第一阶段只要求稳定扫描、页面生成、metadata 写入、状态判断和 CodeBuddy 调用链，不要求 LLM 增强和复杂结构分析。

原因：

- 当前最紧迫的问题不是内容质量，而是 runtime 与 workflow 契约缺失。
- deterministic baseline 更容易测试、调试和稳定演进。
- 后续迭代仍可在不破坏 contract 的前提下替换生成器内部实现。

备选方案：

- 直接引入 LLM-assisted 生成：内容质量会更高，但会把第一阶段从“建立稳定 contract”变成“建立模型调用平台”，优先级错误。

### 决策 2：Runtime 采用“正式索引 + 文件缓存”分层

第一阶段 runtime 明确拆成三层：

- `.wiki/*.md`：正式 Wiki 页面
- `.wiki/wiki.metadata.json`：正式索引
- `.wiki/.cache/*.json`：运行时缓存

原因：

- 页面、索引、缓存职责不同，混在一起会让 `status/update/sync` 难以演进。
- 当前先使用 JSON 文件缓存，便于调试和版本演进。
- 正式索引和缓存分离后，未来即使替换 cache 实现，也不会破坏外部格式。

备选方案：

- 直接让 metadata 兼做 cache：实现最省事，但后续脏状态、上下文缓存、query cache 都会把 metadata 拖成不稳定的大文件。
- 第一阶段就引入 DB：复杂度过高，不符合当前阶段目标。

### 决策 3：`wiki.metadata.json` 先对齐最小必需字段，再逐步扩展

第一阶段要求 metadata 至少稳定承载：

- schema/version/language
- repo root / branch / generated time / commit
- wiki items
- relations
- source files
- dirty state

并要求字段命名和职责尽量贴近参考样例。

原因：

- 当前代码已经有最小 metadata 结构，直接完全重做成本高。
- 第一阶段更重要的是让字段语义稳定，保证 `status/update/query/sync` 可以依赖它。
- 后续迭代可在不破坏现有最小字段的前提下继续扩展更完整结构。

备选方案：

- 立即完全复刻参考样例：会引入大量当前代码尚未使用的字段，增加实现噪音。

### 决策 4：第一阶段 `update` 允许使用全量刷新策略

第一阶段 `update` 的 contract 只要求：

- 检测到 runtime 过期
- 刷新页面、metadata 和 cache
- 最终使 runtime 回到 fresh 状态

内部实现允许先采用“判定 stale 后执行全量刷新”的策略，不要求真正增量更新。

原因：

- 项目设计中真正增量更新明确属于后续迭代。
- 先把 `update` 的入口与状态契约定住，比过早引入脏图回溯更重要。

备选方案：

- 在本 change 中直接实现 source-to-page 增量更新：会把范围拉到迭代 5。

### 决策 5：CodeBuddy Agent 保持 thin Agent

第一阶段 CodeBuddy Agent 只负责：

- 暴露工具
- 解析当前 Windows binary
- 调用 `wiki-core`
- 透传结果和错误

不在 Agent 层承载 Wiki 业务逻辑、metadata 解析或页面生成规则。

原因：

- `wiki-core` 才是长期核心资产。
- 将业务逻辑留在 Agent 层会阻碍未来 CLI / 其他宿主复用。
- 当前用户已经明确要求后续统一命名为 Agents，thin Agent 更符合这一方向。

备选方案：

- 在 TS 层补更多业务兜底：短期看更快，但会形成 core 与 Agent 双重规则。

## Risks / Trade-offs

- [第一阶段 metadata 只做最小对齐] → 在 design 和 specs 中明确最小字段集，后续迭代按兼容方式扩展，不在第一阶段追求完全等同参考样例。
- [`update` 先做全量刷新，性能有限] → 在 workflow 规范中仅约束结果语义，把真正增量更新留到后续迭代。
- [thin Agent 让部分错误直接暴露给宿主] → 统一 core 返回结构和错误语义，让 Agent 只做最小包装。
- [Windows-only 设计可能诱导平台分支写死] → 在 design 中明确当前是阶段性限制，不把平台常量扩散进 core domain model。

## Migration Plan

1. 先以本 change 的 specs 固化第一阶段 runtime、workflow 和 Agent 契约。
2. 根据 specs 调整 `wiki-core` 的 metadata、storage、app workflow 与 CodeBuddy Agent 调用层。
3. 用 Windows 下的 e2e 与 Agent 测试验证 `init/status/update/query/sync/rebuild` 最小闭环。
4. 保持现有对外命令名不变，避免对已经存在的脚本和打包链路造成额外迁移成本。

本 change 不涉及线上迁移；回滚方式为回退本次 change 对 Rust core、CodeBuddy Agent 和 runtime contract 的实现。

## Open Questions

- 第一阶段 metadata 与参考样例的“最小必需字段集”最终以哪些字段为准，需要在实现前结合现有代码再做一次收敛。
- `query` 第一阶段是否只返回页级命中，还是同时返回 source/relations 的最小结构，需要在任务拆解时明确。
