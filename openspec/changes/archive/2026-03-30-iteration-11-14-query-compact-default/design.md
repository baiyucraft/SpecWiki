## Context

上一轮 query 输出瘦身只解决了“字段太胖”，没有解决“结果层级太散”。默认外部响应仍要求调用方在多套平行数组之间自己拼接上下文，这对 Agent 并不友好。

## Decision

1. 保持 `run_query()` 内部 rich `QueryReport` 不动
2. 默认 transport 只保留稳定 runtime 字段，加一个 compact `summary`
3. 统一输出 `hits`，按 `page -> symbol -> source -> module -> call_edge` 排序并限长
4. graph 证据不再单独返回大数组，改为压缩后的 `call_edge` hits

## Trade-offs

- 默认 payload 更短，但机器侧原始并列数组不再直接可见
- rich 结构仍在 runtime 内部，后续如果要 debug/verbose，再单独起合同
