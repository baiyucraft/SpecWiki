# refactor-specwiki-around-contract-closure-governance-isolation 系统测试用例

## 用例总览

本组用例覆盖只读治理内核的完整纵向合同：`.spec` evidence discovery、治理状态与 validator parity、独立 fingerprint 和 SQLite 派生缓存、status/query/update 产品组合、Rust/TS transport 闭集，以及不触发 archive 或 code graph 副作用的边界。所有用例均以 fixture repository 和 CLI / Rust workflow 可观察结果验证，不要求浏览器交互。

## 系统测试用例

### ST-001 未启用治理时不阻断 Wiki runtime

- 关联成功标准: 无 `.spec` 的仓库返回 `governance.readiness: not_enabled`，普通 status/query/update 继续可用。
- 覆盖设计点: Governance readiness；产品级组合规则。
- 前置条件: 已初始化且 core readiness 为 ready 的 fixture repo，不创建 `.spec`。
- 操作 / 触发: 依次执行 runtime status、query 和 source no-op update。
- 期望结果: 三个响应均包含唯一 `governance` summary，readiness 为 `not_enabled`；core fusion、query trust 和 update 结果不因治理未启用而降级。
- 验证方式: Rust runtime integration test 序列化断言，并通过 TS `parseResult` 解析同一 payload。

### ST-002 合法治理仓库可发现 active/archive change 和 evidence refs

- 关联成功标准: 合法 `.spec` 能列出 active/archive changes、stage、parent/child 关系、artifact refs 和治理 readiness。
- 覆盖设计点: Evidence discovery；Status、validate 与 inspect；公开 DTO。
- 前置条件: fixture 同时包含 enabled-empty `.spec`、standalone active change、parent/child active change和 dated archived child，metadata 与 artifact 完整。
- 操作 / 触发: 执行 governance status、list changes 和 inspect change。
- 期望结果: enabled-empty 在没有 cache 时仍为 ready 且 counts 为 0；其它 fixture 的 active/archive 数量、location、stage、role、parent、order、depends_on、artifact path/hash 均正确且排序稳定；不返回 artifact 正文。
- 验证方式: Rust integration test 对结构化 DTO 和重复执行结果进行断言。

### ST-003 合法但未满足 gate 的 change 返回 blocked

- 关联成功标准: missing required artifact、report 非 full/pass 等生成稳定 blocking issue。
- 覆盖设计点: GovernancePolicy；blocked 与 archive readiness 边界。
- 前置条件: 分别准备缺失当前 stage required artifact、verification report 非 pass、scope 非 full 的 fixture。
- 操作 / 触发: 对每个 fixture 执行 validate 和 repo status。
- 期望结果: readiness 为 `blocked`；issue 携带稳定 rule_id、change_id、artifact_ref、`blocking` severity 和 `review_governance`；正常中间 stage 仅 `archive_ready: false`，不会因尚未归档而 blocked。
- 验证方式: Rust integration test 断言 rule id 和 gate summary，不断言可变展示文案。

### ST-004 损坏或矛盾 evidence 返回 conflict 且保留可读结果

- 关联成功标准: 非法 metadata、parent/child relation 和损坏 report frontmatter 产生稳定阻断诊断。
- 覆盖设计点: 错误分类；单 change 故障隔离；路径安全。
- 前置条件: 同一 repo 包含一个合法 change，以及 YAML 损坏、id/path 不一致、archive marker 矛盾和不可读 artifact fixture。
- 操作 / 触发: 执行 governance status、list 和 validate。
- 期望结果: repo governance readiness 为 `conflict`；合法 change summary 仍可返回；每个失败项携带定位 path 和 rule id；不得猜测损坏 change 的 stage。
- 验证方式: Rust integration test；不可读权限场景在支持权限位的平台执行，Windows 使用 store error fixture 替代。

### ST-005 Rust validator 与 UniSpec 0.1.0 parity fixtures 一致

