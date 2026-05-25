## MODIFIED Requirements

### Requirement: restore 必须从正式产物恢复本地 runtime，而不是重新生成知识
系统 MUST 在 restore 前同时校验 projection digest snapshot 的合法性与最小对齐关系。若 `page-digests` 中存在 status/reason 非法、引用的 `page_id` 无法对齐 metadata/pages，或 projection snapshot 与正式页面输出发生不可接受 mismatch，系统 MUST 显式拒绝 restore，而不是静默继续恢复 `.wiki/.cache/**`。

#### Scenario: restore 拒绝非法 projection digest snapshot
- **WHEN** `.wiki/.knowledge/runtime/page-digests.jsonl` 中存在非法 `projection_status`、reason contract 不一致，或 digest 引用的页面无法与 metadata/pages 对齐
- **THEN** restore MUST 显式拒绝该 snapshot
- **THEN** 系统 MUST NOT 静默继续恢复 `.wiki/.cache/**`
