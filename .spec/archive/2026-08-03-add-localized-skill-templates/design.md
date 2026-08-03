# add-localized-skill-templates 设计方案

## 方案概述

SpecWiki Lite 将 package 内 Skill 资产由单语言扁平目录改为与 Wiki 相同的 locale registry。`.wiki/config.yaml` 的 `wiki.language` 是唯一语言 SSOT；`projectAssetsForLanguage(language)` 返回对应语言的 Wiki 文件和完整 Skill 文件树。安装目标保持 `.agents/skills/wiki-*/**`，因此语言切换只替换 package 登记内容，不改变调用标识或目录。

Skill 正文继续以 Lite 当前 8 个阶段职责为骨架，并吸收 UniSpec 0.1.0 的可复用行为：权威状态路由、research 与 parent/child 约束、proposal/design 边界、TDD 规划、apply 证据更新、full review、技术栈标准和 fail-closed archive。所有内容按 Lite CLI、`.wiki` 与 `.spec` 合同重写。

## 资产结构与 Inventory

package 结构：

```text
assets/skills/
├── zh/
│   ├── wiki-continue/SKILL.md
│   ├── wiki-explore/{SKILL.md,references/research-template.md}
│   ├── wiki-propose/{SKILL.md,references/research-template.md,references/proposal-template.md}
│   ├── wiki-design/{SKILL.md,references/research-template.md,references/design-template.md}
│   ├── wiki-plan/{SKILL.md,references/system-tests-template.md,references/unit-tests-template.md,references/tasks-template.md,references/browser-automation.md}
│   ├── wiki-apply/SKILL.md
│   ├── wiki-review/{SKILL.md,references/review-report-template.md,references/test-report-template.md,references/review-standard.md,references/review-standard.frontend.md,references/review-standard.go.md,references/review-standard.java.md,references/review-standard.python.md}
│   └── wiki-archive/SKILL.md
└── en/
    └── <相同英文稳定路径>
```

每种语言登记 24 个文件：8 个 `SKILL.md` 和 16 个 references。安装目标路径不含 locale：

```text
.agents/skills/wiki-*/SKILL.md
.agents/skills/wiki-*/references/*.md
```

Registry 保留 `PROJECT_SKILL_NAMES` 作为 status 的稳定顺序，并增加：

- `projectSkillAssetsForLanguage(language)`：返回该语言全部 24 个 `ownership: skill` 资产。
- `projectSkillAssetsByNameForLanguage(language)`：按 8 个 Skill 分组，供 status 验证完整 inventory。
- `projectAssetsForLanguage(language)`：合并目标 Wiki 与 Skill 资产。

旧的非语言化 `PROJECT_SKILL_ASSETS` 不再作为运行时 registry；如为测试或公共导出保留名称，其值必须明确绑定目标语言而不能绕过配置。

## 同步与 Ownership

现有 `ownership: skill` 的同步语义继续适用：

- 目标缺失：创建。
- 目标内容不同：普通 `init/update` 即覆盖，不需要 `--force`。
- 目标内容相同：unchanged。
- Skill 目录中未登记文件：不扫描为删除候选，不删除、不翻译、不改名。
- 配置解析发生在 registry 选择和写操作前；无效配置导致零 Skill 写入。

语言切换只参与 Skill 资产选择，不加入 Wiki migration source/target 预检。Wiki 继续执行原有 scaffold/managed 迁移；Skill 因稳定目标路径和 package-owned 语义，在同一同步事务中直接覆盖为目标语言。若任一写操作失败，现有 snapshot rollback 恢复所有已触碰 Wiki、Skill 与配置文件。

## Status Readiness

`status.skills[]` JSON 结构保持：

```ts
type SkillStatus = {
  installed: boolean;
  name: string;
  path: string;
};
```

其中 `path` 仍指该 Skill 的 `SKILL.md`。计算方式改为：

1. 读取并严格校验 `.wiki/config.yaml`，获得目标 `WikiLanguage`。
2. 按 Skill 分组取得该语言登记资产。
3. 对组内每个文件检查：路径存在、是普通文件、UTF-8 内容与 package asset 完全一致。
4. 全部通过才令该 Skill `installed: true`。

任一 `SKILL.md` 或 reference 缺失、被修改、仍为另一语言或旧版本均返回 false；`ProjectStatusReport.ready` 继续要求所有 Skill installed。读取失败按未安装处理，不把异常内容误报为 ready。

## 八个 Skill 内容边界

### wiki-continue

- `status --json`、`show`、strict validate 是权威，不用单纯文件存在性绕过 metadata consistency。
- 处理 standalone、parent/child、order/dependsOn、bootstrap pending 和 stage/artifact 矛盾。
- 每次只路由一个目标 Skill，不直接推进 stage、签发报告或归档。

### wiki-explore

- 只问阻塞性必要问题；research 只记录会改变 scope、ownership、risk 或 delivery shape 的证据。
- 明确 single-change standalone stub、multi-change parent split、child stub 的 Lite-compatible metadata 示例。
- research 使用内置模板；不提前写 proposal/design/implementation。

### wiki-propose

- 校验 canonical kebab-case id、同名 active/archive 冲突和 parent/child 一致性。
- 复用 explore research，只对 proposal 缺口做定向补充。
- 用 proposal 模板写 problem/goals/non-goals/success criteria/impact/risks/references；不提前设计。

### wiki-design

- 从已接受 proposal 定义 page/asset ownership、接口、路径安全、failure/rollback 和 verification design。
- external/upstream 必须记录 source、target landing area、adoption mode。
- research/design 模板可用；不生成 system tests/tasks，不引入 frontend interaction standard。

