# integrate-codegraph-into-wiki-skills 任务计划

## 任务总览

按 CodeGraph orchestration、CLI/status、localized Skills、文档和 distribution boundary 分成可独立验证的能力块。

## 实现模式

tdd

## 1. CodeGraph 外部命令 orchestration

- [x] 1.1 Red: UT-01/UT-02 添加 runner 与 bootstrap 失败测试（`pnpm --filter spec-wiki-lite test -- --run src/orchestration/codegraph/runner.test.ts src/orchestration/init/runInit.test.ts` Red 已证实）
- [x] 1.2 Green: 实现参数数组 runner、版本检测、npm 安装、Codex MCP 配置和项目 init
- [x] 1.3 Refactor: 收口 warning、recovery 和可注入 runner

### CheckList

- [x] Red 失败已确认
- [x] 成功/失败/skip 测试通过
- [x] 参数安全测试通过
- [x] 类型和注释检查通过

## 2. CLI init 与 status

- [x] 2.1 Red: UT-03 添加 `--no-codegraph`、`init --json` 和 status 字段断言
- [x] 2.2 Green: 扩展 parse/execute、init JSON envelope 和只读 status
- [x] 2.3 Refactor: 保持现有 ready/exit code 合同

### CheckList

- [x] help/JSON/skip 合同通过
- [x] status 不产生外部副作用
- [x] package typecheck 通过

## 3. 双语 Wiki Skills 与文档

- [x] 3.1 Red: UT-04 添加 8 个 zh/en Skill CodeGraph 合同断言
- [x] 3.2 Green: 写入阶段化 CodeGraph MCP/CLI/fallback 提示并同步 repo-local Skills
- [x] 3.3 更新 README、README-CN 和相关 Wiki 设计/快速上手文档

### CheckList

- [x] 16 个 package Skill 文件覆盖规则
- [x] repo-local update 逐字同步
- [x] 文档说明外部工具边界和 warning 行为

## 4. Distribution 与验收

- [x] 4.1 Red/Green: 增加 tarball 不含 `.codegraph` 数据和 runtime 的断言
- [x] 4.2 执行 package/root tests、lint、两层 typecheck、build、pack、Wiki validate、diff check（`pnpm test`、`pnpm lint`、两层 `tsc --noEmit`、`pnpm build`、`pnpm run pack`、strict validate、`git diff --check`）
- [x] 4.3 生成 full/pass review 与 verification 并归档

### CheckList

- [x] tarball smoke 不依赖真实网络/用户配置
- [x] 所有质量门禁通过
- [x] archive 前 strict validate 通过

## 用例到任务映射

| 系统测试用例 | 大 task | 小 task |
| --- | --- | --- |
| ST-01/02/04 | 1 | 1.1-1.3 |
| ST-01/03/06 | 2 | 2.1-2.3 |
| ST-05 | 3 | 3.1-3.3 |
| ST-06 | 4 | 4.1-4.3 |

## 执行顺序

1. 先完成 runner/bootstrap Red。
2. 实现 CodeGraph orchestration，再接 CLI/status。
3. 更新双语 Skills、文档和分发测试。
4. 执行全量验证、review、verification、archive。

## 暂缓事项

- 不支持 Claude/Cursor 等其他宿主配置。
- 不把 CodeGraph 数据导入 `.wiki` 或 `.spec`。
