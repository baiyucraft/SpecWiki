# wiki-managed-section-kernel Specification

## Purpose
定义受管 section 的写入、识别、合并与迁移边界，说明 Wiki 页面中的 managed 区段与 user 区段应如何稳定协作。

## Requirements

### Requirement: Managed section 必须以稳定 marker 写入 Wiki 页面
系统 MUST 为每个 runtime 托管 section 写入稳定的 Markdown marker。marker MUST 至少包含稳定 `section_id` 和格式版本，并且不得破坏页面的正常 Markdown 渲染。

#### Scenario: 初始化页面时写入 managed marker
- **WHEN** 系统在 `init`、`update` 或 `rebuild` 中写出一个 managed section
- **THEN** 页面中必须出现该 section 的开始与结束 marker
- **THEN** marker 中必须包含稳定 `section_id`
- **THEN** section 的可见标题和正文仍必须保持为正常 Markdown 内容

#### Scenario: 重复生成同一 section 时 marker 保持稳定
- **WHEN** 同一页面的同一 managed section 在输入未变化时被重复生成
- **THEN** 该 section 的 marker 中 `section_id` 必须保持不变
- **THEN** 系统不得因为重新写盘而生成新的随机 marker 身份

### Requirement: 页面解析必须通过 marker v2 fail closed
系统 MUST 从 `.wiki/*.md` 的 marker v2 解析 managed sections 与 user sections。marker MUST 至少携带稳定 `section_id`、`owner` 和 `version`，并按合同携带 binding/hash 属性。marker 缺失、损坏、版本不支持、结束 id 不一致或 metadata binding 不一致时，系统 MUST 返回显式诊断，不得按已知标题猜测 managed boundary。

#### Scenario: 用户在 managed sections 之间插入新的手工区段
- **WHEN** 页面中在两个 managed sections 之间新增了一个不带 marker 的 `##` 区段或普通 Markdown 内容
- **THEN** 系统必须把这段内容识别为 `managed = false` 的 user section
- **THEN** 系统必须记录该 user section 相对于前后 managed section 的锚点信息

#### Scenario: 页面缺少 marker
- **WHEN** 页面没有 managed marker，即使仍保留旧版稳定 section 标题
- **THEN** 系统 MUST 将页面标记为 `UnmanagedOnly` 并返回 `marker_missing` 诊断
- **THEN** 系统 MUST NOT 从标题反推 managed sections 或 declared/derived truth

#### Scenario: 用户直接修改 managed section 正文
- **WHEN** 页面中某个 managed section 的 marker 和 `section_id` 仍然存在，但正文被手工修改
- **THEN** 系统必须继续把该区段识别为 managed section，而不是把它降级成 user section
- **THEN** 系统必须能够记录该区段当前磁盘内容与最近一次生成内容不一致

### Requirement: merge 必须只替换 managed sections 并保留 user sections
系统 MUST 在页面重生成时只替换 managed sections，并把已同步的 user sections 按原有锚点尽可能插回页面。对于仍然存在的同一 `page_id` 页面，`update` 和 `rebuild` 都 MUST 保留已同步的 user sections。

#### Scenario: 源码变化后保留用户新增区段
- **WHEN** 用户已经在页面中插入了 user section，随后源码变化触发 `update`
- **THEN** 系统必须重新生成受影响的 managed sections
- **THEN** 系统必须把该 user section 插回最终页面
- **THEN** 最终页面不得因为整页重写而丢失这段用户内容

#### Scenario: managed section 集合发生变化时恢复用户锚点
- **WHEN** 页面中的 managed section 集合因生成结果变化而增删顺序
- **THEN** 系统必须优先把 user section 插回仍然存在的相邻 managed section 之间
- **THEN** 当完整原锚点不存在时，系统必须退化到最近仍存在的锚点或页面末尾，而不是直接丢弃用户内容

### Requirement: 无 marker 页面迁移必须显式重建
系统 MUST 通过显式 `init / rebuild` 或受控迁移流程重新生成 marker v2 页面，不得在 `sync / restore` 中进行标题启发式迁移。

#### Scenario: 无 marker 页面进入迁移
- **WHEN** 页面缺少 marker 且需要重新纳入 runtime 管理
- **THEN** 系统 MUST 先保留或报告人工内容，再由正式规划和投影合同重新生成 managed sections
- **THEN** 未完成显式重建前，系统 MUST NOT 把该页面标记为可安全 writeback
