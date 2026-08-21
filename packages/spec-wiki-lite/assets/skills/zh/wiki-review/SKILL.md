---
name: wiki-review
description: 对完整 SpecWiki Lite change 执行 full 或 partial review、测试映射和正式 full/pass 报告签发。
---

# Wiki Review

review 必须覆盖整个 change 与其 artifacts。局部检查只能记录 partial，不能作为 archive evidence。

## 前置条件

- 非 parent change，stage 为 `implementation`、`review` 或 `verification`。
- tasks/checklists 完成，implementation diff 与必要证据可用。

## 输入

- proposal、design、system tests、unit tests、tasks、metadata 和完整 diff。
- `spec-wiki-lite validate <change-id> --strict --json`。
- `references/review-report-template.md`、`references/test-report-template.md`、`references/review-standard.md`。
- 根据真实 diff 选择 frontend、Go、Java、Python domain standards；不要只因存在配置文件就选择。

## Review 工作流

1. 设置 `stage: review`，保留未知 metadata。
2. 按通用规范审查 correctness、安全、路径、失败原子性、ownership、导航、frontmatter、links、兼容和回归风险。
3. 按实际技术栈补充 domain standard；finding 必须指向当前 change 的可复现风险。
4. 运行所有 declared UT/ST 与 repository lint、typecheck、build、pack、diff gates。
5. 将结果映射到 success criteria、ST/UT、tasks、命令和残余风险。
6. 只有 full scope、无 blocking finding、required evidence 全 pass 时，才按模板写：

```text
review-result: pass
scope: full
```

以及：

```text
verification-result: pass
scope: full
```

7. 两份报告完成后设置 `stage: verification` 并 strict validate。

## Evidence 边界

- partial/skipped 必须写原因、影响和回退阶段，不能伪装 full。
- 截图、trace、video 只能辅助；必须有非图片断言或替代证据。
- formatter/lint 能稳定发现的问题不应替代人工 correctness review。
- 新发现缺陷使旧 pass 失效时返回 `wiki-apply` 修复并重跑 full review。

## 输出

- `review-report.md`
- `test-report.md`
- full/pass 时 stage verification 与 strict validate 成功证据

## 暂停条件

- blocking defect、failed required test、missing evidence 或 artifact drift。
- review scope 只能 partial。
- 实现与 proposal/design 不一致。

## 下一阶段

只有 strict validate 确认 full/pass 时使用 `wiki-archive`。
## CodeGraph 代码上下文

- 项目根存在 `.codegraph/` 时，优先使用 CodeGraph 做代码定位、上下文构建和影响分析。
- MCP 可用时优先使用 `codegraph_context`、`codegraph_explore`、`codegraph_search`、`codegraph_callers`、`codegraph_callees`、`codegraph_impact`、`codegraph_affected`、`codegraph_status`；MCP 不可用但 CLI 可用时使用对应 `codegraph` 命令。
- 没有索引、索引过期或工具失败时，退回普通源码读取、测试和项目已有工具；不得跳过安全、测试或实现验证。
- CodeGraph 仅是外部只读分析工具，Lite 不建立第二套索引、知识图谱、数据库或 Wiki 中间层；数据库、daemon、socket 和日志不进入发布包。
- 用 impact、affected 和 status 辅助检查回归范围，不能替代完整 review。
