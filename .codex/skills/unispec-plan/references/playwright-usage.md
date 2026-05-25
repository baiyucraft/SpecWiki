# Playwright 用法参考

用于 `unispec-plan` 在设计 UI、前端页面或浏览器交互类系统测试时，补充 `system-tests.md` 中 `ST-*` 的 `验证方式`。

本文档只指导目标项目如何规划 Playwright 验证方式。UniSpec CLI 不执行 Playwright、不安装 Playwright、不提供内置 runner、不新增 workflow stage，也不要求目标项目新增 `dependencies` / `devDependencies`。

## 配置前置条件

使用本文档前，先读取目标项目 `.spec/config.yaml` 的顶层 `playwright`：

- `playwright: true`：允许 UI / browser `ST-*` 优先规划 Playwright；实际执行仍取决于用户授权、宿主 Playwright 可用、项目可启动、数据 / 登录态齐备和操作风险可接受。
- `playwright: false`、缺失或无效：不规划、不执行 Playwright；涉及浏览器操作但无法由 CLI / API / 单元测试覆盖的 `ST-*`，应写为手工验证或项目已有非 Playwright 自动化验证，并记录人工步骤、人工断言、替代证据和 evidence gap / fallback。

本 reference 仅在 `playwright: true` 时适用。`playwright: false`、缺失或无效时，即使用户临时提到浏览器自动化，也不以本文档作为系统测试规划依据；应先由用户确认更新目标项目配置。

`playwright` 与 `imageAnalysis` 相互独立：`playwright: true` 不表示允许图片内容分析；`imageAnalysis: true` 也不表示允许使用 Playwright。

## 适用场景

- 页面跳转、表单填写、列表筛选、分页、弹窗、上传、下载、键盘操作等用户可观察 UI 流程。
- 需要浏览器状态验证的场景，例如 URL、DOM 文本、可访问性结构、localStorage / sessionStorage、network 请求、console error 或页面状态。
- proposal / design 的成功标准必须通过真实浏览器交互才能可信验证的场景。

不适用时不要机械使用 Playwright。CLI、API、数据库、纯后端逻辑或无需浏览器交互的行为，应使用对应验证方式。

## 命令选择

优先使用目标项目已经约定的命令，不为了 UniSpec 额外引入 Playwright 依赖。

| 场景 | 推荐写法 |
| --- | --- |
| 项目已有 E2E / browser 测试脚本 | 使用项目脚本，例如 `npm run test:e2e`、`pnpm test:e2e` |
| 项目已安装 Playwright | 使用项目内 Playwright，例如 `playwright test`、`pnpm exec playwright test`、`npm exec playwright test` |
| 需要 headed 调试 | 使用 `playwright test --headed` 或项目等价命令 |
| 只跑 Chromium | 使用 `playwright test --project chromium` 或项目等价命令 |
| 需要 trace | 使用 `playwright test --trace on` 或项目等价命令 |
| 项目未安装且用户授权临时验证 | 仅在没有项目脚本、没有项目内 Playwright、且 `playwright: true` 已配置时，fallback 使用 `npx playwright test`、`npx playwright --help` 等 `npx playwright ...` 命令 |
| 缺少浏览器二进制 | 可由执行者按目标项目约束运行 `npx playwright install chromium` 或项目等价命令 |

如果目标项目已有包管理器、workspace、测试脚本或 CI 约定，以项目事实为准。不要在 `system-tests.md` 中要求修改依赖文件。

本文档不使用本地宿主的 shell wrapper；不要生成 `.sh` wrapper、`CODEX_HOME`、`PWCLI` 或宿主专属输出目录作为目标项目的必需用法。

## Playwright CLI 具体命令

本节用于规划“打开真实浏览器并按步骤操作页面”的验证方式。以下命令属于 `@playwright/cli` / `playwright-cli` 交互式 CLI，不是 `@playwright/test` 的 test runner。

以下命令只用于已配置 `playwright: true` 的目标项目。

先确定命令前缀：

~~~text
# 已安装 playwright-cli 时
<pw> = playwright-cli

# 未安装或临时使用时
<pw> = npx --package @playwright/cli playwright-cli
~~~

`<pw>` 是文档占位符。写入 `system-tests.md` 时，可以保留 `<pw>` 表示“按目标项目实际环境替换”，也可以写成目标项目确认可用的真实命令。

### 标准交互循环

先打开页面，再获取 snapshot，之后使用 snapshot 中的稳定元素引用执行操作。页面导航、弹窗、tab 切换或 DOM 大幅变化后重新 snapshot。

~~~text
<pw> open https://example.com --headed
<pw> snapshot
<pw> click e3
<pw> snapshot
~~~

常用规则：

- 操作元素前先 `<pw> snapshot`，不要凭空编造 `e12` 这类元素引用。
- 元素引用失效时重新 `<pw> snapshot`。
- 需要人工观察窗口状态或定位问题时使用 `--headed`。
- 需要固定视口时先执行 `<pw> resize 1280 720` 或使用 CLI 配置文件。

### 表单与控件

~~~text
<pw> open https://example.com/form --headed
<pw> snapshot
<pw> fill e5 "user@example.com"
<pw> fill e6 "password123"
<pw> check e7
<pw> select e8 "admin"
<pw> upload ./fixtures/avatar.png
<pw> press Enter
<pw> snapshot
~~~

更多控件操作：

~~~text
<pw> click e3
<pw> dblclick e7
<pw> hover e4
<pw> drag e2 e8
<pw> uncheck e7
<pw> dialog-accept
<pw> dialog-dismiss
~~~

### 导航、键盘和鼠标

