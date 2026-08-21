# integrate-codegraph-into-wiki-skills 系统测试

## 测试环境

- runtime/platform：Node.js >=20.19、Windows/CI
- fixture/data：临时项目、注入的外部命令 runner、双语 package assets
- 外部依赖：测试中 mock；tarball smoke 不访问真实网络或用户配置

## ST-01 CodeGraph 成功初始化

- 类型：normal
- 前置：runner 模拟已有 CLI、MCP 配置和项目 init 成功
- 操作：执行 `init --json`
- 断言：Wiki 初始化成功，CodeGraph requested/available/configured/initialized 全为 true，warnings 为空

## ST-02 CodeGraph 各阶段失败降级

- 类型：failure
- 前置：分别模拟 npm 安装、MCP 配置或项目 init 失败
- 操作：执行 init
- 断言：CLI 仍成功完成 Wiki/.spec/Skills；对应 warning 包含 stage/message/recovery；ready 不受影响

## ST-03 显式跳过

- 类型：boundary
- 前置：`--no-codegraph`
- 操作：执行 `init --no-codegraph --json`
- 断言：requested 为 false，runner 无调用，Wiki 初始化正常

## ST-04 参数安全

- 类型：failure
- 前置：项目路径包含空格、分号或 shell 特殊字符
- 操作：执行 CodeGraph project init
- 断言：路径作为单独 argv 传递，无 shell 注入或越界写入

## ST-05 Skill 内容与同步

- 类型：normal
- 前置：zh/en 8 个 package Skill
- 操作：扫描并执行 build/update
- 断言：全部 Skill 有 CodeGraph 优先/fallback/边界说明，repo-local 与 package-owned 文件一致

## ST-06 status 与发布边界

- 类型：boundary
- 前置：有/无 `.codegraph`
- 操作：执行 status、pack
- 断言：status 只读报告 initialized；tarball 不包含 `.codegraph`、数据库或 CodeGraph runtime

## 成功标准映射

| 成功标准 | ST |
| --- | --- |
| init 结果和成功路径 | ST-01 |
| 失败 warning 不阻断 | ST-02 |
| no-codegraph 无副作用 | ST-03 |
| 参数安全 | ST-04 |
| 双语 Skill 同步 | ST-05 |
| status/tarball 边界 | ST-06 |
