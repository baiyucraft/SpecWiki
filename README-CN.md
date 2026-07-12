# spec-wiki

`spec-wiki` 为人和 Agent 提供共享的本地 Repo Wiki。它扫描代码仓库，在 `.wiki/` 下构建 repo-local knowledge runtime，并为 Codex、Claude、CodeBuddy 安装受管接入资产。

## 快速开始

```bash
spec-wiki init --host codex --repo-root .
spec-wiki status --repo-root .
spec-wiki query "payment flow" --repo-root .
spec-wiki update --repo-root .
```

`init` 是唯一初始化入口：先安装所选宿主资产，再初始化 runtime，最后返回 landing 状态。多个宿主可重复传入 `--host`，或使用逗号分隔的 `--hosts`。

## 命令面

默认 help 突出产品主路径：

```text
spec-wiki init [--host <host> | --hosts <host,host>] [--repo-root <path>] [--no-interactive]
spec-wiki status [--repo-root <path>]
spec-wiki query <term...> [--repo-root <path>]
spec-wiki update [--repo-root <path>] [--bridge-stdio]
```

`spec-wiki --help-all` 还会列出已经实现的高级命令：

```text
spec-wiki sync [--repo-root <path>]
spec-wiki rebuild [--repo-root <path>] [--bridge-stdio]
spec-wiki changes [--repo-root <path>]
spec-wiki change <change-id> [--repo-root <path>]
spec-wiki validate <change-id> [--repo-root <path>]
```

`--json` 输出机器可读 JSON 或 NDJSON。`--bridge-stdio` 隐含 machine mode，且只适用于流式命令。退出码固定为：成功 `0`，统一 init partial 或 change 校验无效 `2`，用法错误 `64`，workflow/protocol 失败 `1`。

## Runtime 合同

runtime 在 `.wiki/` 下写入 formal knowledge、页面投影、metadata 和可恢复 cache。查询沿 `index -> knowledge -> page fallback` 路由；治理信息只暴露结构化引用，不把 change artifact 正文复制进 Wiki。

JavaScript API 继续保留 `wikiInit`、`wikiStatus`、`wikiQuery`、`wikiUpdate`、`wikiSync`、`wikiRebuild`。宿主资产继续保留 `wiki-*` skill identity 和 Claude `/wiki:*` identity，但执行统一的一级 CLI。

长期架构与协作说明从 [.wiki/INDEX.md](./.wiki/INDEX.md) 进入。

## 平台与协议

当前正式目标是 Windows x64。本项目采用 GNU GPL v3.0。
