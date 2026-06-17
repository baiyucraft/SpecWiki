# Frontend Review 规范

符合通用规范。Frontend 是一个 review domain，覆盖 TypeScript、React、浏览器 UI、前端交互、样式和前端测试，不再拆分 TypeScript / React 独立 standards 文件。

## 适用范围

- `.ts` / `.tsx` / `.js` / `.jsx` 中属于前端页面、组件、Hooks、状态管理、浏览器 API、样式、表单、查询列表或交互流程的变更。
- React / Vue / 其它前端框架依赖、前端页面目录、样式文件，或 proposal / design / tasks / wiki 明确说明的前端工作。
- 仅有 `package.json` 或 `tsconfig.json` 不能单独判定为前端；纯 Node、CLI、构建脚本或库代码应按实际 domain 选择。

## 必须同时读取

- `references/review-standard.md`
- 当前项目的前端框架、路由、状态管理、表单库、请求库、样式方案和测试方案事实。

## Blocking / P0

- 类型与运行时边界：API 响应、URL 参数、表单输入、环境变量、localStorage/sessionStorage、上传文件和外部 JSON 必须有类型契约和运行时校验，不能只依赖 TypeScript 静态类型。
- 异步和副作用：Promise rejection、请求取消、组件卸载、定时器、订阅、resize / scroll 监听和 useEffect 清理必须处理；不能在卸载后 setState 或吞掉失败。
- React / 组件状态：Hooks 调用顺序、effect 依赖语义、闭包旧值、受控 / 非受控状态、列表 key、派生状态和乐观更新回滚必须符合真实生命周期。
- 查询与分页：点击查询从第 1 页开始；分页必须携带已确认查询条件；修改条件但未查询时翻页不能误用新条件；删除后应按批量 / 单个删除语义刷新正确页。
- 表单与提交：必填、长度、格式、日期范围、坐标范围、正整数等约束必须阻止无效提交；接口调用按钮必须 loading / disabled 防重入。
- UI 反馈：loading、empty、error、disabled、未实现动作、批量未选中、删除确认、操作成功 / 失败提示和可恢复路径必须可见。
- 安全：禁止未消毒的 `dangerouslySetInnerHTML`，URL 参数必须编码，token 和敏感数据不能明文放入 localStorage/sessionStorage，日志和错误提示不得泄露敏感信息。
- 前端验证：关键用户行为、错误态、空状态、提交防重入、请求失败和成功标准必须有组件测试、端到端验证或明确替代证据。
- 浏览器交互验证：涉及 UI / browser 交互的验证方式可以使用 Playwright、项目已有自动化、组件测试、手工验证或替代证据。审查重点是步骤、断言、结果和证据是否足够，而不是是否存在某个配置开关。

## Non-blocking / P1/P2

- 性能：大列表缺少虚拟滚动、大图未懒加载、路由级代码未分割、渲染中重复重计算、无意义 memoization 或高频事件缺少防抖 / 节流。
- 体验一致性：placeholder、必填提示、按钮位置、时间格式、空值展示、Tooltip / title、ellipsis、分页 / 查询区域固定等与项目约定不一致但未阻断主要流程。
- 可访问性：焦点路径、键盘可达性、语义标签、错误提示关联和图标按钮可理解性不足。
- 可维护性：组件过大、props 透传失控、跨层级传参过深、状态位置不合理、样式污染或 z-index 层级混乱。

## 建议工具化

- 配置 TypeScript strict、ESLint、Prettier、eslint-plugin-react-hooks、import 规则、accessibility scan、coverage、项目已有或用户授权的 Playwright 验证、E2E smoke 和 visual regression。
- Hooks deps、基础 no-any、JSX / CSS 格式、导入排序、命名、未使用变量、简单 a11y 属性和覆盖率阈值优先交给工具。
- React Testing Library 应优先测试用户行为和可见结果；API 请求使用 MSW / mock，不连接真实后端。

## 非目标

- 不因审美偏好、组件文件命名偏好或格式偏好给 blocking，除非影响可用性、设计系统一致性或现有工作流。
- 不机械要求所有函数都 useCallback / useMemo / React.memo；只有引用稳定性、子组件渲染或昂贵计算产生真实风险时才提出。
- 不把 TypeScript / React 拆成独立 standards 文件；本文件统一覆盖前端 domain。

## 常见误报

- 仅看到 `package.json`、`tsconfig.json` 或 `.ts` 文件就判定为前端。
- 把 Hooks lint 可稳定发现的问题逐条人工列为主 findings，而没有说明真实生命周期风险。
- 对内部状态测试实现细节提出强要求，而不是关注用户行为和可见结果。
- 因没有 Playwright evidence 机械判风险；应检查组件测试、E2E、手工验证、项目自动化或替代证据是否覆盖相关 `ST-*` 和成功标准。
