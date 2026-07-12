# refactor-specwiki-around-contract-closure-cli-product-surface

## 问题

当前 SpecWiki 的用户侧 CLI 产品面仍未与已经稳定的 runtime 和 governance 合同对齐：

- TypeScript CLI 同时存在顶层 `init` 与 `wiki <action>` 两套入口，用户需要理解内部 namespace 才能完成日常操作。
- 顶层 `init` 目前只负责宿主资产安装，没有形成 host bootstrap、runtime init 和 landing state 的单一初始化体验。
- runtime transport 已支持 Wiki 主路径动作，但现有只读 `GovernanceService` 尚未通过顶层 `changes / change / validate <change-id>` 暴露。
- 默认人类输出、显式机器输出和 bridge stdio 的边界不完整，调用方难以稳定判断 JSON、NDJSON、终态和退出码。
- 宿主资产、当前文档和测试仍生成或断言 `spec-wiki wiki ...`、`--tool`、`--tools` 等旧产品面。
- 阶段性设计包含 workspace validate、archive、doctor、repair、trace 等尚未闭合的能力；若在本 child 一并注册，会把产品面收口扩大为新功能建设。

本 change 要解决的真实问题是：在不复制治理规则、不提前实现 archive 的前提下，将已经存在的 Wiki runtime 与只读 governance 能力映射为一致、可解释、机器协议稳定的一级 CLI 产品面。

## 目标

- 将默认主路径收敛为顶层 `init / status / query / update`，Quick Start 和默认 help 只突出这四个用户意图。
- 在完整 help 中提供顶层 `sync / rebuild / changes / change / validate <change-id>`；`advanced` 只作为 help 分组，不成为 namespace。
- 通过薄 Rust transport 将 `changes / change / validate <change-id>` 路由到现有 `GovernanceService`，不在 CLI 或 TypeScript 层复制治理规则。
- 统一 `init` 的 host bootstrap、runtime init 和 landing summary，并明确各阶段状态、部分成功、恢复提示和退出码责任。
- 默认输出面向人类的解释；显式 `--json` 保留机器协议，短流程使用单 JSON，长流程保持 NDJSON 事件顺序和唯一终态。
- `--bridge-stdio` 强制进入机器模式，协议不因 TTY 与否发生变化。
- 将宿主选择参数统一为 `--host / --hosts`，删除用户侧 `--tool / --tools` 兼容入口。
- 更新宿主资产生成器、当前产品文档和测试中的旧 CLI 调用文本，同时保留不同产品层已有的 JS tools API 与宿主 skill identity。

## 非目标

- 不注册、占位或伪装实现 `archive`；archive dry-run、manifest、apply 和 recovery 属于后续 `refactor-specwiki-around-contract-closure-archive-dry-run-manifest`。
- 不提供无参 workspace validate；本 change 的 `validate` 只验证显式指定的 change-id。
- 不实现 `doctor / repair / trace`，也不把 `advanced` 变成新的二级命令入口。
- 不增加 `codebase-memory-mcp` 式代码索引、memory、MCP 或检索能力，保持现有 index/knowledge/query 设计。
- 不删除或重命名 `wikiInit / wikiStatus / wikiUpdate / wikiQuery / wikiSync / wikiRebuild` 等 JS tools API。
- 不全局重命名 `wiki-*` skill identity 或 Claude `/wiki:*` identity；只替换宿主资产内实际执行的旧 CLI 调用文本。
- 不为旧 `spec-wiki wiki ...` namespace、`--tool / --tools` 或 `query --term` 保留兼容 alias。
- 不承诺跨 host bootstrap 与 runtime init 的事务回滚；失败恢复只承诺幂等重试和明确 recovery hint。
- 不修改 archive 历史记录、upstream 参考库、历史 release notes、构建产物或依赖目录中的旧命令文本。

## 成功标准

- 默认 help 和 Quick Start 只突出 `spec-wiki init / status / query <term> / update`，完整 help 以分组方式展示其余已实现一级命令。
- CLI 不注册 `wiki`、`governance`、`advanced` 或 `archive` namespace/命令；旧 namespace 不保留兼容转发。
- `query` 的公开输入只接受位置参数 `<term>`，不再接受 `--term`。
- `changes / change / validate <change-id>` 复用 `GovernanceService` 的事实和判断；CLI renderer 不重算 readiness、blocking issue 或 recommended action。
- 默认 human renderer 能解释 Rust DTO；`--json` 对短流程保持单 JSON，对流式动作保持 NDJSON 顺序与唯一终态。
- `--bridge-stdio` 始终使用机器协议，TTY 检测不改变 stdout 协议。
- 统一 `init` 能报告 host bootstrap、runtime init 和 landing summary 的阶段结果，并区分完整成功、部分成功、用法错误与内部失败。
- 治理未启用、change 列表为空、change 不存在和 change validate 失败能保持可区分的产品语义。
- 宿主初始化使用 `--host / --hosts`；`--tool / --tools` 不再被接受。
- 当前 README、Wiki capability baseline、host asset 生成器和测试完成一级命令迁移；固定扫描在排除历史与依赖范围后不再发现用户侧旧命令调用。
- 现有 JS tools API、`wiki-*` skills 和 Claude `/wiki:*` identity 继续存在，且只更新其需要执行的 CLI 文本。