- 关联成功标准: runtime validate 与当前治理规则在 fixture 上保持 parity，且不存在 Skill/TS/runtime 三套 required artifact matrix。
- 覆盖设计点: Governance policy 与 parity；唯一规则所有权。
- 前置条件: 版本化 fixture 覆盖 standalone、exploration stub、parent、child、各 stage required artifacts、report evidence 和 archive marker。
- 操作 / 触发: 运行 Rust parity integration suite，对照仓库内规范化 expected JSON。
- 期望结果: identity、stage/role/dependencies、required artifacts、blocking classification、report gate 和 archive readiness 全部一致；常规测试不调用全局 `unispec` executable。
- 验证方式: `cargo test -p wiki-runtime --test runtime governance_policy`；可选 oracle 刷新脚本不属于通过前置条件。

### ST-006 仅修改 `.spec` 时只刷新 governance derived cache

- 关联成功标准: `.spec` 修改改变 governance fingerprint，但不改变 code graph snapshot、source fingerprint 或 source dirty set。
- 覆盖设计点: 独立 fingerprint；update governance refresh；SQLite cache。
- 前置条件: 已 init 且 governance cache fingerprint 与 evidence 一致的 fixture repo。
- 操作 / 触发: 记录 graph/source/knowledge snapshot；只修改 active change 的 `proposal.md`；执行 status 后执行 update。
- 期望结果: status 先返回 governance `stale` 且 source state 仍 fresh；update 在 source workflow no-op 的情况下事务刷新 governance tables；graph/source/knowledge snapshot 保持不变。
- 验证方式: Rust integration test 比较修改前后 snapshot id、dirty set、governance fingerprint 和 SQLite rows。

### ST-007 governance blocked/conflict 不污染 core readiness

- 关联成功标准: governance blocked/conflict 只影响治理结果和 next action，不把普通 Wiki query/update 强制降为全局 blocked。
- 覆盖设计点: 产品级组合；recommended action 优先级。
- 前置条件: core ready repo 分别放入 blocked 与 conflict governance fixture；另准备 core blocked + governance blocked fixture。
- 操作 / 触发: 执行 status、普通源码 query 和 source update。
- 期望结果: core ready 时 `RuntimeReadiness.fusion` 保持 ready，governance summary 分别为 blocked/conflict，产品 action 为 `review_governance`；core blocked 时保留 core 的 rebuild/repair 类 action，不被 `review_governance` 覆盖；index/knowledge query result 的 trust 与 confidence 不被连带降级。
- 验证方式: Rust status/query/update integration test。

### ST-008 governance query 只返回结构化 refs 并遵守 freshness

- 关联成功标准: ready 时 query 返回 `governance_evidence_ref / governance_summary_hit`，并携带 refs、provenance 和 recommended action。
- 覆盖设计点: Governance query refs；query trust；cache freshness。
- 前置条件: ready、blocked、stale、conflict 四类 governance fixture，并存在与 query term 匹配的 change id/artifact path。
- 操作 / 触发: 执行相同 term 的 query。
- 期望结果: ready 返回 summary/artifact refs；blocked 返回带 blocked provenance 的 refs；stale 不返回旧 cached refs并建议 update；conflict 只返回诊断 ref；所有结果都不包含 artifact 正文。
- 验证方式: Rust query integration test 检查 route tag、ref kind、confidence、provenance、action 和 payload 字段。

### ST-009 `.spec` 不进入 code graph、knowledge 或业务 FTS

- 关联成功标准: `.spec` 原始证据保留原位，不进入 Wiki 正文、code facts 或业务 FTS。
- 覆盖设计点: truth 分层；结构化 query 边界。
- 前置条件: `.spec` artifact 写入一个只在治理正文出现的唯一 token，然后执行 update。
- 操作 / 触发: 查询 index files/symbols、knowledge records、Markdown fallback 和治理 refs。
- 期望结果: 唯一 token 不命中 code/knowledge/page route；按 change id 或 artifact path 可命中 governance route；`.spec` 文件内容未被移动或改写。
- 验证方式: Rust SQLite/query integration test和文件 hash 前后比较。

