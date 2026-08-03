# add-localized-skill-templates 实现任务

implementation-ready: true
implementation-mode: tdd

## Red

- [x] R1 新增 registry inventory 测试（UT-01、ST-01、ST-02）。
- [x] R2 新增 Skill 同步、双向切换、ownership、配置失败关闭和 rollback 测试（UT-02、ST-03 至 ST-05）。
- [x] R3 新增 status readiness、引用闭包、双语内容合同和禁用词扫描测试（UT-03、UT-04、ST-06、ST-08）。
- [x] R4 扩展 staged tarball smoke（UT-05、ST-07）。Focused Red：4 suites failed，10 tests failed；缺少 locale Skill registry、16 个 references、双语切换与逐文件 readiness。

## Green

- [x] G1 将 `assets/skills` 重组为 zh/en 两套 8 Skill 完整文件树，并按 UniSpec 参考改写主文档和 templates。
- [x] G2 将 registry 改为 locale-aware，登记 24 个 Skill 文件并提供按 Skill 分组的 inventory。
- [x] G3 扩展 sync operations，使目标语言全部 Skill 文件 package-owned 同步，保留未登记用户文件并复用原子 rollback。
- [x] G4 扩展 status，按目标语言逐文件内容比对计算 `skills[].installed`，读取失败按 not installed 处理。
- [x] G5 更新当前仓库 `.agents/skills/wiki-*` 为 zh 版本并安装全部 references；保留 `unispec-*` 参考 Skill 不动。
- [x] G6 更新 README、Wiki Agents/assets/config/capability/release 文档和测试脚本，形成长期合同。
- [x] G7 focused Green：package 4 suites / 31 tests，root 内容与 distribution 3 suites / 7 tests；当前 status 的 8 个 Skills 全部 installed，Wiki healthy。

## Refactor 与验证

- [x] F1 收口 `WikiLanguage`、registry 类型、inventory helper、错误消息和 JSON 结构；lint 与 root/package 两层 typecheck 通过。
- [x] F2 package 71 passed / 1 Windows 条件跳过，workspace 8 passed；root build、Wiki/status/strict validate 和 `git diff --check` 通过。
- [x] F3 pack evidence 通过：`spec-wiki-lite@0.1.0` tarball 84 files / 96.7 kB；Windows shell-aware smoke 完成中文默认、英文显式与 zh↔en update。
- [x] F4 当前产品源码/Wiki/8 个安装 Skills 扫描无旧可执行命令、CodeBuddy、多宿主、frontend interaction standard 或 Playwright 强制合同；既有 `.spec/archive/**` 无 diff。
- [x] F5 full review 与 verification 报告均为 full/pass；review-stage strict validate 通过，准备 verification strict validate 与 CLI archive。

## 成功标准映射

| 标准 | 测试 | 任务 |
| --- | --- | --- |
| 双语 24 文件安装 | ST-01、ST-02、UT-01 | R1、G1-G2 |
| 语言切换与 ownership | ST-03、ST-04、UT-02 | R2、G3 |
| 失败关闭与 readiness | ST-05、UT-03 | R2-R3、G4 |
| 模板/引用/内容合同 | ST-06、ST-08、UT-04 | R3、G1、G5-G6、F4 |
| 分发可用 | ST-07、UT-05 | R4、F3 |
