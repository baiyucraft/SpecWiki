# refactor-specwiki-around-contract-closure-archive-dry-run-manifest 系统测试用例

## 用例总览

覆盖 validate-first、dry-run、manifest、digest、apply 终态、失败恢复、并发锁、路径安全、Wiki 隔离和 CLI/distribution 合同。真实文件系统与真实 CLI 优先使用集成验证。

## 系统测试用例

### ST-001 validate-first 失败时零业务写入

- 关联成功标准: 默认先 validate；失败不移动目录、不修改 parent、不写 Wiki。
- 覆盖设计点: preflight、`archive_not_ready`。
- 前置条件: required artifact 缺失或 review/verification gate 未通过。
- 操作 / 触发: 执行默认 archive、`--dry-run` 和 `--apply`。
- 期望结果: 返回 typed issue；source、parent、target、`.wiki/**` 不变；dry-run 和失败 apply 不创建 operation plan。
- 验证方式: Rust TempDir 前后递归 hash + CLI JSON/退出码。

### ST-002 默认 dry-run 输出完整只读 manifest

- 关联成功标准: dry-run 不移动目录且输出 readiness/manifest。
- 覆盖设计点: 默认 dry-run、`persisted=false`、`resumable=false`。
- 前置条件: ready child、active parent、唯一 split marker。
- 操作 / 触发: 执行 `archive <id>` 与 `archive <id> --dry-run`。
- 期望结果: 两者输出等价；manifest 含 schema/policy/algorithm version、operation id、UTC target、source/target、validation/readiness、artifact hashes、parent diff、digest、steps、Wiki refs；业务树和 `.spec/.runtime` 不变。
- 验证方式: `archive_workflows` 集成测试。

### ST-003 precondition 变化在首个业务写前被拒绝

- 关联成功标准: apply 前 digest 重验且不一致时零业务写入。
- 覆盖设计点: canonical digest、TOCTOU。
- 前置条件: 先生成 plan，再修改 source、child meta、parent meta、parent split 或创建 target。
- 操作 / 触发: 使用原 plan 执行 apply。
- 期望结果: 返回 `archive_precondition_changed` 或确定性 conflict；不覆盖 target、不移动 source、不修改 parent、不写 Wiki；提示重新 dry-run。只改 mtime、无关源码或 Wiki 不改变 digest。
- 验证方式: digest unit + apply integration。

### ST-004 ready apply 达到完整终态

- 关联成功标准: apply 成功、parent 同步、最终 live verification。
- 覆盖设计点: apply 状态机、固定 UTC target。
- 前置条件: ready child，固定 clock，target 不存在。
- 操作 / 触发: `archive <id> --apply`。
- 期望结果: active source 消失；target 为 `.spec/archive/YYYY-MM-DD-<id>` 且内容完整；child artifact 字节不变；parent meta/split 一致；plan/checkpoints/result 可审计；live GovernanceService 通过；outcome `completed`，退出码 0。
- 验证方式: Rust workflow integration。

### ST-005 target/source/parent 冲突分类稳定

- 关联成功标准: 冲突不覆盖、不合并、不自动改名。
- 覆盖设计点: no-overwrite、active/archive 双占、parent gate、重复归档。
- 前置条件: source 缺失、target 已存在、active/archive 双占、parent 缺失/已归档、child-parent 不一致、重复 operation。
- 操作 / 触发: 执行 plan/apply/resume。
- 期望结果: 返回稳定 typed error；已完成 operation 为 `already_completed`；未完成重复请求为 `archive_recovery_required` 并给出已有 operation id；不产生第二 operation。
- 验证方式: 表驱动 filesystem integration。

### ST-006 parent 同步只接受唯一合法 marker

- 关联成功标准: parent meta 和 split 同步。
- 覆盖设计点: YAML unknown fields、split 唯一 section/marker。
- 前置条件: YAML 含未知字段；split child section/marker 分别为 0、1、多条；marker 非预期。
- 操作 / 触发: 执行 plan/apply。
- 期望结果: 唯一合法输入保留未知字段语义并生成 before/after hash；0/多匹配和格式漂移均 conflict；不做宽泛 Markdown 替换。
- 验证方式: parent sync integration。