~~~text
<pw> go-back
<pw> go-forward
<pw> reload
<pw> press ArrowDown
<pw> keydown Shift
<pw> keyup Shift
<pw> mousemove 150 300
<pw> mousewheel 0 100
~~~

### 数据提取与非图片断言

使用 `eval` 读取文本、URL、DOM 状态或浏览器状态，作为非图片断言来源。

~~~text
<pw> eval "document.title"
<pw> eval "location.href"
<pw> eval "el => el.textContent" e5
<pw> eval "localStorage.getItem('token')"
~~~

系统测试通过证据应优先来自 DOM/text、URL、aria / accessibility 状态、network、console、storage、API 响应、数据库状态或日志，而不是截图内容。

### 保存证据

~~~text
<pw> screenshot
<pw> screenshot e5
<pw> pdf
<pw> tracing-start
# 执行需要复现的交互步骤
<pw> tracing-stop
~~~

截图、PDF、trace、video 或日志只是 evidence artifact。只有在后续 `test-report.md` 中被明确引用，并说明断言点、结果和用途时，才是正式 verification evidence。

### Tabs 与 session

~~~text
<pw> tab-new https://example.com/page
<pw> tab-list
<pw> tab-select 0
<pw> tab-close 1
<pw> --session checkout open https://example.com/checkout
<pw> --session checkout snapshot
~~~

当多个验证流程需要隔离登录态、tab 或浏览器上下文时，使用不同 `--session` 名称。

### Console、network 与 trace 调试

~~~text
<pw> console
<pw> console warning
<pw> network
<pw> tracing-start
# 复现问题
<pw> tracing-stop
~~~

调试命令可用于记录 console warning / error、network 请求和 trace。若这些输出用于验收结论，后续报告必须说明对应的 `ST-*`、断言点和结果。

### CLI 配置文件

`playwright-cli.json` 只适用于 `@playwright/cli` / `playwright-cli`，不是 `@playwright/test` 的 `playwright.config.ts`。

~~~json
{
  "browser": {
    "launchOptions": {
      "headless": false
    },
    "contextOptions": {
      "viewport": { "width": 1280, "height": 720 }
    }
  }
}
~~~

使用配置文件时，在验证方式中写清配置文件路径和影响，例如固定 browser、headed/headless、viewport 或 storage state。

## Playwright Test runner

`playwright test` 用于执行目标项目中已有的 Playwright 测试文件；它不是交互式 `open/snapshot/click` 命令。

~~~text
# 已安装 Playwright Test 或项目已有依赖时
playwright test
playwright test --headed
playwright test --project chromium
playwright test --trace on

# 未安装或临时执行时
npx playwright test
npx playwright test --headed
npx playwright test --project chromium
npx playwright test --trace on
~~~

如果目标项目已有脚本，优先写项目脚本，例如 `npm run test:e2e`、`pnpm test:e2e` 或 CI 中已有命令。不要把 `playwright test` 和 `<pw> snapshot` 这两套命令混用。

## 系统测试写法

当 `ST-*` 计划使用 Playwright 验证时，`验证方式` 至少写清：

- 入口 URL：例如 `http://localhost:3000/settings`，或说明由项目 dev server / test server 提供。
- Browser / viewport：例如 Chromium，`1280x720` / `1920x1080`，或项目既有配置。
- 前置数据：测试账号、fixture、mock 服务、feature flag、初始化命令或“不需要”。
- 操作步骤：按用户行为顺序描述点击、输入、选择、提交、导航、刷新等动作。
- 非图片断言：DOM 文本、URL、aria / accessibility snapshot、network 请求、console error、storage、API 响应、数据库状态或日志。
- 证据类型：Playwright report、trace、video、screenshot attachment、console / network log、命令输出或“不保留附件”。
- fallback reason：Playwright 不可用、dev server 不可用、缺少登录态、操作超出授权、改用手工验证或替代证据的原因；没有则写“无”。

示例：

~~~md
- 验证方式:
  - Playwright: 使用项目既有 `pnpm exec playwright test --project chromium`。
  - 入口 URL: `http://localhost:3000/users`。
  - Browser / viewport: Chromium, 1280x720。
  - 前置数据: 使用 seeded admin 用户和 3 条用户记录。
  - 操作步骤: 打开用户列表，输入关键字，点击查询，切换到第 2 页，再清空条件。
  - 非图片断言: URL query 更新；列表文本只包含匹配用户；分页状态回到第 1 页；console 无 error。
  - 证据类型: Playwright report 和 trace；截图只作为附件路径。
  - fallback reason: 无。
~~~

## 图片分析规则

读取目标项目 `.spec/config.yaml` 的顶层 `imageAnalysis`：

- 缺失、无效或 `imageAnalysis: false`：禁止读取、解释、描述或比较截图 / 图片内容；不得把“截图看起来正确”“视觉上符合预期”作为 pass 证据。
- `imageAnalysis: true`：仅表示目标项目或用户显式允许图片内容分析；图片分析只能作为辅助证据，不能作为某个 `ST-*` 或成功标准通过的唯一依据。

`imageAnalysis: false` 不禁止 Playwright 保存截图、trace 或 video，也不禁止报告引用附件路径。它只禁止模型用图片内容本身做判断。通过证据必须来自非图片断言或明确的替代证据。

## 失败与 fallback

Playwright 无法执行、无法覆盖计划用例或断言不足时，不表示系统测试通过。应在 `system-tests.md` 或后续 `test-report.md` 中记录：

- 未覆盖的 `ST-*`。
- Playwright availability。
- 失败命令或无法执行原因。
- 已保存的证据路径。
- 替代验证方式和覆盖范围。
- 是否存在 evidence gap。
