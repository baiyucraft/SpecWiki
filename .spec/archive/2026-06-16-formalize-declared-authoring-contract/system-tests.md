# formalize-declared-authoring-contract 系统测试用例

## 用例总览

本文件定义 declared authoring contract 的归档级系统验收。覆盖范围来自 `proposal.md` 与 `design.md`：typed `scope`、`relations`、lifecycle status、受管 declared block 回写、`.wiki/.knowledge/declared/**` snapshot、restore/audit、`sync / update / status / query` 消费，以及 page 不能重新成为 declared truth source。

这些用例不要求浏览器验证。即使 `.spec/config.yaml` 中 `playwright: true`，本 change 的验收入口仍是 CLI、Rust runtime tests、artifact 文件检查和人工 artifact 审查。

## 系统测试用例

### ST-001 change artifact 结构可被 UniSpec 识别

- 关联成功标准: change 目录具备 proposal、design、system-tests、tasks 和 meta artifact。
- 覆盖设计点: UniSpec 使用 `.spec/changes/<change-id>/` 作为 active change 控制面。
- 前置条件: 当前 change 保留 proposal、design、system-tests、tasks、meta。
- 操作 / 触发: 运行 `unispec status --json` 和 `unispec validate formalize-declared-authoring-contract`。
- 期望结果: 当前 change 不因 required artifact 缺失或 metadata 不一致被阻塞。
- 验证方式: UniSpec CLI 结构校验。

### ST-002 DeclaredRecord schema 支持 typed scope、relations 与 lifecycle

- 关联成功标准: `DeclaredRecord` 具备 typed `scope`、稳定 identity、`relations`、lifecycle status 与 canonical serialization。
- 覆盖设计点: `DeclaredRecord` 是正式 authoring object，而不是 page writeback 附件。
- 前置条件: `wiki-model` 中存在 declared record / scope / relation / status 模型。
- 操作 / 触发: 运行 declared model 定向测试。
- 期望结果: typed scope canonical key 稳定；`supersedes / replaced_by / deprecated` 关系可验证；冲突 lifecycle、缺失 relation target、关系环被拒绝。
- 验证方式: `cargo test -p wiki-model declared -- --nocapture`。

### ST-003 受管 declared block 可回写 lifecycle relations

- 关联成功标准: managed declared block 能被解析为正式 declared record，且支持 `deprecated / replaced_by / supersedes` lifecycle 关系。
- 覆盖设计点: page 只作为受管 authoring surface；正式 declared truth 以 `.wiki/.knowledge/declared/**` 为准。
- 前置条件: 测试仓库已生成受管 `.wiki` 页面和 knowledge artifacts。
- 操作 / 触发: 在 managed declared section 中写入 active、deprecated、replaced_by、supersedes 的 declared block，并执行 `sync`。
- 期望结果: `sync` 返回 `declared_writeback`；`.wiki/.knowledge/declared/records.jsonl` 写出 typed scope、relations、status；restore 后这些字段保持稳定。
- 验证方式: Rust runtime integration test 覆盖 parse -> artifact -> restore roundtrip。

### ST-004 sync 分类优先级稳定

- 关联成功标准: `sync` 固定区分 `declared_writeback`、`metadata_only`、`illegal_drift`，并按 `illegal_drift > declared_writeback > metadata_only` 优先级输出。
- 覆盖设计点: parse failure、非法 managed drift、metadata-only 和合法 declared writeback 必须有稳定诊断。
- 前置条件: 测试仓库包含 generated managed sections 和 optional declared blocks。
- 操作 / 触发: 分别执行合法 declared writeback、metadata-only 编辑、非法 managed drift、混合 declared + illegal drift。
- 期望结果: 合法 declared 返回 `declared_writeback`；纯 metadata 返回 `metadata_only`；非法或混合非法 drift 返回 `illegal_drift` 且不得污染 declared snapshot。
- 验证方式: `cargo test -p wiki-runtime --test runtime editable_runtime -- --nocapture`。

### ST-005 artifact restore 不从页面正文反推 declared truth

- 关联成功标准: `.wiki/.knowledge/declared/**` 是正式 truth layer；restore/audit 不依赖 page 正文重新构造 declared record。
- 覆盖设计点: page projection 与 authoring surface 不得冒充 declared truth。
- 前置条件: 测试仓库已有 declared artifact、metadata、official page tree。
- 操作 / 触发: 删除 `.wiki/.cache/**` 后执行 status/query restore；另构造 page declared block 与 artifact 不一致的负向场景。
- 期望结果: cache-less restore 从 artifact 恢复；page 与 artifact 锚点冲突时返回显式 stale/blocker 或拒绝恢复，不把页面正文反推为新 truth。
- 验证方式: Rust runtime restore / knowledge artifacts roundtrip tests。

