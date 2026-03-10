## ADDED Requirements

### Requirement: `init`、`update` 与 `rebuild` 必须输出稳定的阶段进度
系统 MUST 让 `init`、`update` 和 `rebuild` 在真实 workflow 阶段边界上输出稳定 progress 事件，而不是只输出最终结果。阶段划分 MUST 基于当前主链中的实际步骤，例如扫描、symbol parsing、graph resolution、graph analysis、module tree、context、page planning、render、state write 与 metadata write。

#### Scenario: init 输出主链阶段进度
- **WHEN** 调用方执行 `init`
- **THEN** 系统 MUST 至少为扫描、符号解析、页面渲染和状态写盘这些阶段输出 progress 事件
- **THEN** 这些事件的阶段顺序 MUST 与实际 `init` 主链执行顺序一致

#### Scenario: update 回退时仍输出当前实际路径的阶段进度
- **WHEN** `update` 因 `missing` 或 `needs_rebuild` 回退到 `init` 或 `rebuild`
- **THEN** 系统 MUST 继续输出回退后实际执行路径对应的 progress 事件
- **THEN** 调用方 MUST 能从事件流中区分“增量 update”与“回退到 init/rebuild”的真实执行情况

### Requirement: `update` 必须优先使用局部 symbol/edge 工作集刷新
当 `update` 只涉及有限数量的 `graph_refresh_sources` 时，系统 MUST 优先按受影响文件及其必要 graph frontier 读取 symbol/edge 工作集，并与本轮 changed snapshot 合并，而不是对任意小变更都无条件回读全量 `symbols / edges`。当受影响范围、frontier 膨胀或状态缺失超出局部刷新可控范围时，系统 MUST 显式回退到全量读取或更高等级的 fallback 路径，而不是静默退化为固定的全量路径。

#### Scenario: 小范围图变化优先走局部工作集
- **WHEN** `update` 只涉及少量受影响源码文件，且 graph frontier 仍处于可控范围
- **THEN** 系统 MUST 优先读取这些文件及必要 dependents 对应的 symbol/edge rows
- **THEN** 系统 MUST 不得对这类小变更固定执行全量 `symbols / edges` 回读

#### Scenario: 局部工作集超阈值时显式回退
- **WHEN** `update` 的受影响文件集合、graph frontier 或关键状态缺失超出局部刷新阈值
- **THEN** 系统 MUST 显式回退到全量读取或更高等级 fallback 路径
- **THEN** 该回退 MUST 能被 progress 事件、诊断或等价可观测方式识别