### wiki-plan

- system tests 覆盖 normal/failure/boundary，稳定使用 `ST-*`；TDD 使用 `UT-*`。
- `tasks.md` 按 Red/Green/Refactor 排序并映射 success criteria、ST/UT 和命令证据。
- `browser-automation.md` 说明可选工具、环境/数据/权限/副作用前置条件、可重复断言和 fallback evidence；不绑定 Playwright 或内置 runner。

### wiki-apply

- 先检查用户授权、tasks `implementation-ready: true` 和 strict validate。
- 按 Red/Green/Refactor 执行；Red 证据必须与目标行为相关。
- 每项证据通过后立即更新 tasks；scope drift 返回 proposal/design/plan，失败时恢复安全状态。

### wiki-review

- 区分 full 与 partial；只有全范围、无 blocking finding、全部必需证据通过才写 full/pass。
- 根据真实 diff/domain 选择通用、frontend、Go、Java、Python standards，不按配置文件名机械判断。
- review/test report 映射 proposal success criteria、ST/UT、tasks、命令和风险；图片不能作为唯一 pass 证据。

### wiki-archive

- strict validate、full/pass 报告、tasks 完成和 archive target 无冲突是硬门禁。
- 只调用一次 `spec-wiki-lite archive <id>`，由 CLI 移动目录并同步 parent/child。
- 如实报告 `wiki-updates-made`、`wiki-updates-required` 或 `wiki-updates-not-needed`，不能用 archive 替代缺失的长期 Wiki 沉淀。

## 模板与稳定字段

所有 reference 文件名保持英文。中英文模板采用相同章节和机器字段：

- research：phase、service boundary、evidence、findings、artifact impact。
- proposal：problem、goals、non-goals、success criteria、impact scope、delivery shape、risks、references。
- design：overview、interfaces、ownership/data flow、failure/rollback、verification、reference boundary。
- system tests：normal/failure/boundary `ST-*`、environment/data/evidence。
- unit tests：`UT-*` 对应 Test/Modify/Red/Green/Refactor。
- tasks：`implementation-ready`、Red/Green/Refactor、success mapping。
- review/test reports：首行稳定 frontmatter `review-result` / `verification-result` 与 `scope`，正文随语言本地化。
- review standards：blocking/non-blocking/tooling/non-goals/false positives。

Skill frontmatter 的 `name: wiki-*` 保持英文。`description` 与正文随 locale；示例中的 CLI、路径、stage 和 metadata keys 保持英文。

## 内容合同与禁用项

自动内容测试扫描 package 当前 Skill 资产和仓库安装面：

- 每个 `SKILL.md` 的 `name` 必须匹配目录。
- Markdown 中每个 `references/<file>` 引用必须对应同目录登记文件。
- zh 资产包含中文阶段说明，en 资产使用英文说明；稳定 token 保持一致。
- 禁止可执行 `unispec ...` 命令、CodeBuddy、多宿主、生成 agents、`frontend-interaction-standard`。
- `browser-automation.md` 可以列举项目已有工具，但不得强制 Playwright，不得声明 Lite 提供 browser runner。
- `assets/skills/**` 不包含 `unispec-*` 目录；仓库已有 `.agents/skills/unispec-*` 仅作为开发参考，不参与 registry 或 pack 断言。

## Wiki 与公共文档落点

更新 current authority：

- README：双语 Skill 跟随 `wiki.language`，24 文件 inventory，update 修复语义。
- `.wiki/06-设计文档/02-Agents设计.md`：本地化结构、阶段责任、模板与 UniSpec 边界。
- `.wiki/03-模块指南/01-assets.md`：locale registry、skill ownership 与 status 内容比对。
- `.wiki/04-对外方法/01-配置与项目产物.md`：语言 SSOT 同时控制 Wiki/Skills。
- `.wiki/05-规格基线/capabilities/codex-skill-distribution/spec.md`：本地化分发和 readiness。
- `.wiki/04-对外方法/02-v0.1.0发布合同.md`：tarball 双语 Skill 内容。

历史 `.spec/archive/**` 不改写。

## 原子性、失败与回滚

- 继续复用 `syncProjectAssets` 的预计算 operations、touched-path snapshots 和 reverse restore。
- Skill 新增 references 与 `SKILL.md` 一起进入同一 operations 集合；中途失败时恢复到同步前内容/缺失状态。
- 配置无效时在创建目录、计算 write operations 前失败。
- status 只读；read/compare 失败返回 installed false，不修改磁盘。
- 如实现发生 scope drift，回到相应 artifact 更新并重新 strict validate，不在 apply 中临时扩展产品边界。

## 参考边界

- 来源：`E:/project/!byAI/UniSpec`，package `@uni-sw/unispec@0.1.0`，参考其 8 个 `unispec-*` Skill 与 references。
- 目标落点：`packages/spec-wiki-lite/assets/skills/{zh,en}/wiki-*`、语言化 registry、status readiness、tests 与 current Wiki。
- 采用方式：结构和行为改写；不复制 UniSpec 源码、产品名、runtime、多宿主/生成 agents 内容，也不合入 `frontend-interaction-standard.md`。Playwright 专项指南被改写为工具中立的 `browser-automation.md`。

## 验证设计

- 单元/内容测试验证 24 文件 inventory、双语正文、reference closure、sync ownership、语言切换与 readiness。
- tarball smoke 在临时项目完成 zh/en init、双向 update、status 和完整文件树检查。
- current-surface scan 排除不适用来源词与旧产品合同，历史 archive 排除。
- 运行 package/root tests、lint、两层 typecheck、build、pack、Wiki strict validate 和两层 `git diff --check`。
