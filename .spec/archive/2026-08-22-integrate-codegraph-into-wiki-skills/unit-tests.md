# integrate-codegraph-into-wiki-skills TDD 单元测试

## UT-01 CodeGraph runner

- Test：`packages/spec-wiki-lite/src/orchestration/codegraph/runner.test.ts`
- Modify：`packages/spec-wiki-lite/src/orchestration/codegraph/runner.ts`
- 映射：ST-01/02/04
- Red：目标 runner 不存在
- Green：参数数组、cwd、stdout/stderr/exit code 可注入和捕获
- Refactor：统一外部命令错误和恢复信息

## UT-02 bootstrap integration

- Test：`packages/spec-wiki-lite/src/orchestration/init/runInit.test.ts`
- Modify：`runInit.ts` 与 CodeGraph orchestration
- 映射：ST-01/02/03
- Red：init 结果无 CodeGraph 状态且外部失败无法降级
- Green：成功/失败/跳过均返回稳定结果，Wiki 失败仍按原合同失败
- Refactor：隔离核心 asset sync 与外部副作用

## UT-03 CLI/status contract

- Test：`packages/spec-wiki-lite/src/lite-red.test.ts`、`src/core/status.test.ts`
- Modify：`cli.ts`、`status.ts`
- 映射：ST-01/03/06
- Red：help 无新 flags，status 无 codegraph 字段
- Green：解析 flags、JSON 输出和只读路径状态
- Refactor：保持现有 JSON envelope 和 ready 判定

## UT-04 localized Skill contract

- Test：`scripts/tests/workflow-contract.test.ts`
- Modify：双语 8 个 `wiki-*` Skill
- 映射：ST-05
- Red：Skills 无 CodeGraph 提示
- Green：加入 MCP/CLI/fallback/边界说明
- Refactor：统一中英文术语和阶段分工

## UT-05 distribution boundary

- Test：`scripts/tests/distribution.test.ts`、`tarball-smoke.test.ts`
- Modify：构建/分发测试合同
- 映射：ST-06
- Red：无 CodeGraph 发布边界断言
- Green：tarball 排除 `.codegraph` 与外部 runtime
- Refactor：复用现有 staging file inventory
