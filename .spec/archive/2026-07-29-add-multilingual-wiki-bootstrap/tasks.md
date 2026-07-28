# add-multilingual-wiki-bootstrap 实现任务

implementation-ready: true
implementation-mode: tdd

## Red

- [x] R1 编写 config、CLI 默认中文/显式英文失败测试（UT-01、UT-05；ST-01、ST-02、ST-04）。
- [x] R2 编写双语 registry、ownership、迁移与旧英文升级失败测试（UT-02、UT-03；ST-05 至 ST-08）。
- [x] R3 编写 bootstrap status、Skill 路由和分发 inventory 失败测试（UT-04、UT-06；ST-03、ST-09 至 ST-11）。
- [x] R4 运行 focused tests并记录 Red：4 suites failed，9 tests failed；缺少 config、locale registry、migration、bootstrap status 和 CLI language。

## Green

- [x] G1 实现 `.wiki/config.yaml` schema、默认 zh、结构化保留写入和失败关闭。
- [x] G2 实现 `init --language zh|en` 与 orchestration 传递。
- [x] G3 重组双语模板、语言化 registry 和旧英文 migration baseline。
- [x] G4 实现普通同步、双向/旧版迁移预检、rollback 和配置提交。
- [x] G5 实现 status language/bootstrapPending 与项目 readiness。
- [x] G6 更新 `wiki-continue` bootstrap 路由。
- [x] G7 更新当前 `.wiki`、README、capability、CLI 和发布合同。
- [x] G8 运行 focused tests，全部 Green：4 suites / 27 tests；另有 staged tarball smoke 1 test 通过。

## Refactor 与验证

- [x] F1 收口类型、错误信息、同步报告和公共 exports；lint 与根/package 两层 typecheck 通过。
- [x] F2 运行 package/root 全量测试、build 和 `git diff --check`；63 package tests 与 5 workspace tests 通过，1 个 Windows 条件测试跳过。
- [x] F3 生成 pack evidence；44 文件 tarball 通过默认中文、显式英文和 bootstrap completion 安装 smoke。
- [x] F4 扫描当前产品面；无 native/index/knowledge runtime 回归，`.spec/archive/**` 无 diff。
- [x] F5 更新任务证据，进入 full review 与 verification。

## 成功标准映射

| 标准 | 测试 | 任务 |
| --- | --- | --- |
| 默认中文与显式英文 | ST-01、ST-02 | R1、G1-G3 |
| 统一配置与失败关闭 | ST-04 | R1、G1、G4 |
| bootstrap readiness | ST-03、ST-09 | R3、G5-G6 |
| ownership 与迁移 | ST-05 至 ST-08 | R2、G3-G4 |
| 分发与长期合同 | ST-10、ST-11 | R3、G7、F2-F4 |
