## ADDED Requirements

### Requirement: 全局 `spec-wiki` CLI 必须同时提供 bootstrap 入口与 runtime 转发入口
系统 MUST 提供一个可全局安装并可执行的 `spec-wiki` CLI。该 CLI MUST 同时暴露顶层 `spec-wiki init` bootstrap 入口，以及 `spec-wiki wiki <action>` runtime 转发入口，其中 `<action>` 在本轮 MUST 覆盖 `init`、`status`、`query`、`update`、`sync`、`rebuild`。

#### Scenario: 用户执行项目 bootstrap
- **WHEN** 用户全局安装 `spec-wiki` 后在仓库根目录执行 `spec-wiki init`
- **THEN** CLI MUST 进入项目 bootstrap 流程
- **THEN** CLI MUST 不把这条命令与 repo wiki runtime action 混为同一个入口

#### Scenario: 宿主或用户执行 runtime action
- **WHEN** 宿主或用户执行 `spec-wiki wiki status`、`spec-wiki wiki query` 或其他受支持 action
- **THEN** CLI MUST 将该命令转发为 `wiki-runtime` 可识别的 action
- **THEN** CLI MUST 继续复用现有 runtime 的 action 合同，而不是在 Node 层发明另一套 Wiki 业务动作

### Requirement: `spec-wiki init` 必须检测或显式选择首批宿主
`spec-wiki init` MUST 支持 `codex`、`claude`、`codebuddy` 三类宿主。CLI MUST 能通过项目根目录下的宿主目录检测当前可用宿主；当检测结果不唯一或用户希望覆盖默认选择时，CLI MUST 支持显式指定宿主。

#### Scenario: 检测到唯一宿主时直接采用
- **WHEN** 用户执行 `spec-wiki init`，且当前项目内只检测到一个受支持宿主目录
- **THEN** CLI MUST 直接采用该宿主完成 bootstrap
- **THEN** CLI MUST 不要求用户再次做冗余选择

#### Scenario: 存在多个宿主时要求显式决策
- **WHEN** 用户执行 `spec-wiki init`，且当前项目内检测到多个受支持宿主目录
- **THEN** CLI MUST 要求用户选择本次要初始化的宿主，或接受显式传入的宿主参数
- **THEN** CLI MUST 不得在多个宿主之间静默猜测一个默认结果

#### Scenario: 显式指定宿主时允许创建缺失目录
- **WHEN** 用户显式指定 `codex`、`claude` 或 `codebuddy`，但项目内尚不存在对应宿主目录
- **THEN** CLI MUST 仍能完成该宿主的 bootstrap
- **THEN** CLI MUST 只创建本次宿主 bootstrap 所需的命令、skill、hook 或 prompt 目录结构

### Requirement: 宿主 bootstrap 必须把资产写入宿主真实可识别路径
`spec-wiki init` MUST 按宿主真实规则生成命令、skill、hook 或 prompt 资产，而不是把三类宿主强行写成同一路径。本轮必须遵循以下正式落点：`Claude` 使用 `.claude/skills/wiki-*/SKILL.md`；`CodeBuddy` 使用 `.codebuddy/skills/wiki-*/SKILL.md`、`.codebuddy/hooks/spec-wiki/*.mjs` 与 `.codebuddy/settings.json`；`Codex` 使用 `.codex/skills/wiki-*/SKILL.md`。

#### Scenario: 为 Claude 写入项目内 skill
- **WHEN** 用户对 `claude` 执行 `spec-wiki init`
- **THEN** 系统 MUST 在 `.claude/skills/` 下只生成 `wiki-init`、`wiki-status`、`wiki-query` 与 `wiki-update`
- **THEN** 系统 MUST NOT 继续生成 `sync` 或 `rebuild` 的显式入口
- **THEN** 系统 MUST NOT 再保留 Claude command 形态

#### Scenario: 为 CodeBuddy 写入项目内 skills、hooks 与 settings
- **WHEN** 用户对 `codebuddy` 执行 `spec-wiki init`
- **THEN** 系统 MUST 在 `.codebuddy/skills/` 下只生成 `wiki-init`、`wiki-status`、`wiki-query` 与 `wiki-update`
- **THEN** 系统 MUST NOT 继续生成 `wiki-sync`、`wiki-rebuild` 或 `spec-wiki-runtime`
- **THEN** 系统 MUST 在 `.codebuddy/hooks/spec-wiki/` 下生成 `spec-wiki` 管理的 hook 脚本，并写入 `.codebuddy/settings.json`

#### Scenario: 为 Codex 写入项目内 skill
- **WHEN** 用户对 `codex` 执行 `spec-wiki init`
- **THEN** 系统 MUST 在 `.codex/skills/` 下只生成 `wiki-init`、`wiki-status`、`wiki-query` 与 `wiki-update`
- **THEN** 系统 MUST NOT 继续生成 `wiki-sync` 或 `wiki-rebuild` 的显式入口
- **THEN** 系统 MUST NOT 再保留 Codex prompt 形态


