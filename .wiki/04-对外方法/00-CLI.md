---
title: CLI
description: spec-wiki-lite 六个公开命令、参数、输出和退出码
updated: 2026-07-29
owner: docs
---

# CLI

## 命令总览

```text
spec-wiki-lite init [path] [--host codex] [--language zh|en] [--force]
spec-wiki-lite status [--json]
spec-wiki-lite show <change-id> [--artifact <artifact>] [--json]
spec-wiki-lite validate <change-id> [--strict] [--json]
spec-wiki-lite update [--force] [--json]
spec-wiki-lite archive <change-id>
```

除 `init [path]` 外，命令都以当前工作目录为项目根。`--help` 或 `-h` 可用于顶层或单个命令，不能与其他参数组合。

## init

`init` 创建目标目录，确保 `.wiki`、`.spec/changes`、`.spec/archive` 和 `.agents/skills` 存在，同步登记资产，随后返回项目 status。

- `path` 相对于当前目录解析；省略时使用当前目录。
- `--host` 只接受 `codex`，省略时同样使用 Codex。
- `--language` 只用于 init，默认 `zh`；`en` 生成英文 Wiki 并写入项目配置。
- `--force` 允许刷新 managed convention；不会覆盖 scaffold 页面或未登记的用户页面。
- 初始化失败返回恢复提示，可在修复文件写入问题后重试。

## status

`status` 返回项目聚合状态：

- Wiki 语言、bootstrap 状态、页面清单与 `missing_index / invalid_frontmatter / broken_link / orphan_page / duplicate_ssot` issues。
- 八个 `wiki-*` Skills 的安装状态。
- `.spec/changes` 下所有 active changes 的校验结果。

只有 Wiki 结构健康、bootstrap 完成、Skills 和 changes 全部就绪时，顶层 `ready` 才为 `true`。status 是只读命令。

## show

`show <change-id>` 默认返回 change 校验结果。指定 `--artifact <artifact>` 时，同时返回该 artifact 的路径和原文。

允许的 artifact ID：

```text
split proposal design cases tasks unit-tests review-report test-report metadata
```

Artifact 不存在或无法读取时命令失败；show 不修改 change。

## validate

`validate <change-id>` 校验 ID、`meta.yaml`、stage、delivery shape 和当前 stage 的 required artifacts。

- 默认模式要求 required artifacts 存在。
- `--strict` 额外要求 required artifacts 非空。
- `verification/archive` stage 的普通 change 还必须具有 full/pass 的 review 与 verification 报告。
- 校验未通过时仍返回结构化 issues，并使用退出码 `2`。

## update

`update` 从 `.wiki/config.yaml` 读取目标语言并重新执行资产同步：Skills 更新到当前包版本，缺失 scaffold/managed 页面被补齐，用户页面始终保留。

语言迁移只处理内容仍等于 package 模板的登记文件；冲突会在写入前失败。`--force` 只额外允许覆盖 managed 页面，不改变 scaffold 和未登记页面的保护规则。

## archive

`archive <change-id>` 没有预演模式。命令先执行严格校验，再把 active change 移到 `.spec/archive/YYYY-MM-DD-<change-id>`。

- 目标存在时拒绝覆盖。
- 普通 change 必须位于 `verification` 或 `archive` stage。
- Child 归档同步 parent metadata 与 split marker，写入失败时回滚。
- Parent 归档要求全部 children 的 active/archive/marker 证据一致。

## JSON 输出

`status`、`show`、`validate`、`update` 支持 `--json`，输出单行 envelope：

```json
{"ok":true,"data":{}}
```

失败时输出：

```json
{"ok":false,"error":"message"}
```

`validate --json` 在校验不通过时保留 `data` 中的 issues，并设置 `ok: false`。

## 退出码

| 退出码 | 含义 |
| --- | --- |
| `0` | 成功 |
| `1` | 文件系统、解析或执行失败 |
| `2` | Change 未就绪 |
| `64` | CLI 用法错误 |
