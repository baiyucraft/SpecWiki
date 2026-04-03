## RENAMED Requirements

### FROM: `v0.2.0` 的公开 workflow surface 必须继续收敛为 `init`、`status`、`update`、`query`
### TO: 当前正式公开 workflow surface 必须收敛为 `init`、`status`、`update`、`query`、`sync`、`rebuild`

## ADDED Requirements

### Requirement: 当前正式 release truth sources 必须对齐到同一 workflow 合同
系统 MUST 让 `README.md`、`README-CN.md`、release note、主包帮助文本、staged README 与主包版本号共同对齐到同一套正式 workflow 合同。当前版本一旦正式采用 `v0.2.0 minimal formal knowledge runtime` 与 6 个公开 workflow，系统 MUST NOT 再允许其中任一 truth source 继续描述 `v0.1.0 index-only` 或仅 4 个公开动作。

#### Scenario: 用户读取任一正式 release 文档
- **WHEN** 用户查看 `README.md`、`README-CN.md`、release note 或 staged README
- **THEN** 文档 MUST 一致描述当前正式 runtime contract，而不是混用 `v0.1.0 index-only` 与 `v0.2.0 knowledge runtime`
- **THEN** 文档 MUST 一致列出当前正式公开的 workflow 入口

#### Scenario: 主包版本号切到正式 release 口径
- **WHEN** 当前发布被命名为 `v0.2.0`
- **THEN** `packages/spec-wiki/package.json` 的版本号与 release 文档 MUST 使用同一版本口径
- **THEN** 系统 MUST NOT 继续保留与当前 release 叙事冲突的旧版本说明
