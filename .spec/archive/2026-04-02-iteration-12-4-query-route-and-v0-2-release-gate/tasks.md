## 1. Release Contract Switch

- [x] 1.1 调整 `crates/wiki-runtime/src/workflows/release_scope.rs` 与相关 workflow 入口，移除 `SPEC_WIKI_V0_1_INDEX_ONLY` / `index_only` 的公开成功短路，使 `init / update / status` 的正式成功语义统一切到 `v0.2.0 knowledge runtime`
- [x] 1.2 收口 `status` 的公开状态投影与推荐动作，确保 facts/index-only 或 `runtime_incomplete` 不再被包装成 `v0.2.0` 发布成功态
- [x] 1.3 同步更新 `README`、帮助文本、宿主 skill/文案与命令合同测试，确保外部说明只反映当前 12.4 已收口的公开合同，不扩成 13 阶段 surface

## 2. Query Route And Observability

- [x] 2.1 在保持外部 `term-only` 输入稳定的前提下，将 runtime query route 收口为 `index -> knowledge -> page fallback`
- [x] 2.2 固定 `provenance_summary` 的稳定 route tags 为 `index_hit / knowledge_hit / page_fallback`，并确保 `query_trust / recommended_action` 只表达 readiness，不与 provenance 混层
- [x] 2.3 调整恢复态 runtime 的 `status / query` 消费语义，确保恢复态可查询但不会被伪装成 `ready`

## 3. Release Gate Verification

- [x] 3.1 补齐 `storybook` 的 primary gate 验证，覆盖最小正式 knowledge runtime、query route 与 release-ready 断言；`dagger` 下沉为后续参考项，不再作为本轮强制收口条件
- [x] 3.2 固定 `chi + zustand` 为 smoke gate，补 `init / status / update / query` 的脚本或自动化验证，证明公开合同不依赖大样本特例
- [x] 3.3 调整本 change 的验收边界，明确完整项目集 `init` baseline guard 不再属于本轮强制收口，只保留为后续可选参考项

## 4. Contract Hygiene

- [x] 4.1 单独执行一次 `.wiki/02-开发指南/00-代码注释规范.md` 合规检查，并把结果写入本 change 的验收记录
- [x] 4.2 复核实现、测试、报告与文案边界，确保本轮没有新增 capability、没有扩成 13 阶段 lifecycle/public-surface 迁移，且不把 `runtime_incomplete` 重新包装成发布成功态