## 影响范围

- `packages/spec-wiki/src/cli.ts` 及 CLI tests：一级 command router、参数、help 分组、human/machine output 和退出状态。
- `packages/spec-wiki/src/runtime/**`：新增 action 的 forwarding、machine protocol 保持和 human renderer 消费边界。
- `crates/wiki-runtime/src/transport/**`：只读 governance action、change-id 输入与响应 DTO 的薄 transport。
- `crates/wiki-runtime/src/workflows/governance.rs`：作为 `changes / change / validate` 的既有规则来源，不在 CLI 层复制判断。
- `packages/spec-wiki/src/agents/**`、`.agents`、`.codex` 及其它宿主资产生成路径：宿主参数命名和一级 CLI 调用文本。
- 当前 README、`.wiki` 对外方法与 capability baseline、相关脚本和测试 fixtures：同步已实现产品合同。
- 全仓旧命令扫描范围：`spec-wiki wiki`、`spec-wiki governance`、`spec-wiki init --tool`、`spec-wiki init --tools`；排除 `.spec/archive/**`、`.upstream/**`、历史 release notes、构建产物和依赖目录。

## 交付形态

single-change

这是 parent `refactor-specwiki-around-contract-closure` 下顺序第 7 个 child。它依赖已归档的 query route/readiness、projection writeback 和 governance isolation 合同，只负责把这些既有能力收敛为 CLI 产品面；后续第 8 个 child 再交付 archive 写事务。

## 风险

- `init` 组合宿主写入与 runtime 长流程后，如果终态所有权不唯一，机器模式可能产生多个相互冲突的终态。
- 退出码 `2` 若笼统映射所有 blocked/invalid/degraded 状态，会混淆观察性命令、验证失败和部分执行；design 必须按命令闭合。
- 当前 change 列表返回值无法天然区分“治理未启用”和“已启用但没有 active change”，transport 产品语义可能不足。
- human renderer、landing 聚合或 CLI 退出码分类若暗中重新推导 readiness/action，会形成第二套治理或 runtime 规则。
- 删除旧 namespace 和参数会同时影响人工调用、宿主资产和测试；迁移范围遗漏会导致安装后命令不可用。
- 阶段性设计包含超出本 child 的目标态，若实现直接照搬全部命令表，会提前暴露未实现或不安全能力。
- 统一 init 跨越 host bootstrap 与 runtime init，但本 change 不提供跨阶段事务回滚；失败提示必须足以支持幂等重试。

## 未知项

- 一级 router 的最终参数语法、默认 help 与完整 help 的具体入口形式。
- `changes / change / validate` 的 transport action、change-id 输入和响应 DTO 如何表达 governance readiness。
- 默认 human renderer 的逐 DTO 映射，以及 stdout、stderr、progress 和 error 的归属规则。
- 短 JSON、长 NDJSON 与 bridge session 的完整协议矩阵。
- unified init 的 phase 状态模型、事件适配和唯一终态所有权。
- `status` 观察到 blocked、`validate` 返回 invalid、`query` 降级、`init` partial success、usage error 和内部失败的逐命令退出码。
- `--host / --hosts` 的选择规则、无宿主发现结果和非交互行为。
- 固定旧命令扫描的实现方式、排除范围和 reviewer gate 测试落点。

## 参考资料

- `../refactor-specwiki-around-contract-closure/split.md`：来源为当前 parent program；目标落点是本 child 的顺序、依赖、CLI 产品面和 archive 后置边界；采用方式为直接约束。
- `../../../.docs/design/specwiki-contract-closure.md`：来源为新版合同收口草案；目标落点是四命令默认主路径、landing state、governance 非阻断和机器可解释输出；采用方式为直接约束并按已实现能力收窄。
- `../../../.docs/design/specwiki-cli-unification.md`：来源为 CLI 阶段性设计；目标落点是一级命令、统一 init、help 分组、宿主参数和文档迁移；采用方式为改写，删除 workspace validate、archive 和未闭合维护命令。
- `../../../.spec/archive/2026-07-10-refactor-specwiki-around-contract-closure-governance-isolation/`：来源为已归档 governance child；目标落点是只读 `GovernanceService`、readiness、list/inspect/validate 合同；采用方式为复用既有合同并增加薄 transport。
- `../../../.upstream/codegraph/src/bin/codegraph.ts`：来源为本地 upstream CodeGraph CLI（一级命令见第 417、628、690、815 行，命令级 `--json` 见第 692、820 行）；目标落点是 TypeScript CLI router、help 和 machine-output flag；采用方式为仅借鉴，不迁移其索引、MCP、命令全集或源码实现。
