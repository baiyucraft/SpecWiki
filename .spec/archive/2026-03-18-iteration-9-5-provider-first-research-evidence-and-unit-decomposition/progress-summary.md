# 9.5 迭代进度摘要

## 当前状态

- UniSpec tasks 进度：`28/29`
- 当前样本范围：`storybook + dagger`
- `5.3` 已完成；`citation / diagram / evidence` 已不再是双样本 95% 对标的主要短板
- 当前 change 仍不能 archive，剩余唯一门槛是 `6.4`

## 已完成内容

### 1. 验收口径与专项范围

- 已锁定 `storybook + dagger` 为本轮页面质量专项样本
- 已固定 `overall_match_rate = matched / reference` 的 95% 验收口径
- 已把 `missing / collapsed / low-fidelity / extra generated` 收成专项报告正式指标

### 2. Core 主链与 provider-first research

- workflow 已通过统一入口选择 runtime research provider，不再默认硬绑 `StructuralResearchProvider`
- `system / domain / unit` research 已接入 provider-first 主链，父页正式消费 child digest
- docs-backed 页面已按 reference 式主章节骨架收敛，不再允许随意回退到英文 raw docs 标题或泛化模板
- `.qoder / .codex / .serena` 这类隐藏派生语料已从 core 正式输入中剔除

### 3. Citation / diagram / Markdown contract

- section-scoped citation digest、evidence block、diagram draft 已成为正式 compose 输出
- citation / evidence / Mermaid 已落到最终 Markdown，可直接从 `.wiki/*.md` 统计
- docs-backed 页面已补齐 `cite -> 目录 -> 简介 -> 项目结构 -> 核心组件 -> 架构总览 -> 详细组件分析 -> 依赖关系分析 -> 性能考量 -> 故障排查指南 -> 结论 -> 附录` 这类主章节骨架 contract
- runtime compose 路径已修正 `overview / architecture` 消费输入，system page 会正式消费 `DomainIndex` digest，不再只是空壳页
- renderer 已补自动目录与 diagram fallback，最终 Markdown 会稳定落 `目录` 与 `章节结构图`

### 4. LLM 预算与停止条件 contract

- LLM hard-stop 预算已改为 steering 可配置，并提高默认值
- 已引入三层停止条件 contract：
  - `workflow stop`
  - `unit research stop`
  - `llm turn stop`
- 已引入显式结构：
  - `ResearchStopReason`
  - `ResearchSessionStats`
  - `PageResearchSessionResult`
- 单页 provider research loop 已支持“连续两轮无有效增量即停”
- workflow / checkpoint / debug trace 已能区分：
  - `completed`
  - `no_further_tool_calls`
  - `no_meaningful_delta`
  - `turn_budget_exhausted`
  - `call_budget_rejected`
  - `provider_error`
  - `invalid_output`
- research cache input hash 已纳入 LLM research contract；预算/模型变化后，旧的 `call_budget_rejected` research cache 不会继续复用

### 5. 专项报告观测能力

- `scripts/collect-reference-project-reports.mjs` 已能直接从 `.wiki/.cache/wiki-cache.db` 的 `research_cache` 读取 unit research stop reason
- 已修复 `storybook` 级别 research cache 在报告脚本中触发 `sqlite3 ENOBUFS` 的问题；stop reason 与 session 指标现已改为 SQLite 侧聚合，不再把整列 `result` JSON 拉回 Node 端
- 专项报告已新增 stop reason 口径：
  - `budget_stopped_pages`
  - `stalled_pages`
  - `invalid_output_pages`
  - `provider_failed_pages`

### 6. 2026-03-17 新增收敛动作

- system/index page 的 compose 骨架已继续向 reference 可识别章节收敛：
  - `简介`
  - `架构总览`
  - `项目结构`
  - `核心组件`
  - `依赖关系分析`
  - `结论`
- system/index page 已补 digest 关系摘要、结论生成与 fallback diagram，不再固定 `diagrams: []`
- citation 下限已继续提高：
  - `MIN_PAGE_CITATIONS = 14`
  - `MIN_SECTION_CITATIONS = 3`
- renderer 已补：
  - 无 `目录` 时自动生成 `## 目录`
  - 页面无 diagram 但章节足够时自动生成 `章节结构图`
- 默认 page research 预算已继续抬高：
  - `max_research_calls: 96 -> 256`
- 已补对应测试，覆盖：
  - system page consume child digest citations / key sources
  - index page relationship section / fallback diagram
  - parent page consume child digests and keep citation density
  - generated toc / outline diagram
  - system page input digest 取 `DomainIndex`
  - LLM 预算变化触发 cache hash 变化
  - higher default budget 真正影响 page research slots
- structural module 与 docs-heavy extra pages 又补了一轮通用收口：
  - 当同一 structural module 同时命中多个 domain 时，只保留最匹配的主 domain，避免跨域重复成页
  - docs-backed `ConceptGuide` 已继续把 `CHANGELOG / releases / contribute/* / roadmap / upgrading` 一类 meta docs 收回 evidence-only
  - 顶层 `CONTRIBUTING.md` 保留为 docs-backed 页面，不再被一刀切误杀
