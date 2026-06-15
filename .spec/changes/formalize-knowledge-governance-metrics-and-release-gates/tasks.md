## 1. Metrics And Judgment Contract

- [ ] 1.1 定义 knowledge-level metrics 聚合对象，确保 `stale / blocked / conflict / missing provenance / degraded answer` 只从正式 runtime signals 聚合
- [ ] 1.2 定义 `metrics / blockers / waivers / release judgment` 的正式对象与最小 blocker 集
- [ ] 1.3 复用 `12-9-7` 的 summary contract，新增 governance aggregation 与 evidence manifest，而不是重造第二套脚本输出协议

## 2. Verification And Reporting

- [ ] 2.1 更新 `workflow-verification` 相关脚本与输出，使其在既有 gate surface 上补充 governance metrics 与 blocker judgment
- [ ] 2.2 更新 `reference-fidelity-reporting`，让 `storybook + dagger` 样本报告带上 governance summary 与 evidence linkage
- [ ] 2.3 让 `19` 项目 batch baseline 进入 release evidence，但保持它与 primary sample 的职责分离

## 3. Acceptance And Release Evidence

- [ ] 3.1 补测试，验证 metrics 聚合不会依赖 markdown/report 文本反推
- [ ] 3.2 补 blocker / waiver 场景测试，覆盖 primary sample 失败、batch baseline 异常与有条件放行
- [ ] 3.3 运行相关脚本与样本验证，确认 release judgment 能区分“脚本通过”与“knowledge system 可放行”

## 4. Docs And Hygiene

- [ ] 4.1 更新 release closeout、quality gates 与 governance 相关文档，明确本轮只做 governance aggregation，不做 dashboard 或后台
- [ ] 4.2 检查实现注释是否符合 [.wiki/02-开发指南/00-代码注释规范.md](/E:/project/!byAI/spec-wiki/.wiki/02-开发指南/00-代码注释规范.md)，删除把 metrics 解释成脚本私有统计的误导性注释
- [ ] 4.3 复核 UniSpec 与实现边界一致，确保本轮不改写 `knowledge-quality-gates` transport shape
