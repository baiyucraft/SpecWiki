## Context

当前仓库里，`wiki-runtime` 已经把三层配置面实现成：

- `~/.spec-wiki/config.yaml`
- `.wiki/config.yaml`
- 显式开发模式下才额外叠加的 `wiki.dev.yaml`

同时，测试脚本侧已经存在 `withTemporaryDevConfig` 这类共享 helper，用于把根级 `wiki.dev.yaml` 临时覆盖到目标 repo，并在结束后回滚。

剩余未收口的是两点：

1. 主 spec 仍有旧路径 `.wiki/wiki.steering.yaml` 与“默认读取 `wiki.dev.yaml`”的遗留表述
2. `run-init-debug-trace.mjs` 还保留了一份局部复制的临时 dev config 逻辑，没有复用共享 helper

这两点都属于“合同和验证脚本对齐”的 follow-up，不足以再扩成 CLI 或 query 级 change。

## Goals / Non-Goals

**Goals:**

- 明确 repo 级共享配置正式路径为 `.wiki/config.yaml`
- 明确 user / repo / dev 配置的优先级与显式开发模式边界
- 让 debug trace 脚本复用共享的临时 dev config helper
- 保持本 change 为配置合同与 harness 收口的小边界

**Non-Goals:**

- 不在本 change 内扩张 `wiki-index/query` 合同
- 不在本 change 内推进 `spec-wiki` CLI forwarding 或 e2e 合同
- 不在本 change 内处理 `packages/spec-wiki` workspace 迁移
- 不在本 change 内改动宿主资产生成边界

## Decisions

### 决策 1：共享 steering 路径统一收敛为 `.wiki/config.yaml`

主 spec 必须和现有 runtime 实现对齐：

- repo 内共享配置使用 `.wiki/config.yaml`
- 用户默认配置使用 `~/.spec-wiki/config.yaml`
- `wiki.dev.yaml` 不属于共享配置本体，只是显式开发模式下的本地覆盖层

理由：

- 继续保留 `.wiki/wiki.steering.yaml` 只会制造双路径错觉
- 现有实现和测试已经围绕 `.wiki/config.yaml` 展开，spec 不能继续落后

### 决策 2：`wiki.dev.yaml` 必须显式 gated，而不是默认生效

`wiki.dev.yaml` 只服务本地开发调试，不应该在普通 workflow 中默认生效。正式合同应明确：

- 默认只解析 user + repo 两层
- 只有显式开发模式开启时，才在最后叠加 `wiki.dev.yaml`

理由：

- 否则本地调试文件会悄悄污染正式运行语义
- 这也能解释为什么脚本侧必须通过 helper 显式注入 dev config，而不是假设 runtime 总会读取它

### 决策 3：脚本侧临时 dev config 必须复用共享 helper

`run-init-debug-trace.mjs` 不再自带一份临时拷贝和回滚逻辑，而是复用 `scripts/testing/helpers.mjs` 里的共享 helper。

理由：

- 避免脚本间出现第二套“何时写入 dev config、何时回滚”的语义
- helper 已经持有 `devContext.command(...)` 这类包装逻辑，调试脚本不应再局部拼 command

## Risks / Trade-offs

- [主 spec 与实现继续漂移] -> 用 delta spec 明确路径、优先级和显式开发模式边界
- [脚本侧再长出第二套 dev config 逻辑] -> 统一复用共享 helper
- [把 CLI/query/Agents 边界继续混进本 change] -> proposal/design/tasks 显式排除这些主题

## Migration Plan

1. 为 `wiki-steering-config` 增加 delta spec，收紧路径、优先级与显式开发模式表述
2. 调整 `run-init-debug-trace.mjs`，改为复用 `withTemporaryDevConfig`
3. 检查 `scripts/testing/helpers.mjs` 注释与参数语义，确保与当前脚本消费方式一致