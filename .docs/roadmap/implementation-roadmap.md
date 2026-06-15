# 实施迭代

## 文档定位

本文档只描述 3.0 的实施路线，不重复主设计和运行时总架构。

请先结合：

- [.wiki/06-设计文档/00-总体设计.md](../../.wiki/06-设计文档/00-总体设计.md)
- [.wiki/06-设计文档/01-Runtime设计.md](../../.wiki/06-设计文档/01-Runtime设计.md)
- [.wiki/06-设计文档/03-核心场景.md](../../.wiki/06-设计文档/03-核心场景.md)
- [.wiki/06-设计文档/04-扩展场景.md](../../.wiki/06-设计文档/04-扩展场景.md)

## 编号约定

当前迭代设计建立在归档迭代之上，但编号从新的 `迭代 10` 开始继续。

这里的含义是：

- 归档中的 `迭代 0-9.9` 作为历史基线保留
- 归档中原本尚未开始的旧 `迭代 10-12` 不再沿用
- 当前 `迭代 10+` 以 3.0 四包设计为新基线重新编号

这样处理的原因很简单：

- 当前主问题已经不是继续补旧 `wiki-core`
- 当前主问题是先把 `wiki-model / wiki-index / wiki-knowledge / wiki-runtime` 四层拆正
- 拆层完成前，旧版的 RAG、宿主桥接和平台扩展顺序都不成立

## 当前前置基线

从归档迭代继承下来的前置能力，当前视为迁移输入，而不是新的迭代主题：

- 已有 scanner、symbols、edges、graph facts、SQLite/runtime、query、provider-backed research 等历史实现
- 已有 `Facts -> Knowledge Planning -> Research -> Compose` 的 2.0 主链方向
- 已有 `storybook + dagger` 的专项验收经验
- 已有 19 个测试项目集与生命周期验证脚本

但 3.0 下的实施重点已经改变：

- 先收边界，再补能力
- 先拆层，再优化 Agent 消费
- page 不再是主本体，query 也不能再 page-first

## 新迭代依赖关系

```text
迭代 10: 四包拆分与基础测试收口
    ↓
迭代 11: wiki-index 能力增强，先服务 AGENT/provider 初步消费
    ↓
迭代 12: wiki-knowledge 收口，建立 knowledge-first update 主线
    ↓
迭代 13: wiki-runtime 收口，完成 query route 与 lifecycle
    ↓
迭代 14: page projection 与知识生命周期专项验收
    ↓
迭代 15: AGENT/provider 深化消费层
```

## v0.2.0 发布收口规划

`v0.1.0` 的正式发布面仍然收敛在 `index-only init / status / update / query`。
`v0.2.0` 不追求直接完成 3.0 终态，而是把已经存在但尚未成为正式发布面的
`Facts -> Knowledge Planning -> Research -> Compose -> Assemble`
主链扶正为可承诺的 `knowledge runtime`。

版本定位：

- `v0.1.0 = index substrate release`
- `v0.2.0 = knowledge runtime first-class release`
- `v0.3.0 = declared knowledge + full lifecycle + richer consumption`

`v0.2.0` 的总目标：

- `init / status / update / query` 不再以 `index_only` 作为正式成功语义
- `KnowledgeDomain / KnowledgeUnit / KnowledgeTree` 成为正式运行主线
- 高层 parent unit 拥有真正的 `UnitResearch`
- 正式 workflow 不再默认 `StructuralResearchProvider` fallback
- `.wiki/.knowledge/**` 落最小正式知识产物集
- query 保持 `index-first -> knowledge -> page fallback`

`v0.2.0` 的总非目标：

- 不承诺完整 `declared knowledge` 生命周期
- 不把 `sync / rebuild` 升级成完整公开支持面
- 不做宿主高级 bridge / session
- 不做 retrieval / RAG
- 不做 3.0 最终 page projection 全细化
- 不把 19 项目全量回归设为前置发布门槛

建议按发布风险拆成 5 个迭代：

```text
迭代 A: Parent Contract 扶正
    ↓
迭代 B: Production Research Policy 收口
    ↓
迭代 C: Knowledge Runtime 产物最小集落盘
    ↓
迭代 D: Knowledge-First Update 收口
    ↓
迭代 E: Query Route 与 v0.2.0 Release 验收
```

### 迭代 A：Parent Contract 扶正

目标：

- 去掉高层父页的 seed-only 短路
- `Overview / Architecture / DomainIndex / config-surface parent` 全部产出真实 `UnitResearch`
- `SystemResearch / DomainResearch` 降为 seed / overlay

通过线：

- 高层父页不再跳过 unit research
- 父页 compose 不越层抓 leaf
- `storybook + dagger` 上可以证明 parent contract 成立

### 迭代 B：Production Research Policy 收口

目标：

- 正式 workflow 默认走 provider-backed research
- `StructuralResearchProvider` 只保留给测试 / 显式开发模式
- provider 失败、中断、恢复路径可诊断

通过线：

- 无 provider 时正式 workflow 失败并留 checkpoint
- 不再 silently structural fallback
- `status` 能区分 `runtime_incomplete / blocker / needs_update`

