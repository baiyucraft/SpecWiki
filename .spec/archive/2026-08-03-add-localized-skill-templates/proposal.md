# add-localized-skill-templates

## 问题

SpecWiki Lite 当前分发的 8 个 `wiki-*` Skill 只有英文 `SKILL.md`，没有阶段 artifact 模板、review standards 或浏览器自动化证据指南。`.wiki/config.yaml` 已经控制 Wiki 语言，但 Skill 正文与 references 尚未跟随 `wiki.language`，导致默认中文项目仍安装英文工作流说明，也无法通过内置模板稳定地产出 proposal、design、tests、tasks、review 与 verification 证据。

现有 Skill 已建立 Lite 的阶段边界，但内容较薄，尚未完整覆盖 parent/child 依赖路由、research 复用、TDD Red/Green/Refactor、scope drift、full/partial review、技术栈 review standards 和 archive 后 Wiki 沉淀状态等约束。

## 目标

- 将 package Skill 资产重组为 `assets/skills/{zh,en}/wiki-*/**`，由 `wiki.language` 同时选择 Wiki、Skill 主文档和 references。
- 保持 8 个 Skill 的名称、安装目录、CLI、`.spec` artifact 文件名和机器字段英文稳定。
- 基于现有 Lite Skill 结构，吸收 UniSpec 0.1.0 中适用于 Lite 的阶段约束、artifact 模板和 review 标准。
- 为 explore/propose/design/plan/review 安装完整 references；增强 continue/apply/archive 主 `SKILL.md`。
- 让 Skill 登记文件始终由 package 覆盖修复，同时保留用户新增的未登记文件。
- 让 `status.skills[].installed` 只有在目标语言 Skill 的全部登记文件存在且与 package 内容一致时才为 true。
- 更新当前中文仓库的 `.agents/skills/wiki-*`、长期 Wiki、README、capability 与发布合同。

## 非目标

- 不安装或分发 `unispec-*` Skill，不引入 UniSpec runtime、源码依赖或产品命名。
- 不增加多宿主、生成式 agents、`skills.language` 配置或新的 CLI language 参数。
- 不本地化 Skill id、目录、reference 文件名、`.spec` artifact 文件名、stage、metadata、`review-result`、`verification-result` 或 `scope` 字段。
- 不合入 UniSpec 的 frontend interaction design standard。
- 不强制 Playwright，也不宣称 Lite 内置浏览器 runner。
- 不改写 `.spec/archive/**` 历史证据。

## 成功标准

- 默认中文和显式英文项目各安装 8 个 `SKILL.md` 与 16 个登记 references，共 24 个 Skill 文件。
- zh→en、en→zh 普通 `update` 覆盖全部登记 Skill 文件，正文与 references 跟随配置语言。
- 普通 `update` 修复被修改或缺失的登记 Skill 文件；用户在 Skill 目录新增的未登记文件保留。
- 无效 `.wiki/config.yaml` 在任何 Skill 写入前失败关闭。
- `status.skills[].installed` 对缺失、旧版本、被修改或语言不一致的任一登记文件返回 false，并使项目 not ready。
- 所有 `references/...` 引用均存在；中文资产使用中文正文，英文资产使用英文正文，稳定机器字段不翻译。
- 当前产品面不残留可执行 `unispec` 命令、CodeBuddy、多宿主、生成 agents、frontend-interaction-standard，browser automation 不绑定特定 runner。
- staged tarball 完成默认中文、显式英文及双向 language update smoke，并包含完整双语 Skill 树。
- package/root tests、lint、两层 typecheck、build、pack、Wiki 校验和 `git diff --check` 全部通过。

## 影响范围

- `core/assets`：语言化 Skill registry、完整文件 inventory 与同步。
- `core/status`：按目标语言和内容一致性计算 Skill readiness。
- `assets/skills/{zh,en}`：8 个本地化 Skill 与 references。
- package/root tests、tarball smoke 和 current-surface 分发扫描。
- README、Agents 设计、assets 模块、配置合同、capability 与发布合同。

## 交付形态

single-change

Skill 本地化、模板分发、同步 ownership 与 readiness 共同构成同一产品合同，拆分会产生语言切换后不可验证或模板缺失的中间状态。

## 风险

- references inventory 扩大后，如果 status 仍只检查 `SKILL.md`，会误报 ready。
- 语言切换若只覆盖部分登记文件，会留下中英文混合 Skill。
- 直接照搬 UniSpec 内容会带入不适用命令、runtime、多宿主或产品约束。
- Skill 目录同步若误删未登记文件，会破坏用户扩展。
- 双语模板中的 relative reference 容易漂移，必须自动验证引用完整性。

## 参考资料

- 用户确认的《SpecWiki Lite 双语 Skill 与模板增强计划》。
- 来源：本机 `E:/project/!byAI/UniSpec`，package `@uni-sw/unispec@0.1.0`；目标落点：Lite 的 8 个 `wiki-*` Skill、artifact templates、browser automation 指南和 review standards；采用方式：基于 Lite 现有结构进行行为与内容改写，不直接复制产品命名、源码或不适用内容。
