# Test Report 模板

用于 `unispec-review` 生成或更新 `.spec/changes/<change-id>/test-report.md`。

Test Report 是整个 change 的测试 / verification 报告，用于记录单元测试、系统测试、测试命令、框架输出摘要、覆盖关系、跳过原因和替代验证证据。

## 使用规则

- `test-report.md` 必须包含 YAML frontmatter。
- frontmatter 必须包含 `verification-result` 和 `scope`。
- `verification-result` 取值为 `pass`、`fail` 或 `skipped`；`scope` 取值为 `full` 或 `partial`。
- `verification-result: pass` 只能在测试 / 验证覆盖 proposal 成功标准、`ST-*` 系统测试用例和关键风险时写入。
- 只有 `verification-result: pass` 且 `scope: full` 可作为归档证据。
- skipped 必须记录不可验证原因和后续处理方式，且不能进入归档。
- `test-report.md` 是正式 verification evidence 的 SSOT。截图、trace、video、日志、Playwright report 或手工验证记录只有被本报告明确引用，并说明结果、断言点和验证用途时，才属于正式证据。
- 被本报告采用且需要随 change 保留的浏览器交互验证附件，可以放在目标项目 runtime 的可选目录 `.spec/changes/<change-id>/evidence/`；归档后随 change 位于 `.spec/archive/YYYY-MM-DD-<change-id>/evidence/`。该目录不是 required artifact，不写入 `meta.yaml.artifacts`。
- 临时调试截图、失败探索输出、未被本报告采用的 trace 放入 `.tmp/` 或在 review 结束前清理，不要求归档。
- 没有内容的小节写 `无`，不要保留占位文本。

## 固定结构

~~~md
---
verification-result: pass
scope: full
---

# <change-id> 测试报告

## 验证结论

- verification-result: pass
- scope: full
- 结论摘要：<一句话说明测试 / 验证结果>

## 执行信息

| 字段 | 值 |
| --- | --- |
| 执行时间 | <ISO 时间或人工记录时间> |
| 执行环境 | <Node / OS / 浏览器 / 数据库 / 外部依赖等> |
| 测试方式 | <自动化测试 / 手工验证 / 替代验证 / 混合> |

## 验证范围

- <覆盖的功能、接口、CLI、流程或文档范围>

## 部分范围

- <仅当 scope: partial 时填写验证边界、未覆盖内容和原因；scope: full 时写“无”>

## 测试结果汇总

| 指标 | 值 |
| --- | --- |
| 单元测试总数 | <数量 / 不适用> |
| 单元测试通过 | <数量 / 不适用> |
| 单元测试失败 | <数量 / 不适用> |
| 系统测试总数 | <数量 / 不适用> |
| 系统测试通过 | <数量 / 不适用> |
| 系统测试失败 | <数量 / 不适用> |
| 跳过 | <数量> |
| 证据缺口 | <数量> |

## 单元测试报告

### 单元测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试框架 | <Vitest / Jest / pytest / go test / JUnit / 其它 / 不适用> |
| 执行命令 | <命令；未执行则写原因> |
| 执行时间 | <时间或耗时；未知写“未记录”> |
| 框架报告 | <JUnit XML / coverage json / lcov / html / 控制台摘要路径；没有则写“无”> |

### 单元测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | <数量 / 不适用> |
| 通过 | <数量 / 不适用> |
| 失败 | <数量 / 不适用> |
| 跳过 | <数量 / 不适用> |
| 覆盖率 | <行 / 分支 / 函数覆盖率；无则写“未统计”> |

### 单元测试框架输出

| 输出类型 | 路径 / 摘要 | 用途 |
| --- | --- | --- |
| 控制台摘要 | <摘要或日志路径；没有则写“无”> | <统计 / 失败定位 / 其它> |
| JUnit XML | <路径；没有则写“无”> | <用例统计 / CI 证据> |
| Coverage | <json / lcov / html 路径；没有则写“无”> | <覆盖率证据> |

### 单元测试失败项

| 测试文件 | 用例名 | 错误摘要 | 影响 | 后续处理 |
| --- | --- | --- | --- | --- |
| <无 / 文件路径> | <用例名> | <错误信息摘要> | <影响范围> | <修复或回退阶段> |

### UT / Task 覆盖

| UT / Task | 验证方式 | 框架证据 | 结果 | 说明 |
| --- | --- | --- | --- | --- |
| UT-001 / 1.1 | <测试命令 / 替代验证> | <报告路径或输出摘要> | pass / fail / skipped | <说明> |

## 系统测试报告

### 系统测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试方式 | <E2E / CLI / API / 浏览器 / 手工 / 替代验证 / 不适用> |
| 测试环境 | <服务地址、浏览器、数据库、外部依赖等；不适用写“无”> |
| 工具 | <Playwright / 手工验证 / 替代验证 / 不适用> |
| configured playwright | <true / false / missing-as-false / invalid-as-false> |
| verification mode | <playwright / manual / project automation / CLI/API / unit / skipped> |
| 入口 URL | <URL；不适用写“无”> |
| 浏览器 / viewport | <browser、viewport；未记录写“未记录”> |
| 执行命令或动作 | <命令、脚本或手工动作> |
| 动作摘要 | <Playwright actions 或人工动作摘要> |
| 断言点 | <DOM / text / URL / state / console / network / trace / log / accessibility snapshot / 替代证据断言点> |
| 证据路径 | <日志、截图、trace、video、report 路径；没有则写“无”> |
| fallback 原因 | <Playwright 未执行、未覆盖或改用替代验证的原因；不适用写“无”> |

