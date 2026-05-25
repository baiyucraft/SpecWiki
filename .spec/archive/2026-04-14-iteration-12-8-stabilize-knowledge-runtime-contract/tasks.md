## 1. Contract 收敛

- [x] 1.1 为 `KnowledgeUnit` 收敛最小正式合同，明确 identity、status、source/citation、declared/derived/projection 引用与失效原因字段。
- [x] 1.2 为 `.wiki/.knowledge/declared/**` 设计并落地最小 declared record artifact 与 writeback 约束。
- [x] 1.3 为 knowledge runtime 落地最小 health signal 对象与 `status` 可消费的聚合摘要。

## 2. Lifecycle 与 Workflow 语义

- [x] 2.1 收敛 `declared -> derived -> projection -> cache` 的状态流转与失效传播规则，并把该规则接入 `update` 的 `AffectedKnowledgeScope`。
- [x] 2.2 收敛 `research / compose` 的最小正式输入输出，使 unit research summary 与 projection digest 成为正式 contract。
- [x] 2.3 收敛 `sync` 的结果分类，明确 `declared_writeback`、`metadata_only` 与 `illegal_drift` 的判定和推荐动作。
- [x] 2.4 收敛 `status / query / rebuild` 对 knowledge health 与 writeback 结果的对外语义，避免 readiness 与 provenance 混层。

## 3. 存储与实现落地

- [x] 3.1 在 `wiki-model / wiki-knowledge / wiki-runtime` 中补齐 declared artifact、health signal、unit contract 与 workflow DTO 的模型定义。
- [x] 3.2 在 formal artifact 写盘与 restore 主链中补齐 declared/health artifact 的读写、一致性与恢复逻辑。
- [x] 3.3 在 `sync`、`update` 与 `status` 的 runtime 编排里接入新的 contract 校验、状态传播与推荐动作生成。

## 4. 测试与验收

- [x] 4.1 为 declared lifecycle、health signals、state transition、sync writeback 结果分类补齐对应层级的自动化测试。
- [x] 4.1a 补一组 formal artifact 写盘/恢复 roundtrip 测试，覆盖 declared artifact 与 health signal artifact 的落盘和 restore。
- [x] 4.1b 补一组 sync 分类矩阵测试，覆盖 `declared_writeback`、`metadata_only` 与 `illegal_drift`，并验证 `illegal_drift > declared_writeback > metadata_only` 的优先级。
- [x] 4.2 运行 `node scripts/run-test-projects.mjs storybook dagger`，验证 knowledge runtime contract 收敛不会退化为样本特化逻辑。
- [x] 4.3 视实现影响范围决定是否补跑全量 `node scripts/run-test-projects.mjs`，并将差异分析沉淀到 change 报告。
- [x] 4.4 单独执行一次 [COMMENTING.md](/E:/project/!byAI/spec-wiki/COMMENTING.md) 合规检查，确认新增或修改注释符合仓库规范。

## 5. 文档与发布面收口

- [x] 5.1 更新相关设计文档、README 或 runtime contract 说明，使其不再把当前实现误写成“完善 knowledge system”。
- [x] 5.2 在 change 报告中明确本轮只收稳 `minimal formal knowledge runtime` 的 contract，不把未完成能力伪装成正式承诺。
