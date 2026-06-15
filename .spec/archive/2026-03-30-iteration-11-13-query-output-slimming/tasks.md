## 1. UniSpec

- [x] 1.1 为 `repo-wiki-runtime` 与 `wiki-bm25-query` 补充 query 默认外部 payload 瘦身要求
- [x] 1.2 完成 design 评审并记录本轮不引入 verbose/debug 视图的边界

## 2. Runtime Query Payload

- [x] 2.1 为 query 增加 transport 外部瘦投影，保留内部 `run_query` 厚结构
- [x] 2.2 收紧默认外部 `matched_symbols`、`matched_symbol_edges`、`matched_sources`、`matched_modules` 和 `matches` 字段集，移除内部 ID/score/confidence/hop 字段，但保留必要 provenance
- [x] 2.3 对空扩展数组使用更薄的默认序列化策略，减少默认 query payload 噪音

## 3. 测试与注释

- [x] 3.1 更新 runtime/transport/CLI/e2e 测试，覆盖默认 query payload 字段瘦身、内部 rich 结构保留与 provenance 负向断言
- [x] 3.2 检查本轮新增和修改注释，确保符合 `.wiki/02-开发指南/00-代码注释规范.md`
