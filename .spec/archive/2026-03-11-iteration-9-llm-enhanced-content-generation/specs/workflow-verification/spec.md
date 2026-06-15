## ADDED Requirements

### Requirement: 验证必须覆盖 LLM 开关、缓存命中与桥接回退
系统 MUST 提供自动化测试，验证在 LLM 关闭、桥接不可用、缓存命中和增强开启四种典型路径下，`init`、`update` 和 `rebuild` 都能保持可回退性与页面稳定性。验证 MUST 同时覆盖 core 直接执行和 CodeBuddy Agent 桥接执行，不得只测 happy path。

#### Scenario: 未启用 LLM 时 deterministic 主链保持可用
- **WHEN** 测试在未开启 LLM 增强的情况下执行 `init`、`update` 或 `rebuild`
- **THEN** 测试 MUST 观察到 workflow 成功完成
- **THEN** 测试 MUST 观察到页面、状态库和 metadata 与 deterministic 预期保持一致

#### Scenario: 相同输入第二次执行命中 LLM 缓存
- **WHEN** 测试对同一仓库、同一模型和相同输入连续执行两次启用增强的 workflow
- **THEN** 第二次执行 MUST 复用 `llm_cache`
- **THEN** 测试 MUST 观察到真实 LLM 请求次数少于第一次执行

#### Scenario: Agent 桥接不可用时 core 自动回退
- **WHEN** 测试通过 CodeBuddy Agent 执行启用增强的长流程 workflow，但 Agent 对 `llm_request` 返回不可用
- **THEN** 测试 MUST 观察到 core 回退到 deterministic 内容并完成 workflow
- **THEN** 最终结果 MUST 仍然通过终态事件返回

#### Scenario: provider 直连优先于 Agent bridge
- **WHEN** 测试同时提供可用的 provider 直连配置和 Agent bridge
- **THEN** 测试 MUST 观察到 core 优先使用 provider 直连
- **THEN** Agent bridge 不得收到同一请求对应的 `llm_request`

#### Scenario: dev 配置文件可驱动本地 provider 验证
- **WHEN** repo 根存在 `wiki.dev.yaml` 并声明可用的 provider 直连配置
- **THEN** 测试 MUST 观察到 workflow 读取该文件并通过 provider 路径完成请求
- **THEN** 删除该文件后，workflow MUST 回退到共享 steering + Agent/fallback 行为

#### Scenario: provider 并行增强仍保持预算与顺序约束
- **WHEN** 测试为 provider 直连配置开启 `llm.parallel_requests > 1`
- **THEN** 测试 MUST 观察到同深度页面增强可以并行完成
- **THEN** 测试 MUST 同时验证父页晚于子页、总真实调用数不超过预算、缓存命中页不占用真实并行槽

### Requirement: 项目集验证必须覆盖增强后的页面信息密度与 graph 落地
每轮与迭代 9 相关的 tasks 设计、实现或测试时，系统 MUST 对 `.wiki/06-设计文档/00-总体设计.md § 测试项目集` 的完整项目集执行 `init` 分析，并在 `test-project-analysis.md` 中按项目输出增强后的页面信息密度、graph facts 是否进入页面正文、workflow/architecture 页面表现，以及与 reference 的差异。验证可以按 deterministic baseline 与增强模式做对照，但不得只给总表结论。

#### Scenario: 项目集分析逐项目输出增强表现
- **WHEN** 迭代 9 的 tasks 设计或测试阶段执行完整项目集 `init` 分析
- **THEN** 报告 MUST 按项目逐个说明 overview、architecture、module、workflow 页的增强表现
- **THEN** 报告 MUST 说明 communities / processes / cycle warnings 是否真正进入正文，而不只是底层 state

#### Scenario: 有 reference 的项目继续进行结构对照
- **WHEN** 测试项目存在 `tmp/reference/*`
- **THEN** 项目集分析 MUST 继续对照 `.wiki/*.md` 和 `wiki.metadata.json`
- **THEN** 报告 MUST 明确记录增强后页面结构与 reference 的差异或“无显著差异”

#### Scenario: reference 报告脚本执行期具备可观测进度
- **WHEN** 测试脚本批量生成 reference 项目报告
- **THEN** 控制台输出 MUST 能看到项目级和 phase 级进度，而不是只在结束时一次性返回结果
- **THEN** 脚本 MUST 支持通过 `--jobs` 调整项目级并行度
