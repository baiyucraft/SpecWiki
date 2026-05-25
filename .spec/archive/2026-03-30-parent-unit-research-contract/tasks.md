## 1. Parent Research Contract

- [x] 1.1 调整 `crates/wiki-runtime/src/workflows/page_render.rs`，取消 `Overview / Architecture / DomainIndex` 的 seed-only research 短路
- [x] 1.2 调整高层 parent unit 的 research 输入组装，确保其真实产出 `UnitResearch`
- [x] 1.3 收紧 `crates/wiki-knowledge/src/compose.rs` 中高层页 compose 路径，要求高层 parent unit 必须持有 `unit_research`

## 2. Child Rollup Compose Input

- [x] 2.1 收口 parent compose 输入结构，统一消费 child digest、citation digest、diagram digest、key sources 与 readiness
- [x] 2.2 校正 `Overview / Architecture` 与 `DomainIndex / config-surface parent` 的逐层上卷边界，禁止越层直接抓 leaf 输入
- [x] 2.3 清理高层父页依赖 `PageDigest.summary` 或固定骨架直接拼页的弱路径

## 3. Runtime Readiness Summary

- [x] 3.1 扩展 runtime/cache 中 parent contract 的最小摘要，至少保留 `child_unit_ids / child_page_ids / child_digest_ids / citation_digest_refs / diagram_digest_refs / readiness_status`
- [x] 3.2 补齐高层 parent unit 缺少 `UnitResearch` 或 child rollup 未就绪时的 gate/readiness 诊断
- [x] 3.3 确认 `update / rebuild / reference` 相关读取面可以直接回溯 parent contract 摘要，而不是依赖最终 Markdown 反推

## 4. Tests And Verification

- [x] 4.1 为高层 parent unit 新增 Rust 测试，覆盖稳定 identity、独立 `UnitResearch`、逐层 child rollup 与 compose 前置条件
- [x] 4.2 为 runtime gate/readiness 新增 Rust 测试，覆盖高层 parent unit 缺少 research 或 child rollup 时的可诊断状态
- [x] 4.3 先运行 `storybook` 专项验证，确认高层 parent contract 断言成立
- [x] 4.4 记录本轮 `storybook` 专项结论，并明确 `dagger` 延后到后续 change 验证

## 5. Commenting And Change Hygiene

- [x] 5.1 检查本 change 涉及代码的注释是否符合 `COMMENTING.md`
- [x] 5.2 在实现前后核对 UniSpec 产物、测试范围与代码改动保持一致
