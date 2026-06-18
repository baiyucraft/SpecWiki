## Context

当前仓库里，knowledge-first update 主线实现已经单独提交；剩余需要收口的是两条更窄的 contract：

1. 高层 parent `KnowledgeUnit`
   - 现有 `knowledge-unit-decomposition` 主 spec 已经要求它们保持 child-backed 聚合身份
   - 但还没有把“后续 `UnitResearch` 与 child rollup 继续映射回同一稳定 parent identity”写得足够明确
2. lifecycle diagnostics
   - 当前脚本已经会把部分磁盘诊断快照提升成 `runtime_incomplete` 一类可消费状态
   - 但这层语义还缺正式 verification contract，容易再次退化成“脚本内部技巧”

这两个点都和 `update/status` 的 contract 可解释性有关，但都不足以再开启一轮 `wiki-index/query`、`steering/config` 或 `Agents/CLI` 级别的边界扩张。

## Goals / Non-Goals

**Goals:**

- 明确高层 parent `KnowledgeUnit` 需要保持 `UnitResearch` 与 child rollup 的稳定输入身份
- 明确 lifecycle harness 对诊断态的消费语义，避免把恢复态/缺失态误判成普通 `missing`
- 让本 change 保持 contract / verification follow-up 的窄边界

**Non-Goals:**

- 不在本 change 内扩张 `wiki-index/query` 合同
- 不在本 change 内修改 `steering/config` 路径、优先级或 init 行为
- 不在本 change 内引入 `Agents/CLI` forwarding 或宿主边界调整
- 不在本 change 内重做 `update` 主线实现

## Decisions

### 决策 1：高层 parent unit contract 需要显式覆盖 `UnitResearch` 输入身份

`knowledge-unit-decomposition` 已经说明高层 parent unit 不是页面语义空壳，而是显式 parent `KnowledgeUnit`。本 change 继续把这条 contract 收紧一层：

- parent unit 不仅要维护稳定 child 边界
- 还必须让后续 `UnitResearch` 与 child rollup 继续映射回这同一稳定身份

理由：

- 否则 parent unit 很容易再次滑回“聚合边界稳定，但 research 输入不稳定”的半收口状态
- 这会直接削弱后续增量诊断和 contract 复用

### 决策 2：lifecycle 诊断态必须以“可消费状态”进入验证

本 change 不去改 lifecycle 主流程，而是把现有验证语义正式化：

- 当磁盘侧只能恢复出缺失/不完整快照时
- harness 必须把它提升成可消费的 diagnostic runtime state
- 同时保留 `query_readiness / recommended_action` 这类字段

理由：

- 这能保证 `status/query/update` 的诊断语义对测试和宿主都是稳定可消费的
- 也能避免把“磁盘缺失但可诊断”误判成“什么都没有”

### 决策 3：本 change 只做 contract follow-up，不扩成基础设施 change

本 change 明确只覆盖：

- `knowledge-unit-decomposition`
- `workflow-verification`
- 对应的聚焦脚本测试

不覆盖：

- `wiki-index/query`
- `wiki-steering-config`
- `.wiki/06-设计文档/01-Runtime设计.md` 的 query 边界
- `.wiki/06-设计文档/02-Agents设计.md` / CLI forwarding

理由：

- 剩余脏改动已经跨了多个主题
- 如果这里不先把窄边界定住，后续提交会再次失去可审计性

## Risks / Trade-offs

- [高层 parent contract 继续停留在“半口头”状态] -> 用 delta spec 显式写出 `UnitResearch` 输入身份，减少再次漂移空间
- [diagnostic state 语义再次被脚本细节吞掉] -> 用 workflow-verification requirement 把它提升为正式验证对象
- [把无关主题继续塞进本 change] -> 在 proposal/design/tasks 里显式声明排除 `wiki-index/query`、`steering/config` 与 `Agents/CLI`

## Migration Plan

1. 先收紧 proposal 边界，确认本 change 是 contract follow-up
2. 增加 `knowledge-unit-decomposition` 与 `workflow-verification` 的 delta specs
3. 补一条 lifecycle diagnostic coercion 的脚本测试
4. 再决定剩余 `wiki-index/query`、`steering/config`、`Agents/CLI` 应分别去哪一个 follow-up change
