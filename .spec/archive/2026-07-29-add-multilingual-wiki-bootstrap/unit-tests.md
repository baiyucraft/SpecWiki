# add-multilingual-wiki-bootstrap TDD 单元测试蓝图

## UT-01 配置解析与保留写入

- Test：`src/core/config.test.ts`
- Modify：`src/core/config.ts`
- 覆盖默认 zh、合法 en/zh、无效 YAML/version/language、未知字段与注释保留。
- Red：配置模块不存在或无法解析语言。
- Green：结构化 YAML document 读写通过。

## UT-02 语言化 registry

- Test：`src/core/assets/sync.test.ts`
- Modify：`src/core/assets/registry.ts`
- 覆盖两套目标路径、相同 ownership、8 个稳定 Skills 和旧 registry 隔离。
- Red：当前静态英文 registry 不接受 language。
- Green：按 language 返回确定性资产列表。

## UT-03 双语同步与 migration

- Test：`src/core/assets/sync.test.ts`
- Modify：`src/core/assets/sync.ts`
- 覆盖默认 zh、显式 en、幂等、force、双向迁移、旧英文升级、冲突、rollback、用户页保留。
- Red：中文目标缺失、配置未写或迁移产生混合登记资产。
- Green：预检和事务式应用满足所有断言。

## UT-04 bootstrap inspection

- Test：`src/core/wiki/inspect.test.ts`、`src/lite-red.test.ts`
- Modify：`src/core/wiki/inspect.ts`、`src/core/status.ts`
- 覆盖 marker、language、静态健康与项目 readiness 分离。
- Red：status 缺少字段或 pending 项目仍 ready。
- Green：报告和退出语义稳定。

## UT-05 CLI language

- Test：`src/lite-red.test.ts`
- Modify：`src/cli.ts`、`src/orchestration/init/runInit.ts`
- 覆盖 help、合法值、缺值、非法值、非 init 命令拒绝和 JSON 稳定输出。
- Red：`--language` 被视为 unsupported。
- Green：usage 和 init override 符合合同。

## UT-06 分发与文档合同

- Test：`scripts/tests/distribution.test.ts`、`scripts/tests/current-surface.test.ts`
- Modify：package assets、README、Wiki。
- 覆盖双语 inventory、旧 runtime 禁词、tarball smoke 和文档链接。
