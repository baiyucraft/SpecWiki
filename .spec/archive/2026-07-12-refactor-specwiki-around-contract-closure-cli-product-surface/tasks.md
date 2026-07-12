---
implementation-ready: true
---

# refactor-specwiki-around-contract-closure-cli-product-surface 任务计划

## 任务总览

任务按五个可验收能力块推进：一级命令、协议与呈现、统一 init、治理命令、宿主资产与迁移。用户已明确授权自动实现、验证和归档。

## 实现模式

tdd

先确认 Red，再写最小 Green，最后在测试守卫下重构。

## 1. 一级 CommandSpec、help 与参数合同

- [x] 1.1 Red: UT-001 编写一级命令与 help 分层失败测试
- [x] 1.2 Green: UT-001 实现 CommandSpec router、默认/完整/per-command help
- [x] 1.3 Refactor: UT-001 收敛命令和 help 单一事实来源
- [x] 1.4 Red: UT-002 编写 query/host/machine/bridge 参数失败测试
- [x] 1.5 Green: UT-002 实现参数解析、互斥校验和 usage 64
- [x] 1.6 Refactor: UT-002 清理 host 命名与交互边界，删除 tool 兼容入口

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过
- [x] 注释规范检查完成

## 2. 结构化错误、事件流、human renderer 与退出策略

- [x] 2.1 Red: UT-003 编写 errorKind 与退出码矩阵失败测试
- [x] 2.2 Green: UT-003 扩展 CoreResponse parser 并实现集中 exit policy
- [x] 2.3 Refactor: UT-003 删除字符串匹配和散落退出判断
- [x] 2.4 Red: UT-004 编写 NDJSON chunk/唯一终态失败测试
- [x] 2.5 Green: UT-004 实现 coreEventStream 并接入 forwarder
- [x] 2.6 Refactor: UT-004 统一 invoke/forward 的事件校验辅助逻辑
- [x] 2.7 Red: UT-009 编写 typed human renderer 失败测试
- [x] 2.8 Green: UT-009 实现短/长流程 DTO 翻译
- [x] 2.9 Refactor: UT-009 确认 renderer 不推导 readiness/outcome/action

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过
- [x] 注释规范检查完成

## 3. 统一 init 与结构化 partial

- [x] 3.1 Red: UT-005 编写 bootstrap ready/partial/failed 测试
- [x] 3.2 Green: UT-005 改造宿主写入为结构化 BootstrapReport
- [x] 3.3 Refactor: UT-005 保持幂等写入并收敛 recovery hint
- [x] 3.4 Red: UT-006 编写 Rust `cli_init`、输入校验和 outcome 矩阵测试
- [x] 3.5 Green: UT-006 实现内部流式 action、landing 聚合和失败 data
- [x] 3.6 Refactor: UT-006 保持原 `init/wikiInit` DTO 与公开 action 集合不变

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过
- [x] 注释规范检查完成

## 4. Governance 单次 evaluation 与顶层只读命令

- [x] 4.1 Red: UT-007 编写单次 live evaluation report 测试
- [x] 4.2 Green: UT-007 增加 changes/change/validate report API
- [x] 4.3 Refactor: UT-007 保持 policy SSOT 与 live/cache 边界
- [x] 4.4 Red: UT-008 编写 Rust transport、changeId 和 TS parser 测试
- [x] 4.5 Green: UT-008 接入顶层治理 action 与 typed envelopes
- [x] 4.6 Refactor: UT-008 收敛 errorKind、not_enabled/not_found/invalid 语义

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过
- [x] 注释规范检查完成

## 5. 宿主资产、文档与迁移门禁

- [x] 5.1 Red: UT-010 编写一级调用文本、identity 保留和旧命令扫描测试
- [x] 5.2 Green: UT-010 更新宿主生成器、README、Wiki capability、E2E 和测试调用
- [x] 5.3 Refactor: UT-010 固定扫描模式/排除范围并清理旧兼容代码

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过
- [x] 注释规范检查完成

## 用例到任务映射

| 系统测试用例 | 大 task | 小 task / 验证 |
| --- | --- | --- |
| ST-001 | 1 | 1.1-1.6 / UT-001 / UT-002 |
| ST-002 | 1, 2 | UT-002 / UT-003 / UT-004 / UT-009 |
| ST-003 | 3 | 3.1-3.6 / UT-005 / UT-006 |
| ST-004 | 4 | 4.1-4.6 / UT-007 / UT-008 |
| ST-005 | 5 | 5.1-5.3 / UT-010 |
| ST-006 | 5 | 5.1-5.3 / scan gate |

## 执行顺序

- 先完成 1 和 2，建立稳定 CLI/protocol 基础。
- 再完成 3 和 4，接入统一 init 与治理 transport。
- 最后完成 5 和全量回归。

## 暂缓事项

- archive、workspace validate、doctor、repair、trace 和索引扩展均留待其它 change。