### ST-010 Rust/TS transport 闭集一致且治理操作无写副作用

- 关联成功标准: Rust DTO、runtime transport、TS parser 对 readiness/ref 闭集一致；本 change 不产生 archive move、manifest 或 parent 写操作。
- 覆盖设计点: Transport 合同；只读边界；错误闭集。
- 前置条件: 构造五种 readiness、change/artifact refs、blocking/warning issue 和 `review_governance` action payload；准备 active parent/child fixture。
- 操作 / 触发: 序列化 Rust payload并由 TS parser 解析；执行 status/list/inspect/validate/query。
- 期望结果: 合法闭集全部通过，未知状态或残缺 summary 被拒绝；响应不存在旧 `governance_readiness` 字段；所有治理只读操作后 active/archive 目录、parent metadata 和 artifact hash 完全不变。
- 验证方式: `cargo test -p wiki-model`、runtime integration、`pnpm --filter spec-wiki test` 以及 fixture tree/hash 比较。

## 覆盖矩阵

| 成功标准 | 系统测试用例 | 验证方式 |
| --- | --- | --- |
| 无 `.spec` 返回 not_enabled 且 core 可用 | ST-001 | Rust workflow + TS parser |
| 合法治理仓库列出 change、关系和 refs | ST-002 | Rust integration |
| 缺失 artifact / report gate 生成 blocking issue | ST-003 | Policy/runtime integration |
| 非法 metadata、关系和 frontmatter 可诊断 | ST-004 | Error fixture integration |
| runtime validator 与现有规则 parity | ST-005 | Versioned parity fixtures |
| `.spec` delta 与 source/code graph 隔离 | ST-006 | Snapshot/fingerprint comparison |
| governance 问题不阻断 core Wiki | ST-007 | Status/query/update integration |
| ready query 返回治理 refs，stale/conflict 降级 | ST-008 | Query route integration |
| `.spec` 原文不进入 Wiki/code/knowledge/FTS | ST-009 | Storage/query isolation |
| Rust/TS 闭集一致且无治理写副作用 | ST-010 | Contract tests + tree/hash comparison |

## 边界与异常

- enabled-but-empty `.spec` 返回 ready，counts 为 0。
- 单个 change 损坏不得抹去其它可读 change summary。
- status 只比较 live evidence 与 cache，不写 SQLite；validate 绕过 cache；update 才刷新 cache。
- archive readiness 在中间 stage 只作为 gate 信息，不自动形成 blocking issue。
- symlink/路径逃逸和超大 artifact 必须产生可定位错误，不允许读取 repo root 外内容。
- Windows 权限模型无法稳定制造 EACCES 时，使用注入式 store error fixture，不把平台限制当作通过证据。

## 验证数据与环境

- 使用 `tempfile` 构建最小 Rust fixture repository，包含 `.spec/changes`、`.spec/archive`、源码和已初始化 `.wiki/.cache`。
- parity expected JSON 固定 `@uni-sw/unispec` 0.1.0 的规范化行为，不要求 CI 全局安装该包。
- Rust 验证使用 workspace bundled SQLite；TS 验证使用 `packages/spec-wiki` 现有 Vitest。
- 不需要网络、浏览器、真实用户仓库或 archive 写权限。

## 未覆盖项

- 不验证 public CLI 一级命令布局；属于后续 `cli-product-surface` child。
- 不验证 archive dry-run、operation manifest、目录移动和恢复；属于后续 `archive-dry-run-manifest` child。
- 不验证治理全文搜索、release gate 或 capability baseline 产品化；均为 proposal 非目标。

## 参考资料

- `proposal.md`
- `design.md`
- `.wiki/02-开发指南/01-测试与验收.md`
- `@uni-sw/unispec` 0.1.0 规范化 parity fixtures（来源为现有治理实现；目标落点为 Rust validator 测试；采用方式为改写）
