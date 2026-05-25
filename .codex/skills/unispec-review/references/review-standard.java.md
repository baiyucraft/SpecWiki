# Java Review 规范

符合通用规范。本文件只补充 Java 异常、空值、线程池、集合、字符串、资源、分层、日志和测试相关的人工 review 重点。

## 适用范围

- Java 服务、库、CLI、测试、构建配置和框架集成变更。
- Spring / Jakarta / MyBatis / JPA 等框架代码需结合项目约定和运行时契约审查。

## 必须同时读取

- `references/review-standard.md`
- 项目的 Java 版本、框架、分层约定、事务边界、线程池策略、测试框架和依赖管理事实。

## Blocking / P0

- 异常必须具体捕获、记录上下文或包装抛出；不能空 catch、吞异常、捕获 `Throwable` 或无理由捕获宽泛 `Exception`。
- 返回集合应避免 null；Optional 仅用于返回值，不应作为字段 / 参数；链式调用必须处理 null contract。
- 线程池必须有界、命名并配置拒绝策略；共享可变状态必须同步；禁止无界队列和不受控调度任务。
- 集合已知大小应指定容量；遍历时修改集合、`Arrays.asList()` 增删、包装类型 `==`、`subList` 视图误用必须避免行为错误。
- 循环字符串拼接、`String.valueOf(null)`、浮点比较、switch 缺少 default 等常见陷阱不能影响正确性。
- InputStream / OutputStream / Connection / Statement / ResultSet / 事务必须 try-with-resources 或可靠 finally 关闭 / 回滚。
- Controller / Service / Repository、DTO / Entity / Domain 边界和事务位置不能混乱到影响业务语义。
- 测试必须覆盖异常路径、事务边界、并发风险、集合边界和外部依赖 mock。

## Non-blocking / P1/P2

- 日志没有占位符、缺少业务标识或异常对象不是最后参数，但未造成当前排障阻断。
- 方法 / 类职责偏大、Stream / Lambda 过度复杂、Lombok 使用过宽或字段注入影响可测试性。
- 依赖版本、SNAPSHOT、重复版本或无人维护库存在维护风险但未直接阻断当前 change。

## 建议工具化

- 配置 formatter、Checkstyle / Spotless、Error Prone、SpotBugs、PMD、JUnit 5 / Mockito、coverage 和依赖漏洞扫描。
- 导入排序、格式、简单空指针模式、基础静态分析、未使用代码和覆盖率阈值优先交给工具。

## 非目标

- 不因框架注解风格差异给 blocking，除非破坏事务、依赖注入、序列化或运行时行为。
- 不人工替代格式化、Checkstyle 或静态分析工具。

## 常见误报

- 把所有字段注入都判为 blocking，而没有说明测试、生命周期或不可变性风险。
- 把所有 Stream 使用都判为问题，而没有说明复杂度或性能风险。
- 只因缺少 Javadoc 就阻塞非公共 API 变更。