### ST-007 mutation 边界失败进入 recovery_required

- 关联成功标准: 失败可识别、可重试、不误报成功。
- 覆盖设计点: source rename、parent meta、parent split、final verify 故障注入。
- 前置条件: ready operation，注入单个 mutation 或 checkpoint 失败点。
- 操作 / 触发: 执行 apply。
- 期望结果: outcome `recovery_required`；包含 operation id、failure step、recovery hint；不报告 completed。
- 验证方式: 真实文件操作 + test-only fault injector。

### ST-008 checkpoint 缺失窗口按文件系统事实恢复

- 关联成功标准: partial operation 可幂等 resume。
- 覆盖设计点: source/parent mutation 成功但 checkpoint 未落盘的 reconcile。
- 前置条件: source 已移动、parent meta 已替换或 split 已替换但 checkpoint 缺失。
- 操作 / 触发: `archive <id> --resume <operation-id>`。
- 期望结果: 按 before/after hash 前滚；不重复 rename/覆盖；既非 before 也非 after 时 conflict。
- 验证方式: recovery integration。

### ST-009 manifest 损坏时 fail closed

- 关联成功标准: 失败状态可信且不猜测执行。
- 覆盖设计点: schema、plan、checkpoint、path、result 校验。
- 前置条件: 未知 schema、缺失 plan、重复/倒序 checkpoint、非法 step、change-id 不匹配、路径逃逸、result/FS 不一致。
- 操作 / 触发: 执行 resume。
- 期望结果: `archive_manifest_invalid` 或 conflict；不继续 mutation、不自动修复、不覆盖未知内容。
- 验证方式: operation storage integration。

### ST-010 部分完成后 fresh apply 不创建第二 operation

- 关联成功标准: 失败可恢复。
- 覆盖设计点: operation discovery、fresh/resume 互斥。
- 前置条件: 唯一未完成 operation，OS lock 已释放。
- 操作 / 触发: 再次执行 `archive <id> --apply`。
- 期望结果: 返回 `archive_recovery_required`、已有 operation id 和 `--resume` 提示；不创建第二 operation，不新增业务 mutation。
- 验证方式: operation discovery integration。

### ST-011 completed resume 使用固定 target 且幂等

- 关联成功标准: 重试确定性。
- 覆盖设计点: 固定 clock/target、already-completed。
- 前置条件: 完成 operation 后切换 UTC 日期。
- 操作 / 触发: resume completed operation。
- 期望结果: 使用 plan 保存的 target/date；返回成功；source、target、parent、checkpoint 不发生破坏性变化。
- 验证方式: 注入 clock 的 workflow integration。

### ST-012 sibling/fresh-resume 并发受完整 mutation-set 锁保护

- 关联成功标准: parent 无 lost update，操作互斥。
- 覆盖设计点: child+parent 完整 OS advisory lock。
- 前置条件: 两个 sibling 共用 parent，或 fresh apply 与 resume 并发。
- 操作 / 触发: 并发启动两个 operation。
- 期望结果: 只有一个 writer；竞争者返回 `archive_locked` 和 operation 引用；无双 target、parent lost update 或交错 checkpoint。
- 验证方式: 真实线程/进程 integration。

### ST-013 source tree 路径安全

- 关联成功标准: 路径安全且不跨 volume copy。
- 覆盖设计点: symlink、junction/reparse、special file、path escape。
- 前置条件: source tree 注入平台可用危险条目和 repo 外路径。
- 操作 / 触发: 执行 plan/apply。
- 期望结果: 业务写入前返回 conflict；manifest 只含 repo-relative path；repo 外文件不受影响；跨 volume 不 fallback copy+delete。
- 验证方式: Unix symlink、Windows-only reparse/junction；能力不可用时记录 evidence gap。

### ST-014 Wiki 严格隔离但 issue/ref 可见

