# add-multilingual-wiki-bootstrap

## 问题

SpecWiki Lite 当前只分发英文 Wiki scaffold，目录与文件名固定为 `00-conventions`、`01-project`、`02-development`、`03-architecture` 和 `04-reference`。项目无法选择中文 Wiki，也没有统一项目配置承载语言与未来 LLM 设置。

当前根 `.wiki/INDEX.md` 只是普通占位首页，不会明确要求 Codex 探索仓库、填充栏目并把占位页替换为项目事实。因此 `init` 虽能得到结构健康的 Markdown，却不能表达“Wiki 尚待初始化”和下一步动作。

## 目标

- 支持 `zh` 与 `en` 两套 Wiki 目录名、文件名和正文，默认中文。
- 使用 `.wiki/config.yaml` 持久化 `wiki.language`，并为未来 `llm` 配置保留同一扩展入口。
- 让 `init --language zh|en` 写入配置；让 `update` 读取配置并按目标语言同步资产。
- 将根 `INDEX.md` 设计为一次性初始化任务页，指导 Codex 按需探索仓库并最终替换为正式首页。
- 在 `status` 中报告当前语言和 bootstrap 状态，并让 `wiki-continue` 在无 active change 时路由未完成的 bootstrap。
- 为旧英文 scaffold 提供失败关闭的受控中文迁移，不覆盖用户修改或未登记页面。

## 非目标

- 不本地化 CLI 命令、JSON 字段、`.spec` artifact 名、Skill 名或 Skill 目录。
- 不实现 LLM provider、model、调用预算或运行逻辑；本 change 只保留配置扩展位置。
- 不自动翻译用户自建或已修改的 Wiki 正文。
- 不恢复代码扫描、索引、知识图谱、SQLite、native core 或多宿主能力。
- 不改写 `.spec/archive/**` 历史证据。

## 成功标准

- 空项目执行 `spec-wiki-lite init --host codex` 后写入 `.wiki/config.yaml`，语言为 `zh`，目录名、文件名和正文均为中文。
- `init --language en` 生成对应英文资产并写入 `wiki.language: en`。
- 配置的未知字段被保留；无效 YAML、未知 version 或非法 language 在任何 Wiki 写入前失败。
- 两套模板具有一致的信息架构、完整 frontmatter、有效导航和一次性 bootstrap 标记。
- bootstrap 任务页存在时 `status` 报告 `bootstrapPending: true` 且项目未 ready；替换为正式首页后恢复 ready。
- 旧英文 scaffold 只有在仍等于内置模板时才迁移；用户修改、目标冲突或路径逃逸使迁移原子失败。
- `--force` 只允许刷新 managed 页面，不能覆盖 scaffold 或未登记用户页面。
- staged tarball 同时包含中英文模板，并可分别完成中文默认与显式英文初始化 smoke。
- package/root tests、lint、两层 typecheck、build、pack evidence、Wiki 校验和 `git diff --check` 全部通过。

## 影响范围

- `core/assets`：语言化 registry、配置读写、ownership 同步和受控迁移。
- `core/wiki` / `core/status`：语言与 bootstrap 状态。
- CLI / init orchestration：`--language` 参数和配置写入。
- package assets：中英文 Wiki 模板与 `wiki-continue` 路由说明。
- README、当前 Wiki、capability 和发布合同。

## 交付形态

single-change

配置、双语模板、bootstrap 状态和迁移共同形成一个可独立验收的初始化合同，不拆分为相互不可用的中间交付。

## 风险

- 语言切换同时改变多个路径，预检或回滚不完整会留下双语混合状态。
- 旧英文 scaffold 与新英文信息架构不同，必须显式识别历史模板，不能把用户内容误判为 package 资产。
- 根任务页本身结构健康，但业务上尚未完成，status 必须区分静态健康与 bootstrap readiness。
- `.wiki/config.yaml` 与正式 Markdown 同目录，inspector 必须继续只检查 Markdown。

## 参考资料

- 用户确认的 SpecWiki Lite 多语言 Wiki 初始化计划。
- 原 SpecWiki `origin/main` 的 `.wiki/config.yaml` 项目级配置合同；目标落点：Lite 的语言与未来 LLM 统一配置入口；采用方式：沿用配置职责，不迁移旧 runtime schema。
- 来源：本机 `E:/project/!byAI/UniSpec` 的 `@uni-sw/unispec@0.1.0`，以及 `unispec init project --tools codex` 的实际输出；目标落点：Lite 一次性初始化任务页、五栏目骨架和 bootstrap 路由；采用方式：行为结构改写，不直接复制源码、模板正文或产品命名。
