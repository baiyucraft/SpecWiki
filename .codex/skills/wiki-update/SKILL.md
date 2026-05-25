---
name: wiki-update
description: "当仓库已经初始化过 wiki，且源码变更后需要刷新当前 runtime 状态时使用。"
user-invocable: true
---

# wiki-update

## 何时使用
- 源码发生变化，当前 wiki runtime 可能已经过期时使用。
- 仓库已经完成过初始化，需要执行一次刷新或更新时使用。
- 用户明确要求 update、refresh、sync 当前 wiki 状态时使用。

## 执行方式
- 在仓库根目录运行 `spec-wiki wiki update`。
- 把 CLI 作为唯一正式入口，不要在宿主层重写 Wiki 更新逻辑。
- 如实传达 runtime 返回的最终结果或错误，不要改写业务语义。

## 结果解释
- 先说明这次 update 的最终状态：完成、部分完成、跳过或失败。
- 如果 runtime 输出里能看出具体刷新了哪一层，就按实际情况说明，例如：
  - facts 或 index
  - knowledge planning、research、compose
  - page assembly
  - metadata 或 cache
- 如果 runtime 只保证了部分链路刷新，就明确说出边界，不要暗示已经完成全量 knowledge 或 pages 更新。
- `v0.2.0` 的正式成功语义是最小 knowledge runtime 已刷新完成；`runtime_incomplete` 或 blocker 不能包装成成功。

## 后续动作
- 如果 update 成功，简要说明新的 runtime 状态。
- 如果需要进一步确认状态，建议运行 `spec-wiki wiki status`。
- 如果需要继续定位代码或结构，建议运行 `spec-wiki wiki query`。
- 如果仓库还没有完成初始化，明确指向正确的 init 或 bootstrap 流程，不要自行发明 fallback。

## 约束
- 这是一个 CLI 入口 skill，不是手工维护 Wiki 文档的操作手册。
- 不要硬编码样本仓库结构、包到页面的映射、页面 taxonomy 或固定章节树。
- 不要手动编辑 `.wiki/` 来冒充 update 成功。
- 除非 runtime 输出明确支持，否则不要声称 declared knowledge、derived knowledge 或 pages 已刷新完成。
- 不要在宿主层重建一套并行的 Wiki 状态机或业务逻辑。
- 如果 CLI 或 runtime 返回错误、blocked 状态或不支持当前状态，直接准确透传，不要静默改写状态。
