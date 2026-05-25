## 1. 关键源码去噪

- [x] 1.1 在 `crates/wiki-core/src/repo/` 和 `crates/wiki-core/src/generation/context.rs` 中定义低信号文件过滤与高信号源码排序规则
- [x] 1.2 调整模块页摘要输入，使 `关键源码` 不再包含日志、锁文件、纯文档和低价值配置噪声

## 2. 模块关系补强

- [x] 2.1 扩展 deterministic 关系提取，补强 manifest、入口、导入别名和典型前后端/基础设施目录线索
- [x] 2.2 补充混合仓库 fixture 与测试，覆盖更真实的模块依赖和被依赖关系

## 3. Query 与页面输出对齐

- [x] 3.1 收紧 `query` 的结构命中原因与 provenance，使其与模块页使用同一组高信号结构事实
- [x] 3.2 验证 Markdown 回退仍然可用，但不再主导命中说明

## 4. aLocal 对照验收

- [x] 4.1 运行 `'{\"action\":\"init\",\"repoRoot\":\"E:\\\\project\\\\aLocal\"}' | .\\dist\\core\\wiki-core.exe --json`，将生成出的 `.wiki` 快照写入 `tmp/` 下的对比目录
- [x] 4.2 将 `aLocal` 快照与 `tmp/reference-zh` 做结构与内容密度对照，根据差异回调实现与测试

## 5. 文档与回归验证

- [x] 5.1 更新相关测试、fixture 或文档，明确本轮收口后的迭代 1 验收口径
