# refactor-specwiki-around-contract-closure-cli-product-surface 系统测试用例

## 用例总览

用例覆盖一级 CLI、统一 init、human/machine 协议、只读治理入口、宿主资产迁移及旧产品面清理。

## 系统测试用例

### ST-001 一级命令与分层帮助

- 关联成功标准: 默认 help 只突出四个主路径；完整 help 展示已实现高级命令；旧 namespace 不可用。
- 覆盖设计点: CommandSpec、`--help-all`、per-command help、usage 64。
- 前置条件: 已构建 `spec-wiki` CLI。
- 操作 / 触发: 执行默认 help、完整 help、单命令 help、未知命令和旧 `wiki` namespace。
- 期望结果: help 退出 0；默认/完整分层正确；未知/旧命令退出 64。
- 验证方式: Vitest CLI tests + workspace E2E。

### ST-002 参数、输出模式与退出码

- 关联成功标准: query 位置参数、host 参数、human/JSON/NDJSON/bridge 和退出码合同成立。
- 覆盖设计点: 参数互斥、machine 强制非交互、errorKind、exit policy、唯一终态。
- 前置条件: 可注入 runtime 子进程和 host detection。
- 操作 / 触发: 覆盖合法/非法参数、短 JSON、长 NDJSON、bridge、validate invalid。
- 期望结果: 0/2/64/1 映射稳定；TTY 不改变机器协议；零/多/后置终态被拒绝。
- 验证方式: Vitest parser/stream/CLI tests。

### ST-003 统一 init 与 partial recovery

- 关联成功标准: host bootstrap、runtime init 和 landing summary 形成唯一初始化入口。
- 覆盖设计点: BootstrapReport、内部 `cli_init`、Rust outcome 矩阵、recovery hint。
- 前置条件: 临时仓库、可控 host asset 写入和 runtime fixtures。
- 操作 / 触发: 执行 ready、bootstrap partial、runtime failure、fusion degraded/blocked 的 init。
- 期望结果: 机器流唯一终态；outcome 由 Rust 给出；partial 退出 2；已写文件事实不丢失。
- 验证方式: TypeScript orchestration tests + Rust transport/workflow tests。

### ST-004 只读治理命令

- 关联成功标准: `changes/change/validate` 复用 GovernanceService 且语义可区分。
- 覆盖设计点: 单次 evaluation envelope、changeId、not_enabled/not_found/invalid。
- 前置条件: governance fixtures 覆盖未启用、空、合法和阻塞 change。
- 操作 / 触发: 调用三个顶层治理命令。
- 期望结果: envelope 内 summary 与 change 数据一致；无参 validate 被拒绝；validate invalid 退出 2。
- 验证方式: Rust acceptance tests + TS parser/CLI tests。

### ST-005 宿主资产保持 identity 并迁移调用文本

- 关联成功标准: 宿主资产使用一级 CLI，JS tools API 与 skill identity 不变。
- 覆盖设计点: workflow semantics、command assets、host 参数命名。
- 前置条件: Claude/Codex/CodeBuddy bootstrap fixtures。
- 操作 / 触发: 生成各宿主资产并检查公开 exports。
- 期望结果: 调用文本不含旧 namespace/`--term`；`wiki-*`、`/wiki:*`、`wikiInit` 等仍存在。
- 验证方式: bootstrap/asset snapshot tests + export tests。

### ST-006 当前产品文档和代码无旧调用残留

- 关联成功标准: README、Wiki capability、生成器和测试完成迁移。
- 覆盖设计点: 固定扫描模式与排除范围。
- 前置条件: 完成实现与文档同步。
- 操作 / 触发: 执行旧命令扫描脚本/测试。
- 期望结果: 排除 archive/upstream/release/build/deps 后无用户侧旧调用；archive 不被注册。
- 验证方式: workspace test 中的静态扫描 gate。

## 覆盖矩阵

| 成功标准 | 系统测试用例 | 验证方式 |
| --- | --- | --- |
| 四命令默认主路径与一级 router | ST-001 | CLI/E2E |
| query/host 参数与输出协议 | ST-002 | Vitest |
| 统一 init、partial、landing | ST-003 | TS + Rust tests |
| 治理只读命令和 DTO | ST-004 | Rust + TS tests |
| 宿主 identity 保留、调用迁移 | ST-005 | asset tests |
| 当前文档和代码清除旧调用 | ST-006 | scan gate |

## 边界与异常

- `archive/doctor/repair/trace` 必须保持未注册。
- governance not_enabled 不影响普通 status/query/update。
- machine flag 与 help 混用、bridge 用于短动作、host 参数冲突均为 usage error。
- 历史 archive、upstream 和 release 记录不参与迁移断言。

## 验证数据与环境

- Windows PowerShell、Node/Vitest、Rust cargo test。
- 临时 repo fixtures，不依赖真实用户宿主目录或网络。
- 复用 governance fixtures 与可注入 child process mock。

## 未覆盖项

无。

## 参考资料

- `./proposal.md`
- `./design.md`