### 迭代 C：Knowledge Runtime 产物最小集落盘

目标：

- 把最小正式知识产物写入 `.wiki/.knowledge/**`
- SQLite 回到 cache / working state 角色
- 形成 `.knowledge + pages + metadata + cache` 最小恢复闭环

建议最小落盘集：

- `knowledge_domains`
- `knowledge_units`
- `knowledge_tree`
- parent / unit research 摘要
- `page_digests`
- runtime gates / readiness 摘要

通过线：

- Git-tracked 正式产物不再只剩 pages / metadata
- B 用户可基于正式产物恢复 `.cache`
- 恢复后可以直接 `status / query`

### 迭代 D：Knowledge-First Update 收口

目标：

- `update` 从 page-first 改成 knowledge-first
- 主线变成
  `ChangeSet -> affected knowledge scope -> refresh derived knowledge -> refresh projections`

通过线：

- 小改动不会默认重写整批页面
- 能定位受影响 knowledge scope
- `storybook + dagger` 上增量行为可解释、可验证

### 迭代 E：Query Route 与 v0.2.0 Release 验收

目标：

- 对外继续保留 `term-only`
- 内部正式收口 `index -> knowledge -> page fallback`
- 完成 `v0.2.0` 发布门槛验收

通过线：

- page fallback 不再伪装成 facts / index 命中
- query 能回连 knowledge 层
- `storybook + dagger` 证明知识单元拆分、parent contract、citation/evidence/diagram 落页与 runtime readiness 已成立
- 再补 `1-2` 个 smoke 项目后，最后回到 `19` 项目回归

`v0.2.0` 的统一发布门槛：

- `init / status / update / query` 不再以 `index_only` 为正式语义
- 高层 parent unit 全部有独立 `UnitResearch`
- 正式 workflow 无默认 structural fallback
- `.wiki/.knowledge/**` 有最小正式对象集
- `storybook + dagger` 专项通过
- 至少 `1-2` 个小规模 smoke 项目通过
- 完成一次 `.wiki/02-开发指南/00-代码注释规范.md` 合规检查
- 最后再跑 `19` 项目回归

## 迭代 10：四包拆分与基础测试收口

状态：

- 计划中

目标：

- 完成 `wiki-model / wiki-index / wiki-knowledge / wiki-runtime` 四包拆分
- 把旧 `wiki-core` 收缩为真正的 `wiki-runtime`
- 确保拆分后的基础测试和基础工作流跑通

范围：

- 抽出共享领域模型到 `wiki-model`
- 抽出代码事实与索引到底层 `wiki-index`
- 抽出 knowledge planning / research / compose 到 `wiki-knowledge`
- 将 workflow、storage、transport、query route 骨架和 `.wiki` 生命周期归属收回 `wiki-runtime`
- 清理旧的跨层直接引用和大包内循环职责
- 同步收口 crate 依赖方向、workspace 配置和基础测试入口

本轮不做：

- 不把“索引增强”塞进本轮
- 不把“Agent 真正可用化”塞进本轮
- 不补新的 RAG、session、宿主体验层
- 不重开 page 质量专项

完成标准：

- 四个 crate 的边界与 [.wiki/06-设计文档/00-总体设计.md](../../.wiki/06-设计文档/00-总体设计.md) 一致
- `wiki-runtime` 不再承载 facts/index 和 knowledge 的主要实现
- runtime 侧只要求完成归属迁移与不回退，不要求在本轮收口最终 lifecycle contract
- 基础 Rust 测试与工作区构建可通过
- 基础 `init / query / status / update / rebuild` 工作流不因拆分而回退
- 至少形成一组稳定的拆分后 smoke 测试，证明结构调整没有把主链拆坏

## 迭代 11：wiki-index 面向 AGENT/provider 的初步可用

状态：

- 计划中

目标：

- 强化 `wiki-index`，让它先成为 AGENT 或 provider 可直接消费的底座
- 让“找方法、找入口、看影响”优先走索引，而不是先翻 page

范围：

- 收口 `wiki-index` 对外稳定查询接口
- 强化 symbol lookup、file lookup、module lookup、entrypoint lookup
- 强化 callers / callees / imports / exports / impact slice 等图查询能力
- 补齐适合 AGENT/provider 使用的紧凑返回结构，例如路径、符号签名、命中理由、置信度、相关边
- 让 `wiki-runtime` 能以 index-first 方式暴露最小查询路由，但重点仍是 `wiki-index` 本身
- 补齐代表性样本上的方法定位、入口定位、影响分析测试

优先场景：

- “某个方法在哪里”
- “这个符号被谁调用”
- “改这里会影响什么”
- “这个项目的入口在哪”

本轮不做：

- 不扩张 `wiki-knowledge` 的 research / compose 能力
- 不做完整 AGENT session、宿主 bridge、RAG、向量检索
- 不把 query 重新做成 page-first 包装

完成标准：

