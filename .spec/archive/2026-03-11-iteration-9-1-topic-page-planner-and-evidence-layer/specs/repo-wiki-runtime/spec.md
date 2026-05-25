## ADDED Requirements

### Requirement: runtime 必须持久化专题页与 evidence block 的稳定身份
系统 MUST 在现有页面 runtime 中持久化专题页和 evidence block 的稳定身份。专题页 MUST 与现有 overview / architecture / module / workflow 页面共用同一套页面状态、managed section 和缓存 contract；evidence block MUST 复用现有 page/section runtime，而不是写入新的 sidecar 层。

#### Scenario: 专题页进入正式 runtime
- **WHEN** planner 生成专题页
- **THEN** runtime MUST 为其写入正式页面状态、input hash 和 managed sections
- **THEN** 该页面 MUST 与其他正式页面一样进入 `.wiki/*.md` 和状态库

#### Scenario: evidence block 复用现有 runtime contract
- **WHEN** 页面 section 中存在 evidence block
- **THEN** 这些 block MUST 继续受现有 managed section contract 管理
- **THEN** 系统不得为 evidence 单独引入新的正式 runtime 目录

### Requirement: debug trace 模式必须通过显式配置开启且不得污染正式协议
系统 MUST 提供可选的 debug trace 模式，并允许通过启动参数或 steering 配置显式开启。该模式写出的调试信息 MUST 与现有 stdout JSON IPC 正式协议隔离，不能改变 `result/error/progress` 的编码 contract。

#### Scenario: 启动参数开启 debug trace
- **WHEN** 宿主以启动参数显式传入 debug trace 目录或开启标记
- **THEN** 系统 MUST 在指定目录写出调试 trace
- **THEN** stdout 上的正式 JSON 协议 MUST 保持不变

#### Scenario: steering 开启 debug trace
- **WHEN** repo 根配置显式开启 debug trace
- **THEN** `init/update/rebuild` MUST 为本次 workflow 写出调试 trace
- **THEN** debug trace 关闭时系统不得额外写出调试产物