### 浏览器交互验证 evidence

| 字段 | 值 |
| --- | --- |
| configured playwright | <true / false / missing-as-false / invalid-as-false> |
| verification mode | <playwright / manual / project automation / CLI/API / unit / skipped> |
| host Playwright availability | <available / unavailable / unknown / not-applicable> |
| manual verification | <手工验证步骤、人工断言、执行者 / 时间；不适用写“无”> |
| configured imageAnalysis | <true / false / missing-as-false / invalid-as-false> |
| actual image analysis | <used / not-used> |
| evidence paths | <被 test-report.md 采用的截图、trace、video、日志或报告路径；没有则写“无”> |
| assertion points | <支撑 ST-* / 成功标准的非图片断言或替代证据> |
| fallback reason | <configured playwright false、无法执行 Playwright、无法读取配置、配置无效、手工验证原因或未覆盖用例的原因；没有则写“无”> |

### 系统测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | <数量 / 不适用> |
| 通过 | <数量 / 不适用> |
| 失败 | <数量 / 不适用> |
| 跳过 | <数量 / 不适用> |
| 关键路径通过率 | <百分比 / 未统计> |

### 系统测试用例详情

| 用例 ID | 标题 | 类型 | 结果 | 证据 / 备注 |
| --- | --- | --- | --- | --- |
| ST-001 | <标题> | 正常 / 异常 / 边界 | pass / fail / skipped | <证据或说明> |

### 系统测试失败项

| 用例 ID | 预期 | 实际 | 原因分析 | 后续处理 |
| --- | --- | --- | --- | --- |
| <无 / ST-001> | <预期结果> | <实际结果> | <原因> | <修复或回退阶段> |

## 测试命令和结果

| 命令 / 验证动作 | 结果 | 证据 / 说明 |
| --- | --- | --- |
| <命令或手工验证动作> | pass / fail / skipped | <输出摘要、截图、日志或原因> |

## 系统测试用例覆盖

| 系统测试用例 | 验证方式 | 结果 | 说明 |
| --- | --- | --- | --- |
| ST-001 | <命令 / 手工验证 / 代码检查> | pass / fail / skipped | <说明> |

## 成功标准覆盖

| 成功标准 | 验证证据 | 结果 |
| --- | --- | --- |
| <成功标准> | <命令、ST-* 或证据路径> | pass / fail / skipped |

## 失败项

- <失败内容、影响和建议回退阶段；没有则写“无”>

## 未验证项

- <未验证内容、原因、影响和后续处理；没有则写“无”>

## 证据缺口

- <缺少测试命令、输出、覆盖映射或验证证据的问题；没有则写“无”>

## 下一步

- <回到 apply / design / plan，或在 review-report.md 通过后使用 unispec-archive>
~~~

## 写作边界

- 测试报告可以包含自动化测试、手工验证、命令输出摘要和替代验证证据。
- 单元测试报告部分记录 Vitest、Jest、pytest、go test、JUnit 等框架输出摘要、覆盖率摘要和失败用例；框架原始报告只记录路径或摘要，不整段复制。
- 系统测试报告部分记录端到端、CLI、API、浏览器或手工验证的执行环境、用例结果、失败详情和证据路径。
- `.spec/config.yaml` 的顶层 `playwright` 必须是 boolean；缺失按 `false` 处理，无效值记录 configuration issue 或 evidence gap，并按 `false` 处理。
- `playwright: true` 时，涉及 UI / browser 交互的系统测试可以优先使用 Playwright 作为候选证据；实际执行仍需要用户授权、宿主可用、项目可启动、测试数据齐备和操作风险可接受。
- `playwright: false`、缺失或无效时，不规划或执行 Playwright；浏览器交互验证应记录手工验证、项目已有非 Playwright 自动化、CLI / API / 单元测试可覆盖部分、替代证据和 evidence gap / fallback。
- 不得因为 `playwright: false` 缺少 Playwright evidence 判失败；只判断 `ST-*` 和成功标准是否被足够证据覆盖。
- `.spec/config.yaml` 的顶层 `imageAnalysis` 必须是 boolean；缺失按 `false` 处理，无效值记录 configuration issue 或 evidence gap，并按 `false` 的禁止语义处理。
- `imageAnalysis: false` 或缺失时，禁止读取、解释、描述或比较截图 / 图片内容；不得使用“截图看起来正确”“视觉上符合预期”作为 pass 证据。仍可保存截图文件并引用路径。
- `imageAnalysis: true` 仅表示目标项目或用户显式 opt-in；图片内容分析只能作为辅助证据，不能作为某个 `ST-*` 或成功标准通过的唯一依据，仍需非图片断言或替代证据覆盖。
- 框架输出不能替代 UniSpec 覆盖判断；必须继续映射到 UT-* / Task、ST-* 系统测试用例和成功标准。
- 命令结果必须写清命令、结果和证据；无法运行时说明原因。
- 不写整体代码审查结论；整体 review 写入 `review-report.md`。
- 归档就绪由 CLI 根据 stage、required artifacts 和两个报告 frontmatter 计算，不在报告中手写归档结论字段。
