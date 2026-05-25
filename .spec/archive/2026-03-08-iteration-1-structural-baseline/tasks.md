## 1. 递归模块树与结构化解析

- [x] 1.1 在 `crates/wiki-core/src/repo/` 中收口递归 `ModuleTree` 构建，补齐稳定发现顺序、稳定 ID、根模块兜底和 `child_ids` 回填
- [x] 1.2 补强 workspace、manifest、入口、源码别名和依赖线索的归一化逻辑，只覆盖会直接影响建树正确性的解析能力
- [x] 1.3 为 monorepo、混合目录、非 workspace 和基础设施目录补齐嵌套模块 fixture 与模块树测试

## 2. 层级化页面规划与 runtime 映射

- [x] 2.1 调整 `Context Builder` 和 `Page Planner`，让页面父子关系、页面标识和页面路径完全由递归模块树导出
- [x] 2.2 收口模块页路径生成规则，处理模块重名、非法路径字符和 Windows 落盘冲突
- [x] 2.3 扩展 metadata 和页面状态导出，使其稳定表达模块层级、页面层级、页面来源和基础 provenance

## 3. 结构优先 Query v1

- [x] 3.1 为页面、模块、源码和关系建立结构化查询输入，明确命中对象与命中原因模型
- [x] 3.2 调整 `query` 工作流，切换到结构优先检索并保留 Markdown 回退路径
- [x] 3.3 补齐 `query` 测试，覆盖结构化命中、Markdown 回退和空结果三类场景

## 4. Baseline 验收夹具与整体验证

- [x] 4.1 建立面向中型本地代码目录的 baseline 验收 fixture，覆盖层级模块、跨模块关系和基础设施目录
- [x] 4.2 增加 `init -> metadata/cache -> query` 的集成或端到端验证，断言 `.wiki` 页面结构和 metadata 层级输出满足 baseline 预期
- [x] 4.3 更新相关文档与 UniSpec 说明，明确“Deterministic Structural Baseline” 的完成口径和夹具使用方式
