# spec-wiki

`spec-wiki` 为人和 Agent 提供共享的本地 Repo Wiki。它扫描代码仓库，在 `.wiki/` 下构建 repo-local knowledge runtime，并安装受管宿主资产。Codex 是唯一 reference host；Claude、CodeBuddy 是 compatible hosts。

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

`spec-wiki --help-all` 还会列出已经实现的高级与治理命令：

```text
spec-wiki sync [--repo-root <path>]
spec-wiki rebuild [--repo-root <path>] [--bridge-stdio]
spec-wiki changes [--repo-root <path>]
spec-wiki change <change-id> [--repo-root <path>]
spec-wiki validate <change-id> [--repo-root <path>]
spec-wiki archive <change-id> [--dry-run | --apply | --resume <operation-id>] [--repo-root <path>]
```

Archive 默认执行 `--dry-run`；`--apply` 执行写入，`--resume <operation-id>` 恢复部分完成操作。退出码为：成功 `0`；统一 init partial、change 校验无效或 archive 未就绪/需恢复为 `2`；用法错误为 `64`；domain、workflow、protocol、I/O 或 archive manifest 错误为 `1`。

## Runtime 合同

Runtime 在 `.wiki/` 下写入 formal knowledge、页面投影、metadata 和可恢复 cache。Query 只接受非空 term，并返回 canonical readiness、governance、`route_groups` 和 `answer`；不同 route 的 score 不跨组比较。

JavaScript API 保留 `wikiInit`、`wikiStatus`、`wikiQuery`、`wikiUpdate`、`wikiSync`、`wikiRebuild`。所有宿主都安装 repo-local `wiki-*` skills 并执行一级 CLI；宿主资产不定义第二套命令或 Runtime 合同。

长期架构与协作说明从 [.wiki/INDEX.md](./.wiki/INDEX.md) 进入。版本化产品 surface 由 [v0.2.0 发布合同](./.wiki/04-对外方法/02-v0.2.0发布合同.md) 定义。

## 平台与协议

Windows x64 是 v0.2.0 发布合同目标。Manifest、staged package、build、tests 或 dry-run 均不能单独证明 registry 已发布。本项目采用 GNU GPL v3.0。
