# Go Review 规范

符合通用规范。本文件只补充 Go 错误语义、context、接口、并发、资源生命周期、内存和测试相关的人工 review 重点。

## 适用范围

- Go 服务、CLI、库、脚本、测试、模块配置和构建相关变更。
- `go.mod`、`go.sum` 或 `.go` 文件变化本身不足以替代 artifact / diff 证据；仍需结合 change 范围判断。

## 必须同时读取

- `references/review-standard.md`
- 项目的 Go 版本、模块边界、入口目录、依赖约束、测试命令和并发 / IO 设计事实。

## Blocking / P0

- error 必须显式处理并包装上下文；不能用 `_` 丢弃关键错误，不能靠字符串比较判断错误类型。
- `context.Context` 应作为跨 IO / RPC / DB / 长任务调用的第一个参数；不能传 nil context，超时 / cancel 后必须释放。
- goroutine 必须有 WaitGroup / errgroup / channel / context 管理；循环变量捕获、panic recover、channel 关闭责任和 goroutine 泄漏必须可解释。
- 共享 map / slice / struct 状态必须同步；channel 关闭只能由发送方负责；不能用 `time.Sleep` 等待并发完成。
- 文件、HTTP body、DB rows / tx、锁、临时资源必须在所有路径下关闭、回滚或释放。
- JSON 大数、nil slice/map、零值语义、类型断言 comma-ok、defer 在循环中延迟释放等常见陷阱不能影响正确性。
- 测试必须覆盖 error path、context cancel / timeout、并发路径、资源释放和外部依赖 mock。

## Non-blocking / P1/P2

- 接口过大、接口定义在实现方、返回接口、泛化 `interface{}` / `any`、包名过泛或模块边界模糊。
- 已知大小 slice / map 未预分配、大量字符串循环拼接、小对象指针滥用或高频临时对象分配。
- 日志缺少结构化字段、业务标识或错误上下文，但未影响当前功能。

## 建议工具化

- 配置 gofmt、goimports、go vet、staticcheck、race detector、coverage、govulncheck 和模块依赖检查。
- 格式、导入排序、基础 vet/staticcheck、包名格式和简单未使用代码优先交给工具。

## 非目标

- 不人工替代 gofmt / go vet / staticcheck。
- 不机械要求所有依赖都抽象接口；只在测试边界、依赖倒置或稳定契约需要时提出。

## 常见误报

- 把短小函数的裸 return 一律判错。
- 不区分应用初始化 panic 与业务流程 panic。
- 只因没有预分配小切片就给 blocking。