- 关联成功标准: archive 不调用 Wiki 写流程。
- 覆盖设计点: Wiki warning/non-blocking、evidence refs。
- 前置条件: ready、not-ready、apply 成功和 recovery operation，记录 `.wiki/**` tree hash。
- 操作 / 触发: dry-run/apply/resume。
- 期望结果: `.wiki/**` hash 不变；不调用 sync/update/rebuild；报告保留 `wiki_sync_issues` 和 evidence refs；Wiki warning 不阻断正确 archive。
- 验证方式: runtime integration + call counters。

### ST-015 CLI help、参数、human/JSON 和退出码

- 关联成功标准: CLI 输出和退出码稳定。
- 覆盖设计点: advanced help、archive modes、typed errors。
- 前置条件: source CLI 与 built/staged package。
- 操作 / 触发: 默认 help、`--help-all`、command help、默认 archive、`--dry-run`、`--apply`、`--resume` 和非法组合。
- 期望结果: 默认 help 不显示 archive；help-all/command help 显示；互斥/change-id mismatch 返回 64；human/JSON 表达 mode/outcome/target/failure/recovery；成功 0、domain rejection 2、manifest/internal 1。
- 验证方式: Vitest + distribution tests。

### ST-016 source/staged distribution parity

- 关联成功标准: 分发 CLI 与源码合同一致。
- 覆盖设计点: bundled CLI、help parity、archive forwarding。
- 前置条件: staged package 已构建。
- 操作 / 触发: source/staged 分别执行 help-all 和 archive dry-run smoke。
- 期望结果: help、参数、JSON envelope 和退出码一致；默认 help 不扩大；staged 不写 Wiki。
- 验证方式: `scripts/tests/distribution.test.ts`。

## 覆盖矩阵

| 成功标准 | 系统测试用例 | 验证方式 |
| --- | --- | --- |
| 默认先 validate，失败零写 | ST-001 | Rust/CLI integration |
| dry-run 只读并输出 readiness/diff | ST-002、ST-014 | Rust integration |
| manifest 字段完整且可审计 | ST-002、ST-009 | storage/workflow |
| digest 限定输入并防 TOCTOU | ST-003、ST-013 | unit + integration |
| target 命名和异常闭集 | ST-005、ST-010、ST-011 | filesystem integration |
| apply 完整终态 | ST-004、ST-006 | workflow integration |
| 失败状态、恢复和幂等 | ST-007、ST-008、ST-009、ST-010、ST-011 | fault/integration |
| Wiki 只输出 issue/refs | ST-014 | tree hash + call counter |
| human/JSON/退出码/分发 | ST-015、ST-016 | Vitest/distribution |

## 边界与异常

- 不模拟硬件掉电、控制器缓存丢失、杀毒软件和磁盘满；使用进程级 fault injection 替代。
- 跨 volume 只验证拒绝，不实现 copy+delete fallback。
- YAML 注释和原始排版不在承诺内，只验证未知字段语义保留。
- Windows reparse/junction 权限不足时记录 capability evidence gap，不静默通过。
- 不遵守 archive lock 协议的外部进程不在保护范围内。

## 验证数据与环境

- Rust `tempfile::TempDir` 动态构造 standalone、parent/child、sibling、双占和 operation 目录。
- 注入 UTC clock、operation id、fault point 和 lock barrier。
- Unix symlink 与 Windows junction/reparse 分平台测试。
- 重点命令：`cargo test -p wiki-model --test archive_contract`、`cargo test -p wiki-runtime --test archive_planning`、`archive_storage`、`archive_workflows`、`archive_recovery`、`archive_concurrency`、`archive_path_safety`、`cargo test -p wiki-runtime --test acceptance command_contract`、`pnpm --filter spec-wiki test`、`pnpm run lint`、`cargo clippy --workspace --all-targets -- -D warnings`、`pnpm run test`。

## 未覆盖项

- 真正断电、硬件缓存丢失和跨 volume 成功迁移不在本 change 覆盖。
- 不验证 YAML 注释/空白格式保持。

## 参考资料

- `./proposal.md`
- `./design.md`
- `../../../.wiki/02-开发指南/01-测试与验收.md`
- `../../../.wiki/02-开发指南/02-脚本与工作流.md`
- `../../../crates/wiki-runtime/tests/governance_workflows.rs`
- `../../../crates/wiki-runtime/tests/governance_policy.rs`
