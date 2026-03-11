## 1. 协议与桥接

- [x] 1.1 扩展 `crates/wiki-core/src/transport/{dto,json_rpc}.rs` 与 `crates/wiki-core/src/main.rs`，为长流程加入可协商的 LLM 会话协议，并保持未协商时的现有 `progress / result / error` 兼容路径。
- [x] 1.2 扩展 `agents/codebuddy/src/runtime/{invokeCore,parseResult}.ts`，支持消费 `llm_request`、回写 `llm_response / llm_unavailable`，同时保持 thin Agent 边界和现有六个工具外壳不变。
- [x] 1.3 在 `crates/wiki-core` 内补齐 provider API 直连能力，并实现“provider 直连优先、未配置时回退 Agent bridge”的统一选择逻辑。

## 2. LLM 契约与缓存

- [x] 2.1 在 `crates/wiki-core` 内定义 LLM assist 输入/输出模型、prompt type、prompt version 和输入哈希策略，并补齐 `llm_cache` 的 SQLite 读写与 TTL / model 校验。
- [x] 2.2 扩展 `crates/wiki-core/src/domain/steering.rs` 与相关配置消费路径，加入 `llm` 配置块和 `pages.hints` 的实际读取/传递逻辑。
- [x] 2.3 新增 repo 根 `wiki.dev.yaml` 的本地 dev 配置覆盖能力，让开发环境默认可通过 dev 文件配置 provider API，并避免被 `.wiki/` runtime 清理误删。
- [x] 2.4 将 provider 配置调整为 `llm.providers.<provider>.models.<model>`，并把顶层 `llm.model` 收敛为 `provider/model` 选择语义。
- [x] 2.5 扩展 `crates/wiki-core/src/domain/steering.rs` 的 `llm` 配置，新增 provider 直连路径使用的 `parallel_requests` 并行度控制项，并允许通过 `wiki.dev.yaml` 覆盖。

## 3. Uncertainty Gate

- [x] 3.1 在 `crates/wiki-core/src/repo/scanner.rs` 中为 `FilePurpose` 兜底分类接入可缓存的 Uncertainty Gate，并保证无 LLM 时回退到现有 deterministic 规则。
- [x] 3.2 在 `crates/wiki-core/src/repo/hierarchy.rs` 与依赖语义判定路径中，为顶层目录晋升临界值、`module_kind` 兜底和低置信度跨模块关系接入 Uncertainty Gate。
- [x] 3.3 把 `uncertainty_gate` 的同类型高频判断改成批量模式；本轮至少要先把 `FilePurpose::Utility` 兜底改成批量请求，并保持单条缓存/回退语义。

## 4. 页面增强与图表达

- [x] 4.1 在 `crates/wiki-core/src/generation/*` 与 `crates/wiki-core/src/workflows/{init,update,rebuild}.rs` 中加入 `PageEnrichmentInput`、叶子优先增强编排和父页消费子摘要的链路。
- [x] 4.2 升级 `build_section_drafts` / `render_page_bundle`，让 overview、architecture、module、workflow 页面能写出 LLM 增强正文和受控 Mermaid fenced block，并在校验失败时回退到 deterministic 内容。
- [x] 4.3 调整 page input hash、增量 update 受影响集合和 cache 收口，确保只有事实或增强输入变化的页面才会重新请求和重写。
- [x] 4.4 在 `crates/wiki-core/src/llm/mod.rs` 与 `crates/wiki-core/src/workflows/page_render.rs` 中把 provider 直连页面增强改为“同深度有限并行 + 父子层串行”，并保持 Agent bridge 串行回退。

## 5. 验证与项目集

- [x] 5.1 为 Rust core 与 CodeBuddy Agent 增加自动化测试，覆盖 LLM 关闭、桥接不可用、缓存命中、增强成功、页面稳定性和协议回退场景。
- [x] 5.2 运行 `node scripts/run-test-projects.mjs` 对 `DESIGN.md` 定义的 19 个测试项目全量执行 `init`，输出并更新 `test-project-analysis.md`，逐项目分析增强后页面信息密度、graph facts 落页情况与 reference 差异。
- [x] 5.3 运行 `node scripts/test-wiki-lifecycle.mjs` 验证 `init -> status -> sync -> query -> update -> rebuild` 全链路在 LLM 关闭和桥接回退场景下都能保持正确状态流转。
- [x] 5.4 增加 provider 直连优先和 `wiki.dev.yaml` dev 覆盖的自动化测试，并更新项目分析脚本/说明，避免再把“未协商 Agent bridge”等同于“没有任何 LLM 路径”。
- [x] 5.5 升级 `scripts/collect-reference-project-reports.mjs`，支持流式输出项目/phase 进度和 `--jobs` 项目并行，并补充对应自动化测试或脚本级验证。

## 6. 注释与收尾

- [x] 6.1 按 `COMMENTING.md` 对本轮新增或修改的 `crates/wiki-core/**`、`agents/codebuddy/**` 注释做一次单独检查，补齐公共接口、核心类型和关键流程注释。
