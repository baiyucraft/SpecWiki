## 1. Spec 收口

- [x] 1.1 新增 `declared-conflict-artifacts` spec，固定 conflict formal object、truth boundary 与 consumer。
- [x] 1.2 更新 `knowledge-runtime-artifacts`、`knowledge-runtime-health-signals`、`knowledge-first-update`，明确 conflict artifact contract。
- [x] 1.3 复核本轮非目标：不碰 override / accept / reject，不碰 query / answer，不做 declared vs code reality 语义冲突。

## 2. Model 与 Artifact 实现

- [x] 2.1 为 deterministic declared conflicts 引入 `KnowledgeConflictRecord`、kind/status/severity 与 snapshot validation。
- [x] 2.2 在 artifact persist / load / restore 主链中补 conflict artifacts 持久化、读取与校验。
- [x] 2.3 保持 conflict artifact 只从 formal declared snapshot 推导，不从页面正文或 cache 猜测。

## 3. Runtime 投影

- [x] 3.1 让 `sync` 在合法 declared writeback 后重算 conflict artifacts，并保持 `illegal_drift` 优先。
- [x] 3.2 让 `status` / health signals 正式消费 open conflicts，并输出稳定 `review` 建议。
- [x] 3.3 保持本轮只做只读 conflict projection，不引入 decision / override / arbitration workflow。

## 4. 测试与验证

- [x] 4.1 补 declared conflict model / artifact roundtrip / restore validation 测试。
- [x] 4.2 补 `sync` / `status` / health 对 conflict artifact 的测试，覆盖 conflict 生成、冲突消失与 illegal drift 优先。
- [x] 4.3 跑 `cargo test -p wiki-runtime`。
- [x] 4.4 跑 `node scripts/run-test-projects.mjs --jobs 1 --timeout-minutes 90 storybook dagger`。
- [x] 4.5 单独检查 [.wiki/02-开发指南/00-代码注释规范.md](/E:/project/!byAI/spec-wiki/.wiki/02-开发指南/00-代码注释规范.md) 合规性，并同步更新 tasks 状态。
