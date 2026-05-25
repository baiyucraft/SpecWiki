# Storybook Parent Contract Verification

- 变更：`parent-unit-research-contract`
- 样本：`storybook`
- 结论范围：本轮只验证 `storybook`；`dagger` 明确保留到后续 change，不作为当前前置门禁

## 执行记录

- 冷启动验证：`node scripts/run-test-projects.mjs --no-build --timeout-minutes 180 storybook`
  - 结果：`226` 页完成落盘，`runtime_state=completed`，`assembled_pages=226`
  - 用量：`total_tokens=723778`
- 修复复核：`cargo test -p wiki-runtime --lib`
  - 结果：`59 passed`
- warm 复核：`node scripts/run-test-projects.mjs --run-mode warm --timeout-minutes 60 storybook`
  - 结果：`226` 页完成落盘，`runtime_state=completed`，`assembled_pages=226`
  - 用量：`total_tokens=80318`

## 核心结论

- 高层 parent unit 已真实进入 `UnitResearch` 主线：
  - `Overview`：`unit-e50a8961f419`
  - `Architecture`：`unit-3ce08b39bd6b`
  - `DomainIndex`：`unit-13344a044882` / `unit-6733528f117a` / `unit-0478352352c4`
- 上述 5 个高层 parent unit 在 `research_cache` 中都存在 `research_type='unit'` 记录，且 `page_context_cache` 中都满足：
  - `has_unit_research_contract = true`
  - `unit_research_input_hash` 非空
  - `readiness_status = compose_ready`
  - `missing_child_unit_ids = []`
- direct-child rollup 边界成立，没有发现越层直抓 leaf：
  - `Architecture` 的 `child_unit_ids = ["unit-13344a044882","unit-6733528f117a","unit-0478352352c4"]`
  - `Overview` 的 `child_unit_ids = ["unit-3ce08b39bd6b","unit-13344a044882","unit-6733528f117a","unit-0478352352c4"]`
- runtime gate 正常：
  - 5 个高层 parent unit 都是 `research=ready / compose=done / assemble=done`
  - 全库 `blocked gate = 0`

## Parent Snapshot

| Unit | 页面 | child_unit_ids | child_digest_ids | citation_digest_refs | diagram_digest_refs | citation_count |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| `unit-e50a8961f419` | `项目概述.md` | 4 | 4 | 52 | 5 | 622 |
| `unit-3ce08b39bd6b` | `系统架构.md` | 3 | 3 | 39 | 3 | 595 |
| `unit-13344a044882` | `多框架支持/多框架支持.md` | 17 | 17 | 68 | 0 | 298 |
| `unit-6733528f117a` | `构建系统/构建系统.md` | 6 | 6 | 24 | 6 | 178 |
| `unit-0478352352c4` | `概念指南/概念指南.md` | 12 | 12 | 138 | 2 | 1174 |

## 关键观察

- `storybook` 的 parent contract 现在可以直接从 runtime/cache 回读，不需要依赖最终 Markdown 反推：
  - `page_context_cache` 已持久化 `child_unit_ids / child_page_ids / child_digest_ids / citation_digest_refs / diagram_digest_refs / readiness_status`
  - `unit_runtime_gates` 已持久化 `blocked_reason / missing_dependencies / ready stage`
- 在本轮 warm 复核前，`research_cache.input_hash` 已写入 SQLite，但 `UnitResearch.input_hash` 没有透传回内存对象，导致 `page_context_cache.unit_research_input_hash` 为空。
- 本轮已补齐该透传链路，并通过 `wiki-runtime` 单测与 warm `storybook` 复核确认高层 parent page 的 `unit_research_input_hash` 已非空。

## 最终判断

- `workflow-verification` 对 `storybook` 的当前门禁可以通过：
  - 高层 parent unit 存在自己的 `UnitResearch`
  - 高层父页消费的是 direct-child rollup，而不是越层 leaf
  - runtime/cache 可以直接回溯 parent contract 摘要
- 本报告不覆盖 `dagger`；`dagger` 仍留给后续 change 单独验证
