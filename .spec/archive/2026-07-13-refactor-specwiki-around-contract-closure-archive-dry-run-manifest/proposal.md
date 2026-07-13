# refactor-specwiki-around-contract-closure-archive-dry-run-manifest

## 问题

治理内核已经能够读取 `.spec` evidence、计算 readiness 并识别 parent/child 归档一致性，但 archive 仍缺少完整的执行合同。当前无法稳定表达“先校验、再预览、最后执行”的流程，也无法在目录移动和 parent 文件同步发生部分失败时，判断操作是否尚未开始、部分完成或已经完成。

如果 archive 直接复用全仓治理 fingerprint，或把跨目录移动描述成文件系统级原子事务，会产生错误的 TOCTOU 判断和不可恢复的中间状态。Wiki 写入也不应混入 `.spec` archive 事务。

## 目标

- 为 `archive <change-id>` 建立默认先 validate 的 preflight 合同。
- 提供完全只读的 dry-run，并输出可审计的 operation manifest。
- 使用限定于本次归档输入的 precondition digest，在 apply 前重新校验。
- 支持将 active change 归档到固定命名的 archive target，并处理目标冲突。
- 在 child 归档时同步 parent `meta.yaml` 和 `split.md` 的归档状态。
- 让 apply 失败后能够通过持久化状态判断已执行步骤，并提供可重试或人工恢复提示。
- 将 Wiki 交互限制为 issue 和 evidence refs 输出，不调用 Wiki 写流程。

## 非目标

- workspace-wide validate、`doctor`、`repair`、`trace` 或通用 rollback engine。
- Wiki 页面生成、`.wiki/**` 写入、知识重投影或自动调用 `sync/update`。
- 整个项目归档、release 清理、已有 archive 覆盖、合并或自动改名。
- 任意 parent 文本重写器或通用 Markdown AST 平台。
- 兼容旧 archive CLI 或旧 manifest schema。
- 跨设备、跨文件系统的严格 ACID 事务保证。
- 重新实现 GovernanceService 的治理规则或在 CLI transport 中复制规则。
- archive 完成后自动 commit、自动调用 UniSpec skill 或自动修改 Wiki。

## 成功标准

- archive 默认先执行治理 validate；validate 未通过时不移动目录、不修改 parent、不写 Wiki。
- dry-run 不移动 active change，不修改 parent，不写 `.wiki/**`，并能给出 readiness、blocking issues、目标路径和预期 parent diff。
- manifest 包含 schema/version、operation id、change/source/target、mode、validation/readiness、artifact hash summary、parent 预期 diff、precondition digest、步骤状态、失败步骤和 recovery hint。
- precondition digest 绑定本次归档所需的 source tree、目标不存在事实、child metadata、parent 文件、archive readiness evidence、规则版本和 manifest schema；apply 前不一致则拒绝写入。
- archive target 使用固定日期格式和时区；目标已存在、source 不存在、重复归档、parent 缺失或 parent/child 不一致时均返回可识别失败结果。
- apply 成功终态同时满足：active source 不存在、archive target 存在、parent `meta.yaml` 与 `split.md` 的归档标记一致，并且 manifest 标记为 completed。
- apply 的每个可恢复步骤都有持久化状态；失败不会被报告为成功，重试能够区分未开始、部分完成和已完成操作。
- Wiki 只输出 issue/refs，不调用 Wiki 写流程；Wiki 侧问题不会被静默吞掉。
- CLI 的 human 与 JSON 输出、退出码和现有治理命令保持一致，并能表达 dry-run、冲突、失败步骤和恢复建议。

## 影响范围

- `wiki-runtime` 的 archive workflow、治理 preflight、文件系统操作和 manifest 持久化边界。
- `wiki-model` 的 archive request/result、manifest、步骤状态、失败分类和恢复提示 DTO。
- CLI archive 命令及 human/JSON 输出合同。
- `.spec/changes/**`、`.spec/archive/**` 目录移动，以及 active parent `meta.yaml` / `split.md` 同步。
- archive 的单元测试、系统测试、失败恢复/幂等重试测试和分发测试。
- CLI 与 archive 相关的稳定 Wiki 契约文档。

本 change 的规则来源于项目本地 `.docs/design/specwiki-contract-closure.md`、`governance-runtime-integration.md` 和 `specwiki-cli-unification.md`，属于对本地设计的改写落地，不是 upstream 项目的直接迁移。现有 GovernanceService 和 governance evidence store 作为只读 preflight 输入复用，archive 不复制其治理规则。

## 交付形态

single-change

这是 `refactor-specwiki-around-contract-closure` parent split 下的 child change，依赖 governance isolation 与 CLI product surface 两个已完成 child。

## 风险

- Windows 上目录 rename 与多个文件替换不是天然的跨文件系统原子事务，若没有 durable manifest 和幂等恢复，可能留下可见的部分完成状态。
- dry-run 与 apply 之间存在 TOCTOU，digest 输入范围过宽或过窄都会造成误拒绝或错误执行。
- 当前 parent marker 判断依赖固定文本格式，`split.md` 格式漂移可能造成冲突或误判。
- active/archive 双占、重复 archive、parent 已归档和 parent metadata 不一致需要保持确定性错误分类。
- manifest 持久化位置、锁/单 writer 语义和崩溃恢复协议如果定义不清，会使重试行为不可验证。

## 未知项

- dry-run manifest 的最终持久化位置、命名和是否允许只输出 stdout，留待 design 根据现有 runtime storage 约定确定。
- precondition digest 的具体规范化编码、算法版本和文件排序规则留待 design 定稿。
- apply 的 staging、临时文件替换、durable 状态落盘和恢复步骤留待 design 定稿；不预先承诺自动 rollback。
- archive CLI 的最终参数形式、锁实现、退出码细分和机器协议字段留待 design 与现有 CLI DTO 对齐。
- Wiki-sync issue 的具体分类和是否 blocking 需要在 design 中明确，但不得触发 Wiki 写事务。

## 参考资料

- `.spec/changes/refactor-specwiki-around-contract-closure/split.md`
- `.wiki/06-设计文档/01-Runtime设计.md`
- `.wiki/06-设计文档/02-Agents设计.md`
- `.wiki/04-对外方法/00-CLI.md`
- `.docs/design/specwiki-contract-closure.md`
- `.docs/design/governance-runtime-integration.md`
- `.docs/design/specwiki-cli-unification.md`
