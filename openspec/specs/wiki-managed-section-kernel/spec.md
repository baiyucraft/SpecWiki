## ADDED Requirements

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

### Requirement: 页面解析必须区分 managed sections、user sections 和 legacy 页面
系统 MUST 能够从 `.wiki/*.md` 中解析出 managed sections 与 user sections。marker 存在时 MUST 以 marker 为主；marker 不存在但页面仍是迭代 3 的 legacy 格式时，系统 MUST 提供基于已知 section 标题的 best-effort fallback 解析。

#### Scenario: 用户在 managed sections 之间插入新的手工区段
- **WHEN** 页面中在两个 managed sections 之间新增了一个不带 marker 的 `##` 区段或普通 Markdown 内容
- **THEN** 系统必须把这段内容识别为 `managed = false` 的 user section
- **THEN** 系统必须记录该 user section 相对于前后 managed section 的锚点信息

#### Scenario: 页面仍处于 legacy 无 marker 格式
- **WHEN** 页面没有 managed marker，但仍保留当前页面类型的稳定 section 标题
- **THEN** 系统必须按已知 section 标题把对应区段恢复为 managed sections
- **THEN** 系统必须把 managed sections 之间的其余内容恢复为 user sections

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

### Requirement: legacy 页面迁移必须是显式的 best-effort 过程
系统 MUST 对 legacy 无 marker 页面提供 best-effort 迁移能力，但不得在无法可靠识别 managed boundary 时假装迁移成功。

#### Scenario: legacy 页面成功迁移
- **WHEN** legacy 页面保留了预期的稳定 managed section 标题，且系统能唯一识别这些边界
- **THEN** 系统必须把该页面迁移为可解析的 managed/user section 状态
- **THEN** 下一次由 runtime 重写该页面时，输出必须采用 managed marker 格式

#### Scenario: legacy 页面无法可靠迁移
- **WHEN** legacy 页面缺失关键标题、重复标题或 boundary 已被破坏，导致系统无法可靠恢复 managed sections
- **THEN** 系统必须给出明确 warning
- **THEN** 系统不得把该页面误记为已经完成 editable runtime 迁移
