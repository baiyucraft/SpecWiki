# add-localized-skill-templates 系统测试

## ST-01 默认中文 Skill bootstrap（正常）

- 在空临时项目执行 `spec-wiki-lite init --host codex`。
- 断言配置 `wiki.language: zh`，8 个 Skill 均安装，登记文件总数为 24。
- 断言中文 `SKILL.md`、references 路径和正文存在，英文 Skill 路径不写入项目。
- 断言 status 的 `wiki.language` 为 `zh`，所有 `skills[].installed` 为 true。

## ST-02 显式英文 Skill bootstrap（正常）

- 在另一空临时项目执行 `spec-wiki-lite init --host codex --language en`。
- 断言配置为 `en`，只安装英文正文和 references，Skill 标识与目录仍为 `wiki-*`。

## ST-03 双向语言切换（边界）

- 在 zh 项目把配置改为 en 后执行 `update`，再改为 zh 执行 `update`。
- 断言 24 个登记文件每次全部切换，目标语言内容与 package 完全一致，无双语混合登记文件。

## ST-04 ownership 修复与用户扩展（正常/边界）

- 修改或删除登记的 `SKILL.md` / reference 后执行普通 `update`，断言恢复 package 版本。
- 在 `.agents/skills/wiki-plan/` 新增未登记 `custom.md`，重复 update，断言文件保留。
- `status.skills[].installed` 对缺失、旧版本、修改或语言不一致的登记文件返回 false，修复后恢复 true。

## ST-05 配置失败关闭（失败）

- 分别提供无效 YAML、未知 `version`、非法 `wiki.language`。
- 断言 init/update 失败，任何 Wiki、Skill 和配置内容在失败前后相同。
- 断言配置中的未知字段和注释在合法语言切换后保留。

## ST-06 模板引用闭包与内容合同（正常/失败）

- 扫描 zh/en 所有 `SKILL.md`，断言每个 `references/<name>` 引用存在且 inventory 无遗漏。
- 断言 frontmatter `name: wiki-*` 与目录一致，locale 正文符合语言，reference 文件名、stage 和 metadata keys 稳定英文。
- 断言不残留 `unispec` 可执行命令、CodeBuddy、多宿主、生成 agents、`frontend-interaction-standard`；browser automation 不强制 Playwright。

## ST-07 发布包双语 smoke（正常）

- 从 staged tarball 安装到隔离目录，分别运行中文默认 init、英文 init 和双向 update。
- 断言 tarball 含双语 Skill/Wiki assets、README、LICENSE、dist/bin，不含 Rust/native/index/knowledge runtime。

## ST-08 当前仓库长期合同（回归）

- 当前仓库配置保持 zh；仓库 `.agents/skills/wiki-*` 与 zh package assets 一致。
- README、Agents 设计、assets 模块、capability 和发布合同描述一致；`.spec/archive/**` 无本 change 改写。
