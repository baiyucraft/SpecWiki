---
implementation-ready: true
---

# build-specwiki-lite 任务计划

## 任务总览

按可验收能力拆为资产同步、Wiki 健康、`.spec` validation、archive、安全/CLI、旧 runtime 清理和文档发行七个大 task。用户已明确要求按本计划完整实施，因此 implementation-ready 直接确认。

## 实现模式

tdd

先确认对应 UT 的 Red 失败，再做最小实现，局部通过后重构；每个提交边界保持相关测试通过。

## 1. 可安装的 Wiki 与 Codex Skills 资产

- [ ] 1.1 Red: UT-001 写入 asset ownership 与 init/update 幂等失败测试并确认失败
- [ ] 1.2 Green: UT-001 实现 assets registry、同步器、Wiki scaffold、`.spec` 初始化和八个 Codex Skills
- [ ] 1.3 Refactor: UT-001 收口 managed/scaffold/skill ownership 与原子写入

### CheckList

- [ ] 失败测试已确认
- [ ] 最小实现后测试通过
- [ ] 重构后测试仍通过
- [ ] 本大 task 局部质量检查通过
- [ ] 注释规范检查完成

## 2. Wiki 静态健康状态

- [ ] 2.1 Red: UT-002 写入 INDEX/frontmatter/link/orphan/SSOT 失败测试并确认失败
- [ ] 2.2 Green: UT-002 实现 Markdown/YAML 静态 inspector 和结构化 report
- [ ] 2.3 Refactor: UT-002 收口 issue taxonomy、目录排除和可达性分析

### CheckList

- [ ] 失败测试已确认
- [ ] 最小实现后测试通过
- [ ] 重构后测试仍通过
- [ ] 本大 task 局部质量检查通过
- [ ] 注释规范检查完成

## 3. `.spec` 状态、show 与 validate

- [ ] 3.1 Red: UT-003 写入 stage/required artifact 失败测试并确认失败
- [ ] 3.2 Green: UT-003 实现 metadata parser、stage schema、status/show/validate core
- [ ] 3.3 Refactor: UT-003 让所有 workflow 消费同一 artifact registry
- [ ] 3.4 Red: UT-004 写入 verification full/pass 失败测试并确认失败
- [ ] 3.5 Green: UT-004 实现 YAML frontmatter evidence validation
- [ ] 3.6 Refactor: UT-004 收口 blocking issue 和 strict validation 语义

### CheckList

- [ ] 失败测试已确认
- [ ] 最小实现后测试通过
- [ ] 重构后测试仍通过
- [ ] 本大 task 局部质量检查通过
- [ ] 注释规范检查完成

## 4. 原子 archive 与 multi-change 一致性

- [ ] 4.1 Red: UT-005 写入普通归档和目标冲突失败测试并确认失败
- [ ] 4.2 Green: UT-005 实现 dated target、precondition 和原子 rename
- [ ] 4.3 Refactor: UT-005 收口 injected clock 和 archive report
- [ ] 4.4 Red: UT-006 写入 child/parent 同步失败测试并确认失败
- [ ] 4.5 Green: UT-006 实现 parent YAML/split 同步和 parent archive gate
- [ ] 4.6 Refactor: UT-006 增加失败回滚与未知字段保留守卫

### CheckList

- [ ] 失败测试已确认
- [ ] 最小实现后测试通过
- [ ] 重构后测试仍通过
- [ ] 本大 task 局部质量检查通过
- [ ] 注释规范检查完成

## 5. 路径安全和 Lite CLI

- [ ] 5.1 Red: UT-007 写入 traversal/absolute/symlink escape 失败测试并确认失败
- [ ] 5.2 Green: UT-007 实现 canonical id、lexical/realpath containment guard
- [ ] 5.3 Refactor: UT-007 让所有读写入口统一经过 path guard
- [ ] 5.4 Red: UT-008 写入新命令闭集、Codex-only、退出码失败测试并确认失败
- [ ] 5.5 Green: UT-008 重写 CLI/bin/package exports 和 human/JSON renderer
- [ ] 5.6 Refactor: UT-008 删除旧 action/runtime API 并收口错误分类

### CheckList

- [ ] 失败测试已确认
- [ ] 最小实现后测试通过
- [ ] 重构后测试仍通过
- [ ] 本大 task 局部质量检查通过
- [ ] 注释规范检查完成

## 6. 删除 index/knowledge/native runtime 并重建发行

- [ ] 6.1 删除 Rust workspace、四 crate、runtime bridge、旧 Agents 适配和相关测试
- [ ] 6.2 重写 root test/build/stage/pack 脚本为纯 TypeScript `packages/spec-wiki-lite`
- [ ] 6.3 添加 ST-001/ST-007 distribution、tarball inventory 和 current-surface contract tests
- [ ] 6.4 更新 workspace/lockfile，验证 tarball 不含 native binary 且无 os/cpu 限制

### CheckList

- [ ] 失败测试已确认
- [ ] 最小实现后测试通过
- [ ] 重构后测试仍通过
- [ ] 本大 task 局部质量检查通过
- [ ] 注释规范检查完成

## 7. 当前 Wiki、公开合同与自举闭环

- [ ] 7.1 重写 README、总体/Agents/场景、模块和 CLI 文档为 SpecWiki Lite authority
- [ ] 7.2 删除旧 runtime capability current baseline，建立 Lite workflow/distribution capability 与 `v0.1.0` 发布合同
- [ ] 7.3 运行 ST-002 至 ST-008、lint/build/pack/link/frontmatter/diff 门禁并修复问题
- [ ] 7.4 完成 full review/test reports，使用新 CLI 自举 validate/archive

### CheckList

- [ ] 失败测试已确认
- [ ] 最小实现后测试通过
- [ ] 重构后测试仍通过
- [ ] 本大 task 局部质量检查通过
- [ ] 注释规范检查完成

## 用例到任务映射

| 系统测试用例 | 大 task | 小 task / 验证 |
| --- | --- | --- |
| ST-001 | 5, 6 | UT-008 / 6.2-6.4 |
| ST-002 | 1, 5 | UT-001 / UT-008 |
| ST-003 | 2 | UT-002 |
| ST-004 | 3, 5 | UT-003 / UT-004 / UT-008 |
| ST-005 | 4 | UT-005 / UT-006 |
| ST-006 | 5 | UT-007 |
| ST-007 | 6, 7 | 6.1-6.4 / 7.1-7.2 |
| ST-008 | 7 | 7.3-7.4 |

## 执行顺序

- 1/2/3/4/5 先以新 package 并行能力块逐步替代旧 wrapper。
- 6 在新 CLI/package tests 通过后删除旧 runtime。
- 7 最后统一 current authority、完整验证并归档。

## 暂缓事项

- npm publish、Git tag、Linux/macOS 真实安装和合并 `lite` 回 `main`。