### ST-006 declared lifecycle 驱动 update/status/query 可观察诊断

- 关联成功标准: declared lifecycle 能影响 readiness、health summary、recommended_action，并驱动 derived / projection refresh。
- 覆盖设计点: `sync / update / status / query` 正式消费 declared lifecycle，而不是只消费 page writeback 结果。
- 前置条件: 测试仓库已有 declared record 关联 unit / projection。
- 操作 / 触发: 通过 declared writeback 或 declared health scope 标记 affected declared records，再执行 `status`、`update`、`query`。
- 期望结果: `status` 暴露 declared health/readiness；`update` 在无源码 dirty set 时仍消费 declared stale scope；`query` 保持可用并通过 provenance/recommended_action 表达 degraded 或 stale。
- 验证方式: `cargo test -p wiki-runtime --test runtime status_and_update -- --nocapture` 与 `cargo test -p wiki-runtime --test runtime query_sync_rebuild -- --nocapture`。

### ST-007 workspace gate 无 declared 回归

- 关联成功标准: 本 change 不破坏 workspace regression gate。
- 覆盖设计点: declared authoring 不能引入 page-tree、init workflow 或 runtime restore 回归。
- 前置条件: 已补齐相关 artifact / tests / wiki baseline。
- 操作 / 触发: 运行 `pnpm run lint` 和 `pnpm run test`。
- 期望结果: lint 通过；workspace test 通过。若存在与本 change 无关的失败，必须有正式隔离依据和后续修复记录，不能作为归档 pass 证据。
- 验证方式: workspace 命令输出。

### ST-008 样本验证是非门禁 smoke check

- 关联成功标准: 样本验证不得在 0 total / 0 coverage 时冒充 pass 证据。
- 覆盖设计点: 归档证据必须来自实际执行的测试或明确降级的 smoke check。
- 前置条件: 当前仓库可能不存在 `storybook` / `dagger` 样本目录。
- 操作 / 触发: 运行 `node scripts/run-test-projects.mjs` 或明确指定可用样本。
- 期望结果: 有样本时记录 pass/fail；无样本时记录 skipped / not-applicable，不计入归档门禁。
- 验证方式: 脚本输出和 test-report 记录。

## 覆盖矩阵

| 成功标准 | 系统测试用例 | 验证方式 |
| --- | --- | --- |
| UniSpec required artifact 完整 | ST-001 | `unispec status --json` / `unispec validate formalize-declared-authoring-contract` |
| DeclaredRecord typed scope、relations、status、identity 成立 | ST-002 | `cargo test -p wiki-model declared -- --nocapture` |
| Managed declared block 回写 lifecycle relations | ST-003 | Runtime integration test |
| sync 分类优先级稳定 | ST-004 | `cargo test -p wiki-runtime --test runtime editable_runtime -- --nocapture` |
| declared artifact 是 restore/audit truth source | ST-005 | Runtime restore / artifact roundtrip tests |
| update/status/query 消费 declared lifecycle | ST-006 | `status_and_update` / `query_sync_rebuild` runtime tests |
| workspace regression gate 不回退 | ST-007 | `pnpm run lint` / `pnpm run test` |
| 样本验证不冒充归档证据 | ST-008 | `node scripts/run-test-projects.mjs` 输出记录 |

## 边界与异常

- 不把本 change 扩成 answer assembly、provider hardening 或治理平台。
- 不要求浏览器或图像验证；`playwright: true` 与 `imageAnalysis: true` 不改变本 change 的 CLI/runtime 验收入口。
- 样本验证在没有实际样本项目时只能记录为 skipped / not-applicable。
- `system-tests.md` 只定义验收边界；测试执行结果以 `test-report.md` 为准。

## 验证数据与环境

- 当前仓库 `.spec/changes/formalize-declared-authoring-contract/`。
- Rust workspace：`wiki-model`、`wiki-runtime`。
- UniSpec CLI。
- `.spec/config.yaml`：`playwright: true`、`imageAnalysis: true`，但本 change 无浏览器入口。

## 未覆盖项

- 无意扩展 provider-backed full compose 稳定性。
- 无意扩展 answer assembly host contract。
- 无意定义 conflict 自动裁决或治理看板。

## 参考资料

- `proposal.md`
- `design.md`
- `.wiki/05-规格基线/capabilities/declared-knowledge-lifecycle/spec.md`
- `.wiki/05-规格基线/capabilities/knowledge-runtime-artifacts/spec.md`
- `.wiki/05-规格基线/capabilities/repo-wiki-workflow/spec.md`
