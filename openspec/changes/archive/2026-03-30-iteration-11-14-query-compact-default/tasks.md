## 1. OpenSpec

- [x] 1.1 为 `repo-wiki-runtime` 与 `wiki-bm25-query` 补充默认 compact query payload 要求

## 2. Runtime Query Payload

- [x] 2.1 将默认 transport payload 改为 `summary + hits`
- [x] 2.2 保留稳定顶层字段，移除默认并列 `matched_* / matches` 数组
- [x] 2.3 把页面、符号、源码、模块、调用边压平为统一 `hits`

## 3. 测试与脚本

- [x] 3.1 更新 acceptance 与 e2e 断言
- [x] 3.2 更新 lifecycle 验证脚本的 query 断言
- [x] 3.3 检查本轮新增注释与命名是否符合 `COMMENTING.md`
