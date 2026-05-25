# Storybook + Dagger Runtime Gate Analysis

生成时间：2026-03-20

本报告记录 9.7 最终一轮 `storybook + dagger` 专项中的真实 runtime gate 观察。重点是把两类结论拆开归档：

- 长窗口 `init` 是否已经能把 runtime 主链完整跑通
- 5 分钟 `lifecycle` 冷启动预算是否仍然失败

## 观测窗口

| 命令 | 预算 | storybook | dagger | 结论 |
| --- | --- | --- | --- | --- |
| `node scripts/run-test-projects.mjs --jobs 1 --timeout-minutes 45 storybook dagger` | 单项目 45 分钟 | `runtime_incomplete`，`knowledge_units=195 / unit_research=142 / page_drafts=0 / wiki_pages=0 / runtime_state=researching` | `ready`，`knowledge_units=64 / unit_research=64 / page_drafts=64 / wiki_pages=64 / markdown=64` | 证明 dagger 已跑通正式 runtime；storybook 已从 scan 阻断推进到 research 深水区，但 45 分钟仍不够。 |
| `node scripts/run-test-projects.mjs --jobs 1 --timeout-minutes 75 storybook` | 单项目 75 分钟 | `ready`，`knowledge_units=195 / unit_research=195 / page_drafts=195 / wiki_pages=195 / markdown=195` | n/a | 补齐了 storybook 的高层父页 contract 与 runtime gate 正式证据。 |
| `node scripts/test-wiki-lifecycle.mjs --jobs 1 --timeout-minutes 5 storybook dagger` | 单项目 5 分钟 | `75s` 进入 `research`，`300s` 内超时 | `118s` 进入 `research`，`300s` 内超时 | 冷启动预算仍不通过，但失败点已经从 scan 前段收敛到 research 吞吐。 |

## 关键结论

- `storybook` 与 `dagger` 现在都已经有正式 `ready` runtime，可直接从 runtime/state/report 读取 parent contract 与 runtime gate 证据。
- `storybook` 的补齐证据来自 75 分钟长窗口 init：
  - `parent_pages=7`
  - `compose_ready_parents=7`
  - `child_digest_parents=7`
  - `missing_readiness_parents=0`
- `dagger` 在 45 分钟窗口内已经稳定完成：
  - `parent_pages=3`
  - `compose_ready_parents=3`
  - `child_digest_parents=3`
  - `missing_readiness_parents=0`
- 这说明 9.7 关注的两条专项现在都已经能给出明确、可归档的验证结论：
  - 高层父页 contract：两个样本都能从 ready runtime 中回读 child-backed parent contract 摘要
  - runtime gate / readiness：两个样本都能从 workflow summary 与 unit gates 中读取正式阶段状态
- 仍未解决的问题也很明确：
  - `storybook + dagger` 在 5 分钟冷启动预算内都不能完成 init
  - 这已经不再是“runtime 不透明”或“scan 乱打 request”，而是 provider-backed research 吞吐问题

## 分项目观察

### storybook

- 45 分钟窗口先证明了主阻断已从 scan 前段挪到 research：
  - `unit_research=142/195`
  - `runtime_state=researching`
  - 但还没有 `page_drafts / wiki_pages`
- 75 分钟窗口补齐了正式验收证据：
  - `ready`
  - `knowledge_units=195`
  - `unit_research=195`
  - `page_drafts=195`
  - `wiki_pages=195`
  - `markdown=195`
  - `parent_contract=7/7 ready`
- lifecycle 冷启动仍失败：
  - `11` 个 file-purpose request 后，`75s` 进入 `research`
  - 超时前仍未完成 init
- 结论：`storybook` 的 9.7 结构性问题已经被 runtime 与父页 contract 证据覆盖，但冷启动预算仍是后续吞吐专项。

### dagger

- 45 分钟窗口已经完成正式 runtime：
  - `ready`
  - `knowledge_units=64`
  - `unit_research=64`
  - `page_drafts=64`
  - `wiki_pages=64`
  - `markdown=64`
  - `parent_contract=3/3 ready`
- lifecycle 冷启动仍失败：
  - `15` 个 file-purpose request 后，`118s` 进入 `research`
  - `300s` 内没有完成 init
- 结论：`dagger` 的 runtime gate 问题已关闭，剩余问题集中在冷启动预算与页面 fidelity，而不是 runtime incomplete。

## 对 4.1 / 4.2 / 4.3 的影响

- `4.1` 已满足：
  - `storybook` 已给出高层父页 contract 的 ready 证据
  - `dagger` 已给出 runtime gate / readiness 的 ready 证据
  - lifecycle 失败结论也被独立归档，没有与长窗口 init 结论混淆
- `4.2` 已满足：
  - `storybook + dagger` 的 init 专项回归已执行
  - 分析报告已刷新到最新 fresh runtime
- `4.3` 已满足：
  - lifecycle 验证已执行
  - 失败结论已归档到 change 目录

因此，本 change 的 UniSpec 任务可以收口为 `12/12`，但需要明确记录：冷启动吞吐与页面 fidelity 仍应进入后续 change。