- AGENT/provider 可以先用索引结果完成初步定位，而不是先依赖 page 文本
- `wiki-index` 能稳定回答方法定位、入口定位和基础影响分析
- 返回结构足够紧凑，适合 prompt/context 直接消费
- `wiki-runtime` 上的 query route 已能体现 `index first -> knowledge -> page` 的方向，但不在本轮扩成完整 runtime 收口

## 迭代 12：wiki-knowledge 收口

状态：

- 计划中

目标：

- 把 `wiki-knowledge` 做成正式知识层，而不是旧页面生成器的变体
- 建立 knowledge-first 的 update 主线

范围：

- 收口 `KnowledgeDomain / KnowledgeUnit / KnowledgeTree`
- 收口 `derived knowledge` 与 `declared knowledge`
- 收口 record / summary / digest / projection decision
- 把 `wiki-update` 的旧 page-first 心智替换为 knowledge-first
- 明确“先更新记录和摘要，再决定是否升级为 page”

本轮不做：

- 不收 `.wiki` 生命周期细节
- 不收 query route 和 transport
- 不回到“新功能后默认重写 page”的旧路径

完成标准：

- 新功能更新、bug 避坑沉淀、主动声明规范都有正式知识归属
- `declared / derived / page projection` 的边界清晰
- update 不再默认等于“重写页面”

## 迭代 13：wiki-runtime 收口

状态：

- 计划中

目标：

- 把 `wiki-runtime` 收成真正的运行时壳
- 正式收口 query route、lifecycle 和 `.wiki` 恢复链

范围：

- 收口 `init / status / sync / update / rebuild / query`
- 收口 `.wiki/.knowledge + pages + wiki.metadata.json + .cache`
- 收口 A 提交、B 拉取后的本地恢复链
- 让 query 明确遵循 `index first -> knowledge -> page projection`
- 暴露 `ready / stale / needs_update / conflict` 等状态

本轮不做：

- 不再重做 knowledge 模型
- 不补新的页面质量专项
- 不做宿主可用化抛光

完成标准：

- runtime 生命周期与恢复策略和 [.wiki/06-设计文档/01-Runtime设计.md](../../.wiki/06-设计文档/01-Runtime设计.md) 一致
- `.cache` 仍不上库，但能由正式产物恢复
- query route 不再 page-first

## 迭代 14：page projection 与知识生命周期专项验收

状态：

- 计划中

目标：

- 验证 page 已经被降级为 projection
- 验证知识生命周期和核心场景已经闭环

范围：

- 校准 page projection 何时生成、何时升级、何时回收
- 校准 declared knowledge、derived knowledge 与 page 的投影关系
- 围绕 [.wiki/06-设计文档/03-核心场景.md](../../.wiki/06-设计文档/03-核心场景.md) 和 [.wiki/06-设计文档/04-扩展场景.md](../../.wiki/06-设计文档/04-扩展场景.md) 做专项验收
- 对 `storybook + dagger` 做页面质量和知识抽象专项复核

重点验收：

- `init` 后是否能快速理解项目
- Agent 开工前是否能读到规范和约定
- 新功能后是否更新的是知识，而不是盲目增页
- bug 修复后是否能稳定沉淀为避坑记录
- 主动声明规范、废止规范、冲突治理是否可见

完成标准：

- page 不再反向主导 knowledge
- 场景闭环以知识层为主，而不是靠页面堆砌
- `storybook + dagger` 不再只是页面专项，也同时验证知识抽象是否站得住

## 迭代 15：AGENT/provider 深化消费层

状态：

- 计划中

目标：

- 在四层边界稳定后，再深化 AGENT/provider 的正式消费面
- 把归档中旧 `10/11` 里真正还有价值的消费层能力，放到更晚、更合适的位置

范围：

- 收口适合 AGENT/provider 的 context pack / compact query payload
- 评估是否引入更深一层的 retrieval、session 或 usage contract
- 收口宿主侧对 progress、query result 和长流程状态的消费方式
- 补齐 e2e 级别的 AGENT/provider 使用验证

本轮不做：

- 不推翻四包边界
- 不把 retrieval 重新做成新的真相层
- 不让宿主层重写 Wiki 业务语义

完成标准：

- AGENT/provider 可以稳定消费 `wiki-index + wiki-knowledge + wiki-runtime` 的正式输出
- 消费层只是消费层，不再反向污染 core 边界

## 测试与验收要求

总体要求不变，仍以 [.wiki/06-设计文档/00-总体设计.md](../../.wiki/06-设计文档/00-总体设计.md) 中的测试项目集和 [AGENTS.md](../../AGENTS.md) 中的测试约束为准。

当前迭代路线下的测试重点：

- `迭代 10` 先保证拆分不破坏基础测试和基础工作流
- `迭代 11` 开始补方法定位、入口定位、影响分析的专项测试
- `迭代 12-14` 逐步把知识更新、runtime 生命周期和场景验收补齐
- `storybook + dagger` 继续作为专项样本
- 全量 19 项目集与生命周期脚本继续保留为总体验收基线

## 一句话总结

```text
3.0 后续迭代的顺序不是先做宿主炫技，
而是先拆正四层，再把 index 做成 Agent 可用底座，再收 knowledge、runtime 和 projection。
```
