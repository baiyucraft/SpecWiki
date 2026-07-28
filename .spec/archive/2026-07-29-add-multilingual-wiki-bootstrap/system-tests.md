# add-multilingual-wiki-bootstrap 系统测试

## ST-01 默认中文初始化

- 在空目录执行 `spec-wiki-lite init --host codex`。
- 断言 `.wiki/config.yaml` 为 version 1 / language zh。
- 断言中文五栏目、中文文件名、中文正文、8 个 Skills 和空 `.spec` 目录存在。
- 断言 status 静态 Wiki 健康、`bootstrapPending: true`、项目 `ready: false`。

## ST-02 显式英文初始化

- 在空目录执行 `spec-wiki-lite init --host codex --language en`。
- 断言配置为 en，只生成英文当前模板路径，不生成中文路径。
- 断言英文导航健康且 bootstrap pending。

## ST-03 完成 bootstrap

- 初始化后用合法正式首页替换根 `INDEX.md` 并移除 marker。
- 断言 status 为 `wiki.ready: true`、`bootstrapPending: false`、项目 `ready: true`。

## ST-04 配置失败关闭

- 分别提供无效 YAML、未知 version、非法 language。
- 断言 init/update 失败，Wiki 和配置原内容不变。
- 断言配置未知字段在 init language override 后保留。

## ST-05 ownership 与幂等

- 连续 init/update，断言第二次全部 unchanged。
- 修改 scaffold、managed 和未登记页面后执行普通 update 与 `--force`。
- 断言 scaffold/用户页始终保留，managed 仅 force 更新，Skills 同步。

## ST-06 英中语言迁移

- 从未修改英文当前模板切换配置为 zh，update 后只剩中文登记资产。
- 从未修改中文当前模板切换配置为 en，结果对称。
- 断言未知用户页面保留，配置保留未知字段。

## ST-07 迁移冲突与回滚

- 修改来源 scaffold、创建不同内容的目标文件、构造 symlink/junction escape。
- 断言迁移在写入前失败并报告冲突路径。
- 注入中途文件操作失败，断言已触碰登记文件恢复原内容。

## ST-08 旧英文 scaffold 升级

- 使用提交 `e830627` 的英文 scaffold fixture 且无配置执行 update。
- 断言自动迁移为中文并创建中文配置。
- 修改任一旧 scaffold 后重试，断言 fail closed；显式写入 en 后可迁移为新英文结构。

## ST-09 Skill bootstrap 路由

- 在没有 active change且 bootstrap pending 的项目执行 `wiki-continue` 约定检查。
- 断言路由说明指向 `wiki-explore`。
- active change 存在时仍按 stage 路由，不被 bootstrap 覆盖。

## ST-10 发布包双语 smoke

- 从 staged tarball 安装到隔离目录。
- 分别完成默认中文 init、显式英文 init、status 和目录 inventory。
- 断言包内包含 `assets/wiki/zh/**`、`assets/wiki/en/**`、migration baseline、Skills、README、LICENSE 和 Node dist，不含 native/index/knowledge runtime。

## ST-11 当前仓库与长期合同

- 当前仓库使用 `.wiki/config.yaml` 的 zh 配置且 Wiki 健康。
- README、CLI、模块指南、capability、发布合同与实现一致。
- `.spec/archive/**` 无批量改写。
