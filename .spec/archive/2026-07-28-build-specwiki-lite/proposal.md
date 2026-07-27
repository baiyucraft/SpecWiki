# build-specwiki-lite

## 问题

当前 `spec-wiki` 以 Rust runtime、代码扫描、符号索引、KnowledgeUnit 映射、SQLite cache 和多宿主 runtime skills 为产品主线。该结构使最基本的 `.wiki` 初始化与维护依赖 native binary、Windows x64 分发和复杂运行时状态，无法提供接近 UniSpec 的轻量安装、阶段推进和 Markdown-first 使用体验。

本 change 需要把产品收敛为独立的 `spec-wiki-lite`：只管理 `.wiki` 长期文档、`.spec` 阶段 artifact 和 Codex Skills，不再构建或持久化代码索引、知识图谱、knowledge projection 或 runtime cache。

## 目标

- 提供可独立安装的 `spec-wiki-lite@0.1.0` 和 `spec-wiki-lite` CLI。
- 使用纯 TypeScript 实现 `.wiki` 模板同步、静态健康检查和 `.spec` 完整阶段工作流。
- 只支持 Codex，并以 `.agents/skills` 作为 Skills 唯一资产位置。
- 保留 `.wiki/**/INDEX.md` 导航和正式 Markdown 页面，移除 index/knowledge/native runtime 产品面。
- 让 `lite` 分支完成后可作为一次 breaking redesign 直接合并回 `main`。

## 非目标

- 不保留旧 `spec-wiki` 包名、CLI 别名、旧字段或兼容 shim。
- 不支持 Claude、CodeBuddy 或其他宿主。
- 不提供 `query`、`sync`、`rebuild`、代码扫描、符号解析、知识图谱或 KnowledgeUnit 映射。
- 不发布 npm package，不创建 Git tag，不在本 change 中合并 `lite` 回 `main`。
- 不改写 `.spec/archive/**` 中的历史 artifact。

## 成功标准

- `npm pack` 产物身份为 `spec-wiki-lite@0.1.0`，bin 为 `spec-wiki-lite`，且不包含 Rust binary。
- CLI 只暴露 `init/status/show/validate/update/archive`，只接受 `--host codex`。
- 空仓库执行 `init` 后得到 `.wiki` 基线、`.spec` 目录和 `wiki-*` Codex Skills；重复执行幂等，自定义 Wiki 页面不被覆盖。
- `.spec` 支持 explore 到 archive 的完整 stage/artifact 合同，CLI 可确定性 status/show/validate/archive。
- Wiki 静态检查覆盖根与栏目 `INDEX.md`、frontmatter、相对链接、孤儿页面、SSOT 重复和路径安全。
- 当前源码、公开文档、help、Skills 和测试不再依赖 `wiki-index`、KnowledgeUnit、route groups、SQLite 或 runtime artifacts；历史 archive 排除。
- root/package tests、lint、build、pack smoke、自举 validate/archive 和 `git diff --check` 全部通过。

## 影响范围

- 删除 Rust workspace、四个 core crate、native build/staging 和 runtime forwarding。
- 重构 npm workspace、主 package、CLI、模板/Skills 资产、测试编排和发布合同。
- 重写 `.wiki` 当前总体设计、模块指南、对外方法、场景和 capability inventory。
- 新增纯 TypeScript `.spec` metadata、stage、validation 和 archive 实现。

## 交付形态

single-change

这是一个独立的 breaking redesign。所有改动共同形成一个可安装、可验证、可归档的 Lite 产品，任何中间状态都不是独立交付物。

## 风险

- 删除 native runtime 会造成较大删除面，必须用公开 surface 和打包测试防止残留或误删纯 Wiki 规则。
- SpecWiki 内置 `.spec` workflow 与 UniSpec 结构相近，必须明确来源并改写实现，避免隐式复制或双重 authority。
- Windows 上的路径、junction/symlink 和大小写行为可能造成目录逃逸，所有 change/artifact/archive 路径必须 fail closed。
- 大量历史 capability 只适用于旧 runtime，当前 authority 清理必须排除只读 archive。

## 未知项

- npm 名称在正式发布前可能被第三方占用；本 change 只记录当前 E404 快照，不把名称可用性当作永久保证。
- 非 Windows 平台只能通过平台无关测试和 package 合同验证，本轮环境不提供真实 Linux/macOS 安装证据。

## 参考资料

- 用户确认的 SpecWiki Lite 实施计划与包名调整。
- `.wiki/06-设计文档/05-产品基线与设计治理.md`
- `.wiki/04-对外方法/02-v0.2.0发布合同.md`
- 来源：本机 `C:/Program Files/nodejs/node_modules/unispec` 的 `@uni-sw/unispec@0.1.0`；目标落点：TypeScript `.spec` workflow、模板 ownership 和 Codex Skill 生成；采用方式：改写借鉴，不直接复制源码或模板正文。