- 已补对应 planner 测试，覆盖：
  - `overlapping_structural_domains_keep_only_best_fitting_module_owner`
  - `docs_guide_skips_meta_reference_noise_pages`

## 已验证内容

- `cargo test -p wiki-core compose_system_page_consumes_child_digest_citations_and_key_sources`
- `cargo test -p wiki-core compose_index_page_adds_relationship_section_and_fallback_diagram`
- `cargo test -p wiki-core render_page_draft_inserts_generated_toc_for_multi_section_page`
- `cargo test -p wiki-core render_page_draft_generates_outline_diagram_when_missing`
- `cargo test -p wiki-core collect_compose_input_digests_uses_domain_indexes_for_system_pages`
- `cargo test -p wiki-core higher_default_budget_scales_page_research_slots`
- `cargo test -p wiki-core zero_max_research_calls_falls_back_to_default`
- `cargo test -p wiki-core unit_input_hash_changes_when_llm_budget_contract_changes`
- `cargo test -p wiki-core missing_file_returns_defaults`
- `cargo test -p wiki-core --test hierarchy topic_page_planning`
- `cargo fmt --all`
- `node scripts/collect-reference-project-reports.mjs storybook --jobs 1 --run-mode warm --init-timeout-minutes 120 --change iteration-9-5-provider-first-research-evidence-and-unit-decomposition`
- `node scripts/collect-reference-project-reports.mjs dagger --jobs 1 --run-mode warm --init-timeout-minutes 120 --change iteration-9-5-provider-first-research-evidence-and-unit-decomposition`
- `node scripts/collect-reference-project-reports.mjs storybook dagger --jobs 1 --skip-init --change iteration-9-5-provider-first-research-evidence-and-unit-decomposition`
- `cargo test -p wiki-core --test hierarchy topic_page_planning -- --nocapture`

## 2026-03-18 新验证

- planner 新增两条通用收口：
  - `ConfigDoc` 聚合页与 synthetic `DomainIndex` 同路径时，跳过后者
  - `single-module` 回收扩展到 `IntegrationPlatform / ApiSurface`，但只有在 topic/family 覆盖足够丰富时才回收，避免误杀简单结构页
- `CoreRuntime` fallback `核心模块` 不再额外生成 runtime signal family/topic 子树；同时 fallback 覆盖口径已纳入 `TestingInfra / ApiReference`，避免测试/API 模块再次回流到 `核心模块`
- `BuildSystem` 域识别已补充 `gradle / maven / bazel / buck` 关键词，避免构建容器目录继续回流到 fallback core
- 顶层 `examples / demo / sample / fixture / sandbox` 等明显容器目录已按低信号模块过滤，不再单独成 `ModuleDoc`
- `ApiReference` 域所有权已扩展为：
  - `public_surface >= 2`
  - 或模块内部存在显式 API 文件信号（如 `addons.ts`、`store.ts`、`public-types.ts`、`api/**` 等）
  这样即使符号解析不足，API 容器模块也不会再次掉回 fallback `核心模块`
- `ConfigReference` 对 `package.json`、`Cargo.toml`、`pyproject.toml`、`pom.xml`、`build.gradle*`、`settings.gradle*` 一类泛 manifest 已改为优先收回聚合 `配置参考`，不再默认长出低价值单页如 `Package.md`
- 新增/更新测试覆盖：
  - dense raw config aggregate 不再与 `DomainIndex` 撞路径
  - Hilt topic family 覆盖后可回收 generic framework module page
  - fallback `核心模块` 不再重复生成 `核心概念 / 高级特性与扩展` 这类 runtime signal 子树
  - API / Testing / Build 已拥有域归属的模块不会再被 fallback `核心模块` 重新认领
  - 泛 manifest config surface 不再长出单独噪声页
- provider 直连结论：
  - 当前 `wiki.dev.yaml` 指向的 proxy 上，`/v1/chat/completions` 可正常返回
  - `/v1/responses` 的 plain-text 最小请求可正常返回
  - `/v1/responses` + `text.format=json_schema` 会返回 `upstream_error`，说明该 proxy 的 structured responses 能力当前不稳定；在此 provider 上做真实专项验证时，应优先使用 `chat_completions`
- `dagger` 实仓短跑观测：
  - 使用 `responses` 配置时，cold init 很难稳定进入 `.wiki/.cache`，更像是 provider 结构化 responses 阶段不稳定
  - 临时切回 `chat_completions` 后，cold init 已能进入实仓 planner/research 主链
  - 最新短跑中间态（非最终报告）：
    - `knowledge_units = 68`
    - `ModuleDoc = 39`
    - `ApiDoc = 11`
    - `TestDoc = 8`
    - `DomainIndex = 4`
  - 相比上一轮短跑缓存中的 `knowledge_units = 73`，本轮又下降了 `5`
