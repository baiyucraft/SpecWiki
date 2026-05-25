## MODIFIED Requirements

### Requirement: 系统必须从当前仓库快照和最近一次 runtime 计算稳定 ChangeSet
系统 MUST 基于当前仓库快照、最近一次 SQLite 扫描缓存、关系型 `WikiState` 状态表、页面落盘状态，以及上一轮 formal knowledge identity snapshot 计算稳定的 `ChangeSet`。`ChangeSet` MUST 至少区分新增、修改、删除和结构性变化的源码集合，并保留能够回溯到源码路径、状态表缺失位置、formal identity 缺失位置或页面锚点变化的证据。

#### Scenario: 单个源码文件内容变化
- **WHEN** 某个已登记源码文件的内容 fingerprint 与最近一次 runtime 记录不一致
- **THEN** 系统 MUST 把该源码记录为 modified source
- **THEN** 系统 MUST 保留该源码路径作为 change evidence

#### Scenario: 新增或删除源码文件
- **WHEN** 当前仓库快照中出现新增源码文件，或某个已登记源码文件消失
- **THEN** 系统 MUST 把对应源码记录为 added source 或 removed source
- **THEN** 系统不得把这类变化静默折叠成普通 stale

#### Scenario: 结构性线索发生变化
- **WHEN** manifest、workspace 成员、入口文件、`FilePurpose` 关键角色、formal knowledge identity 相关输入或模块边界相关线索发生变化
- **THEN** 系统 MUST 把这些变化标记为 structural change
- **THEN** 系统 MUST 允许后续 workflow 触发 knowledge tree / module tree / projection planning 的重新计算

### Requirement: 系统必须把 ChangeSet 映射到受影响模块、页面和 section
系统 MUST 基于稳定的 formal knowledge identity、`source -> module -> unit -> page -> section` 映射，把 `ChangeSet` 先转换为 `AffectedKnowledgeScope`，再从该 scope 派生 `AffectedSet`。`AffectedKnowledgeScope` MUST 能表达受影响的 `KnowledgeDomain`、`KnowledgeUnit`、parent propagation、受影响页面与页面内受影响 section；映射过程中系统 MUST 消费 formal identity objects 与稳定投影映射，而不是重新猜测 managed section 边界。系统 MUST NOT 先比较页面差异、`page_digests` 或最终 Markdown，再反推 knowledge scope。

#### Scenario: 局部源码变化先定位受影响知识单元再落到页面
- **WHEN** 某个源码变化只命中已有 unit identity 与已有 projection 映射
- **THEN** 系统 MUST 先把变化记录到对应的 `AffectedKnowledgeScope`
- **THEN** 系统 MUST 只把由该 scope 派生出的模块、页面和 section 标记为受影响

#### Scenario: 模块树或知识树变化触发局部 replan
- **WHEN** 结构性变化导致模块树、knowledge tree 或 projection planning 发生变化
- **THEN** 系统 MUST 重新计算受影响 subtree 对应的 `AffectedKnowledgeScope`
- **THEN** 系统 MUST 识别新增单元、删除单元、新增页面、删除页面、父子关系变化页面和受影响的 `section_anchors`

#### Scenario: 页面差异不得成为 knowledge scope 的一级输入
- **WHEN** 系统观察到页面路径、`page_digests` 或 Markdown 内容与上一轮不同
- **THEN** 系统 MUST 将这些差异视为 projection 层现象
- **THEN** 系统 MUST NOT 仅凭这些差异直接生成 `AffectedKnowledgeScope`

### Requirement: 系统必须定义 cache invalidation 与回退边界
系统 MUST 按 knowledge scope 与 projection scope 执行失效和回退，而不是对任何变化都直接清空整个 runtime。局部可修复时，系统 MUST 只失效受影响 unit 的 research / digest / projection cache、受影响页面的 page context / generation cache、页面状态行和 FTS 索引；关键 runtime 不一致时，系统 MUST 提升为 `needs_rebuild` 或显式 full rebuild。关键 runtime 不一致的判定 MUST 覆盖 formal knowledge artifacts、状态表、section 行、映射表和页面文件之间的一致性。

#### Scenario: 局部知识范围变化只失效局部缓存
- **WHEN** `AffectedKnowledgeScope` 只包含已有 unit 的局部变化
- **THEN** 系统 MUST 只失效这些 unit 对应的 research / digest / projection cache，以及相关页面的 page context / generation cache、页面状态行和 FTS 记录
- **THEN** 未受影响 unit 与页面的缓存必须继续可复用

#### Scenario: 关键运行时不一致触发回退
- **WHEN** formal knowledge artifact 缺失、页面文件缺失、关键状态表缺失、section 行缺失，或 formal identity 与页面锚点映射无法重建一致性
- **THEN** 系统 MUST 把运行时状态提升为 `needs_rebuild`
- **THEN** `update` 不得继续在该状态上尝试局部修复