### Requirement: 生成出的宿主资产必须把显式入口路由回全局 CLI
本轮生成出的宿主入口资产 MUST 只负责引导宿主回到全局 `spec-wiki` CLI，而 MUST NOT 在宿主资产中复制 Wiki 业务规则。对于宿主当前显式暴露的 `init`、`status`、`query`、`update` 四个动作，资产内容 MUST 明确映射到 `spec-wiki wiki <action>`。

#### Scenario: 宿主入口映射到 runtime 子命令
- **WHEN** 宿主读取 `update` 对应的入口资产
- **THEN** 该资产 MUST 明确引导宿主调用 `spec-wiki wiki update`
- **THEN** 该资产 MUST 不得把 `update` 的 Wiki 业务语义重写在宿主文案里

#### Scenario: bootstrap 入口与 runtime 入口语义分离
- **WHEN** 宿主读取 `init` 对应的入口资产
- **THEN** 该资产 MUST 映射到 `spec-wiki wiki init`
- **THEN** 该资产 MUST 不得再次调用顶层 `spec-wiki init` bootstrap 命令

### Requirement: 共享 `spec-wiki wiki <action>` 转发层必须承接现有 runtime 的长流程与 bridge 合同
`spec-wiki wiki <action>` 在 `v0.1.0` MUST 正式承接当前阶段 index-only runtime 的稳定合同，而不是因为宿主入口从专属 JS 包切到共享 CLI 就让核心动作再次分叉。对 `init`、`update`、`query`，共享 CLI MUST 保留当前 `wiki-runtime` 的 `result / error` 合同；对 `init` 与 `update`，若当前 core workflow 通过临时 short-circuit 在 index substrate 持久化后提前结束，CLI MUST 继续把该终态如实传回，而不得伪装成“完整 knowledge/page runtime 已完成”。

#### Scenario: `init` 和 `update` 在 index-only 终态后如实结束
- **WHEN** 宿主或用户执行 `spec-wiki wiki init` 或 `spec-wiki wiki update`
- **THEN** 共享 CLI MUST 允许 core workflow 在完成 `scan / symbol / graph / module tree / facts snapshot` 后提前返回终态
- **THEN** CLI MUST 不得把这类 index-only 终态伪装成 knowledge/page 主链也已经完成

#### Scenario: `query` 正式返回 index-first 结构化结果
- **WHEN** 宿主或用户执行 `spec-wiki wiki query`
- **THEN** 共享 CLI MUST 正式保证返回 index-first 结构化命中结果，例如 modules、sources、symbols、relations 或 symbol edges
- **THEN** `query` 的正式可用性 MUST 不再依赖 knowledge/page projection 已经完整产出

#### Scenario: 非 `v0.1.0` 正式承诺动作不作为发布 blocker
- **WHEN** 当前宿主或用户继续看到 `status`、`sync` 或 `rebuild` 命令入口
- **THEN** 这些动作 MAY 继续保留 CLI 命令面与路由壳
- **THEN** 但 `v0.1.0` 的正式发布承诺 MUST 仅覆盖 index-only 的 `init / update / query`

### Requirement: `spec-wiki init` 必须幂等刷新自己的命名空间资产
`spec-wiki init` MUST 设计为可重复执行。重复执行时，系统 MUST 只刷新 `spec-wiki` 自己管理的宿主入口资产，而 MUST NOT 覆盖宿主目录中非 `spec-wiki` 命名空间的其他文件。

#### Scenario: 重复执行时刷新已有 bootstrap 资产
- **WHEN** 用户在同一仓库对同一宿主重复执行 `spec-wiki init`
- **THEN** 系统 MUST 刷新该宿主下由 `spec-wiki` 管理的宿主入口资产
- **THEN** 系统 MUST 不因已存在旧文件而报错退出

#### Scenario: 重复执行时不污染其他宿主或用户自定义资产
- **WHEN** 用户重复执行 `spec-wiki init`，且宿主目录中存在其他工具或用户自定义文件
- **THEN** 系统 MUST 只修改 `spec-wiki` 管理范围内的宿主资产
- **THEN** 系统 MUST 不得覆盖不属于 `spec-wiki` 命名空间的宿主资产

### Requirement: 全局 CLI 实现必须保持 agents / orchestration / runtime 三层边界
系统 MUST 将宿主差异、`spec-wiki init` 编排和 `spec-wiki wiki <action>` runtime forwarding 保持为相互独立的实现层。宿主差异 MUST 收敛在 `agents` 层，init 主流程 MUST 收敛在 `orchestration` 层，runtime 调用与结果解析 MUST 收敛在 `runtime` 层，而 MUST NOT 重新混写回单一 `bootstrap` 主轴。

#### Scenario: 宿主差异不回流到 init 编排公共层
- **WHEN** 系统为 `codex`、`claude` 或 `codebuddy` 生成宿主资产
- **THEN** 宿主真实路径、能力差异和模板产出 MUST 留在宿主层实现中
- **THEN** `orchestration` 层 MUST 不得重新硬编码单宿主路径细节

#### Scenario: runtime forwarding 不与 bootstrap 资产生成混层
- **WHEN** 系统实现 `spec-wiki wiki <action>` 的 binary resolve、invoke 或结果解析
- **THEN** 这些逻辑 MUST 保持在独立的 runtime forwarding 层
- **THEN** 它们 MUST 不得重新并入 `init` 编排或宿主资产生成流程