- 本轮继续新增 3 条通用收口，目标仍然是压 `extra generated pages`，不是针对 `storybook / dagger` 写特判：
  - 源码树内嵌的 Markdown/README 不再被误判为 docs-backed 页面，补充过滤 `main/java`、`main/kotlin`、`scripts`、`examples` 等嵌入式源码路径
  - 当某个 docs corpus 已足够稠密时，repo 根 `README.md` 不再单独生成 `ConceptGuide` 页面，而是只作为 overview/research 证据输入
  - 对只有少量叶子页、且没有内部层级关系的 domain，停止再生成 synthetic `DomainIndex`
- 对应新增定向测试已补齐并通过：
  - dense docs corpus 下，root `README.md` 不再单独成页
  - thin domain 不再生成额外 synthetic index
  - 原有 `docs shadow / dense config / hilt / dagger signal` 相关回归未被打坏
- 本轮定向回归：
  - `cargo fmt --all`
  - `cargo test -p wiki-core --test hierarchy topic_page_planning -- --nocapture`
- 新一轮 `dagger` 真实短跑虽然在长流程阶段超时，但中间态缓存已成功落库，且规划结果继续收敛：
  - `knowledge_units = 63`
  - `ModuleDoc = 35`
  - `ApiDoc = 15`
  - `TestDoc = 8`
  - `ConceptGuide = 2`
  - `ConfigDoc = 1`
  - `DomainIndex = 0`
  - 相比此前的 `knowledge_units = 69`，本轮又下降了 `6`
  - 说明这轮收口已经把 synthetic domain index 噪声彻底压掉，并继续减少了 planner 侧额外成页

## 当前剩余任务

- `6.4`：只有在两项目同时满足全部完成门槛时，才能将本轮标记为完成

## 当前主要阻塞与风险

### 1. Storybook

- 最新 `reuse report` 已刷新到：
  - `overall_match_rate = 176/176 = 100%`
  - `missing pages = 0`
  - `collapsed pages = 0`
  - `low-fidelity matched pages = 176`
  - `extra generated pages = 165`
  - `budget_stopped_pages = 0`
  - `provider_failed_pages = 0`
  - `citation_density = 69.73 / 77.72`
  - `generatedDiagramPages = 220 / 176`
  - `main_outline_shortfall = 112`
- 这轮通用去重与 docs-noise 收口后：
  - `generated pages = 265 -> 233 -> 220`
  - `extra generated pages = 213 -> 178 -> 165`
- `citation / Mermaid` 已不再是主短板，但 `evidence block` 密度仍有单项目尾差
- 当前主短板已切换为：
  - 单页章节拆分仍比 reference 粗
  - docs-backed 页面主章节骨架仍偏离 reference
  - 解释层正文密度仍低于 reference
  - `extra generated pages = 165` 仍偏高

### 2. Dagger

- 最新 `reuse report` 已刷新到：
  - `overall_match_rate = 65/65 = 100%`
  - `missing pages = 0`
  - `collapsed pages = 0`
  - `low-fidelity matched pages = 65`
  - `extra generated pages = 63`
  - `budget_stopped_pages = 0`
  - `provider_failed_pages = 0`
  - `citation_density = 83.71 / 79.69`
  - `generatedDiagramPages = 107 / 65`
  - `main_outline_shortfall = 61`
- 这轮 provider 配置更新后：
  - `budget_stopped_pages = 9 -> 0`
  - `extra generated pages = 72 -> 63`
- 这轮 planner 通用收口后的最新真实中间态继续下降到：
  - `knowledge_units = 63`
  - `DomainIndex = 0`
  - 说明当前噪声页已不再主要来自 synthetic domain index，而是仍集中在 `ModuleDoc / ApiDoc`
- `citation / evidence / Mermaid` 已不再是主短板
- 当前主短板已切换为：
  - 单页章节拆分仍比 reference 粗
  - docs-backed 页面主章节骨架仍偏离 reference
  - 解释层正文密度仍低于 reference
  - `extra generated pages = 63` 仍偏高

### 3. 完成门槛尚未满足

- 当前尚不能宣称本轮达到 `6.4`
- 当前双样本专项最新状态：
  - `storybook = 100% / missing=0 / collapsed=0 / low-fidelity=176 / extra=165`
  - `dagger = 100% / missing=0 / collapsed=0 / low-fidelity=65 / extra=63`
- 最新双样本汇总中的高频差距只剩：
  - 单页章节拆分粗，主题混杂在同一页里
  - docs-backed 页面主章节骨架仍偏离 reference
  - 解释层正文密度仍低于 reference
  - storybook 仍有 `evidence block` 密度尾差
  - 仍存在明显 page collapse
- 因此当前只能认为：
  - `5.3` 已满足
  - `6.4` 仍未满足

## 下一步建议

1. 优先继续压 `low-fidelity matched pages`，方向转到 `decomposition / outline / 正文密度`，不要再把主要精力放在 citation / diagram 堆叠上
2. 优先回收 `extra generated pages`，尤其是跨 domain 重复成页与低价值派生页
3. 继续收紧 docs-backed 页面骨架保真，减少泛化模板回退
4. 当 `page collapse + docs-backed 骨架 + 正文密度 + extra pages` 四项都明显收敛后，再重新判断是否满足 `6.4`
