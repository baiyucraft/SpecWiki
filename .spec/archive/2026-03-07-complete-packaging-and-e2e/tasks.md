## 1. 收口打包与平台分发脚本

- [x] 1.1 调整 `packages/codebuddy/package.json` 与根级脚本入口，明确主包作为分发元数据来源
- [x] 1.2 重构 `scripts/stage-binaries.mjs`，从适配层包清单读取名称、版本和 bin 配置生成主包 manifest
- [x] 1.3 抽出当前平台到平台包名、二进制文件名和复制目标的推导逻辑，替换写死的 `win32-x64-msvc`
- [x] 1.4 更新 `scripts/build-core.mjs` 和 `scripts/publish-npm.mjs`，使 staging 与发布前流程共享同一套二进制定位约定
- [x] 1.5 补强 `tests/integration/package_staging.test.mjs`，断言主包 manifest、`optionalDependencies` 和平台包二进制都正确生成

## 2. 增加端到端验证链路

- [x] 2.1 新增 `tests/e2e/init-update-query.test.mjs`，创建临时 Git 仓库夹具并准备最小源码样本
- [x] 2.2 在 e2e 测试中接入本地构建的 Rust core 二进制，验证适配层 `init` 能生成 `.wiki/` 与 `wiki.metadata.json`
- [x] 2.3 扩展 e2e 测试覆盖源码变更后的 `update` 行为，并断言返回的更新结果非空
- [x] 2.4 扩展 e2e 测试覆盖 `query` 行为，并断言返回结果命中已生成页面
- [x] 2.5 将 e2e 测试入口接入仓库脚本或现有测试说明，保证开发者能直接运行

## 3. 补齐开发与发布前文档

- [x] 3.1 更新 `README.md`，说明项目结构、Rust core 与 CodeBuddy adapter 的角色分工
- [x] 3.2 在 `README.md` 中补充本地开发、测试、构建 core、执行 staging 与查看 `.wiki/` 产物的步骤
- [x] 3.3 在 `README.md` 中补充发布前检查清单，确保文档引用的命令与仓库脚本一致
- [x] 3.4 运行并核对 Rust、adapter、integration、e2e 测试入口，修正文档或脚本中的不一致项
