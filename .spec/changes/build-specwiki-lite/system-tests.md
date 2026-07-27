# build-specwiki-lite 系统测试用例

## 用例总览

覆盖新包身份、纯 TypeScript 分发、资产 ownership、Wiki 静态健康、`.spec` 完整阶段、路径安全、旧 runtime 清理和自举归档。

## 系统测试用例

### ST-001 新包安装与公开命令

- 关联成功标准: package/bin 身份和 CLI 闭集。
- 覆盖设计点: Package、CLI。
- 前置条件: 构建并 pack 当前 workspace。
- 操作 / 触发: 在临时目录安装 tarball，执行 `spec-wiki-lite --help`。
- 期望结果: 包为 `spec-wiki-lite@0.1.0`，只列出 init/status/show/validate/update/archive，无 native binary。
- 验证方式: distribution contract test + tarball inventory。

### ST-002 Init/Update ownership 与 Codex-only

- 关联成功标准: 空仓库初始化、幂等、自定义页面保留、只支持 Codex。
- 覆盖设计点: Init 与 Update、Skills。
- 前置条件: 空临时仓库。
- 操作 / 触发: init 两次，写入自定义页面，再执行 update/force，并尝试未知 host。
- 期望结果: 基线、`.spec`、八个 Skills 完整；自定义页保留；未知 host 返回 usage error。
- 验证方式: package integration tests。

### ST-003 Wiki 静态健康检查

- 关联成功标准: INDEX/frontmatter/link/orphan/SSOT 校验。
- 覆盖设计点: Wiki Status。
- 前置条件: 构造健康与多种损坏 Wiki fixture。
- 操作 / 触发: 执行 status human/JSON。
- 期望结果: 健康 Wiki ready；各损坏项产生稳定 issue，不声称 runtime readiness。
- 验证方式: inspector unit tests + CLI integration tests。

### ST-004 `.spec` stage、show 与 validate

- 关联成功标准: 完整 stage/artifact 合同和确定性 status/show/validate。
- 覆盖设计点: `.spec` Workflow。
- 前置条件: 不同 stage、缺失 artifact、非法 metadata fixtures。
- 操作 / 触发: status/show/validate。
- 期望结果: required artifact 随 stage 单调增加；非法输入返回 blocking issues；artifact show 不能越界。
- 验证方式: change core unit tests + CLI tests。

### ST-005 普通、child 和 parent 归档

- 关联成功标准: archive 完整阶段与 parent/child 一致性。
- 覆盖设计点: `.spec` Workflow、原子性。
- 前置条件: verification full/pass change 和 multi-change fixtures。
- 操作 / 触发: archive 普通/child/parent，并测试目标冲突。
- 期望结果: 原子移动、child 同步 parent、parent 只在全部 child 实际归档后成功，冲突不覆盖。
- 验证方式: archive integration tests。

### ST-006 路径安全失败关闭

- 关联成功标准: canonical path 与 archive 安全。
- 覆盖设计点: 路径安全。
- 前置条件: traversal、绝对路径、UNC/drive、symlink escape fixtures。
- 操作 / 触发: show/validate/archive/init 目标解析。
- 期望结果: 全部逃逸在读写前被拒绝，仓库外文件不变。
- 验证方式: path guard unit/integration tests。

### ST-007 旧 runtime 当前合同清零

- 关联成功标准: current source/docs 无 index/knowledge/runtime 产品合同。
- 覆盖设计点: breaking redesign、current authority。
- 前置条件: Lite 完整源码树。
- 操作 / 触发: current-surface contract scan。
- 期望结果: Rust/Cargo/native bridge/旧命令不存在；archive 被排除；导航 `INDEX.md` 保留。
- 验证方式: root Vitest contract tests + file inventory。

### ST-008 自举完整闭环

- 关联成功标准: 新 CLI 可验证并归档自身 change。
- 覆盖设计点: 全部设计点。
- 前置条件: review/test reports full/pass，所有 tasks 完成。
- 操作 / 触发: 使用构建后的 `spec-wiki-lite validate/archive build-specwiki-lite`。
- 期望结果: validate 通过，change 移入 dated archive，最终工作区无 active change。
- 验证方式: built CLI + 文件系统核验。

## 覆盖矩阵

| 成功标准 | 系统测试用例 | 验证方式 |
| --- | --- | --- |
| package/bin 与无 native binary | ST-001 | pack smoke |
| init/update/Codex-only | ST-002 | integration |
| Wiki 静态检查 | ST-003 | unit + integration |
| stage/show/validate | ST-004 | unit + CLI |
| archive 与 parent/child | ST-005 | integration |
| path safety | ST-006 | security regression |
| 旧 runtime 清零 | ST-007 | contract scan |
| 自举归档 | ST-008 | built CLI |

## 边界与异常

- npm registry 名称状态不作为测试依赖。
- `status` 的 Wiki issue 不等于 I/O failure；validate blocking 使用退出码 2。
- archive target 已存在、parent 不一致、报告非 full/pass 均禁止写入。

## 验证数据与环境

- Node.js >=20.19、pnpm workspace、Windows 临时目录。
- fixture 只写系统 temp 或测试创建的 repo，不访问外部服务。

## 未覆盖项

- 真实 Linux/macOS 安装未在当前 Windows 环境执行；以无 os/cpu 限制、path 单元测试和 tarball 合同作为替代证据。
- npm publish 不在范围内。

## 参考资料

- `proposal.md`
- `design.md`
