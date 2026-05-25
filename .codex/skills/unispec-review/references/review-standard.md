# Review 通用规范

用于 `unispec-review` 和 `unispec-code-reviewer` 执行整个 change 的最终人工 review。通用规范覆盖所有 review domain；domain 专项规范只补充本文件没有覆盖的人工审查重点。

## 适用范围

- 所有 UniSpec change 的最终 full review 或 partial review。
- 所有语言、框架、CLI、配置、文档生成逻辑和测试变更。
- 第三方库源码、明确标注的生成代码和未参与本 change 的历史代码默认不审查；只有它们被当前 change 修改、包装、配置或影响运行风险时才进入范围。

## 必须同时读取

- `.spec/changes/<change-id>/proposal.md`
- `.spec/changes/<change-id>/design.md`
- `.spec/changes/<change-id>/system-tests.md`
- `.spec/changes/<change-id>/tasks.md`
- `.spec/changes/<change-id>/meta.yaml`
- 代码 diff、测试证据、验证命令输出和本次选中的 domain standards。

## Blocking / P0

- 安全风险：注入、XSS、路径穿越、命令执行、认证授权绕过、敏感信息泄露、不安全反序列化或危险 API 使用。
- 逻辑错误：空值、边界条件、数组越界、错误分支遗漏、状态迁移错误、幂等性破坏或失败后数据不一致。
- Artifact 不一致：实现偏离 proposal 目标 / 非目标 / 成功标准，偏离 design 关键决策，或 tasks / system-tests 无法追溯到真实代码与验证证据。
- API / 架构不一致：接口路径、方法、参数、响应模型、错误码、分层边界、依赖方向或数据模型与已确认设计冲突。
- 资源与并发风险：文件、连接、事务、锁、goroutine / thread / async task、定时器或订阅没有释放、取消、回滚或等待机制。
- 测试缺口：成功标准、关键失败路径、安全边界、数据迁移、并发或外部依赖没有自动测试、手工验证或明确替代证据。

## Non-blocking / P1/P2

- 性能风险：N+1 查询、重复计算、无界队列、大数据一次性加载、频繁渲染或明显不必要的内存分配，但当前 change 未直接触发生产阻断。
- 可维护性风险：职责混杂、模块边界模糊、复杂度过高、隐式全局状态、难以测试的耦合或重复逻辑。
- 可观测性不足：关键失败缺少上下文日志、业务标识、错误分类或排查路径，但不影响当前功能正确性。
- 兼容性风险：公共 API、CLI 输出、配置、文件路径或持久化格式变化没有充分说明，但已有回退或迁移路径。

## 建议工具化

- 使用 formatter、linter、type checker、static analyzer、dependency scanner、coverage 和 CI gate 检查格式、命名、导入排序、基础类型错误、不可达代码、简单安全规则和覆盖率。
- 可稳定由工具发现或修复的问题不作为人工 review 主清单；只有它们造成行为、安全、兼容性或工作流风险时才升级为 findings。
- 报告中可以建议新增或收紧工具配置，但不要把“未运行某个工具”本身当作代码缺陷，除非 change 的验收明确要求。

## 非目标

- 不替代 `test-report.md` 的测试执行记录。
- 不替代 `review-report.md` 的最终 review 结论。
- 不要求 reviewer 手工承担 formatter、lint、coverage 或静态分析工具的完整职责。
- 不对未修改的第三方库、生成代码或历史债务做泛化审查。

## 常见误报

- 只因命名、缩进、导入顺序、注释格式不符合偏好就给 blocking。
- 只因某段代码复杂就要求重构，但无法说明当前 change 的行为风险、测试风险或维护边界风险。
- 把缺少某个工具配置当作失败，而没有证明它影响当前 change 的验收。
