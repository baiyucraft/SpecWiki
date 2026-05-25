## 前置：9.4 前半程已完成的基础工作

> 以下任务在 DESIGN-CORE2.0 重设计之前已完成，建立了 family planner / dossier / research-first compose 的初步骨架。
> 2.0 重设计将这些骨架统一收编为四层 pipeline（Facts → Knowledge Planning → Research → Compose）。

- [x] P.1 引入 family-index / family-child / family-leaf-doc 页面类型和稳定 identity/path 规则
- [x] P.2 基于 docs anchors、public API、config surface 等实现 family candidate discovery
- [x] P.3 完成 family / topic / module 的收编、去重和父子关系规则
- [x] P.4 引入 FamilyDossier，扩展 dossier 输入源
- [x] P.5 把 research contract 从 section patch 升级到正式 compose 计划
- [x] P.6 实现 planner → dossier → research → compose → renderer 的显式顺序
- [x] P.7 降低固定模板主导权，让 renderer 主要消费 compose plan 和 child digest
- [x] P.8 在 runtime/state/cache 主链内持久化 family 页面和 child page digest
- [x] P.9 升级 change set / parent-page rebuild 传播逻辑
- [x] P.10 为 family planner、FamilyDossier、parent consume child digest 补齐 Rust 测试
- [x] P.11 以 storybook + dagger 为样本建立当前差距基线和 reference 对比报告

---

## 1. Knowledge Planning 层（替代旧 family planner + topic planner + module planner）

- [x] 1.1 在 `crates/wiki-core/src/domain/` 下新增 `knowledge.rs`，定义 `KnowledgeDomain`、`KnowledgeUnit`、`KnowledgeTree`、`DomainType`、`UnitType`、`UnitScope` 等核心数据结构，对齐 [DESIGN-CORE2.0.md § Layer 2](E:/project/!byAI/spec-wiki/DESIGN-CORE2.0.md)
- [x] 1.2 在 `crates/wiki-core/src/generation/` 下新增 `knowledge_planner.rs`，实现 `discover_knowledge_domains()`：从 ModuleTree + SymbolGraph + GraphAnalysis + ScanReport 中按规则发现知识域（CoreRuntime / Framework / PluginEcosystem / ApiReference / ConfigReference / ConceptGuide / TestingInfra 等），每个域收集 `DomainEvidence`（matched_modules / file_patterns / docs_anchors / api_surfaces / config_surfaces）
- [x] 1.3 在 `knowledge_planner.rs` 中实现 `plan_knowledge_units()`：按 DomainType 走不同拆分策略（模块结构拆 ModuleDoc / public API surface 拆 ApiDoc / docs anchor 拆 ConceptGuide / plugin 拆 ModuleDoc / 测试拆 TestDoc），输出 `Vec<KnowledgeUnit>`，包含父子关系和 UnitScope
- [x] 1.4 实现 `build_knowledge_tree()`：构建 `KnowledgeTree`，生成叶子优先的 `processing_order`（DFS 先子后父，对齐 CodeWiki `get_processing_order()`）
- [x] 1.5 实现 KnowledgeUnit → `.wiki/` 路径映射规则（Overview → `项目概述.md`，DomainIndex → `{domain_label}/{domain_label}.md`，ModuleDoc → `核心模块/{module_name}.md`，ApiDoc → `API参考/{api_group}.md` 等）
- [x] 1.6 用 Steering 配置扩展知识规划控制：`knowledge.force_domains` / `knowledge.suppress_domains` / `knowledge.unit_weight_threshold` / `knowledge.max_units_per_domain`
- [x] 1.7 重构现有 `planner.rs`：移除旧的 family / topic / module 三套并行规划逻辑，改为消费 `KnowledgeTree` 输出 `Vec<PlannedPage>`（PlannedPage 保留为 compose 层的输入类型，从 KnowledgeUnit 映射生成）

## 2. Research 层（替代旧 page_research + page_enrichment）

- [x] 2.1 在 `crates/wiki-core/src/domain/` 下新增 `research.rs`，定义 `SystemResearch`、`DomainResearch`、`UnitResearch`、`PlannedSection`、`EvidenceCluster`、`SourceCitation`、`DiagramSuggestion` 等数据结构，对齐 [DESIGN-CORE2.0.md § Layer 3](E:/project/!byAI/spec-wiki/DESIGN-CORE2.0.md)
- [x] 2.2 在 `crates/wiki-core/src/generation/` 下新增 `research_engine.rs`，定义 `ResearchEngine` trait（`research_system` / `research_domain` / `research_unit`）和 `ResearchDataSource` 抽象
- [x] 2.3 实现 R1: `research_system()`——从 FactsSnapshot 全量中产出 `SystemResearch`（project_name / description / type / target_users / system_boundary / tech_stack / architecture_pattern / key_domains），含 LLM 调用
- [x] 2.4 实现 R2: `research_domains()`——从 SystemResearch + 域内 Facts 产出 `DomainResearch[]`（每域一份：domain_summary / internal_structure / key_modules / key_apis / relationships / diagram），可并行执行
- [x] 2.5 实现 R3: `research_units()`——从 SystemResearch + DomainResearch + 单元 scope 内 Facts 产出 `UnitResearch[]`（positioning / summary / section_plan / evidence_clusters / diagram_suggestions / key_sources），按 processing_order 叶子优先执行
- [x] 2.6 实现 research 缓存：`research_cache` 表（research_type + target_id + input_hash），有 TTL（默认 7 天），Facts 变化时自动失效
- [x] 2.7 重构 `llm/mod.rs`：移除旧的 `research_page()` / `enrich_pages()` 双路径，统一为 `ResearchEngine` 的三层调用；保留 Uncertainty Gate 和事实层 LLM 辅助不变
- [x] ~~2.8 为每层 research 实现 deterministic fallback~~ → **已废弃**：不再支持退化，见 4.7

## 3. Compose 层（替代旧 renderer + sections + page_render workflow）

- [x] 3.1 在 `crates/wiki-core/src/domain/` 下新增或重构 `compose.rs`，定义 `PageDraft`、`SectionDraft`（含 citations）、`PageDigest`、`DiagramDraft` 等数据结构，对齐 [DESIGN-CORE2.0.md § Layer 4](E:/project/!byAI/spec-wiki/DESIGN-CORE2.0.md)
- [x] 3.2 在 `crates/wiki-core/src/generation/` 下新增 `compose_engine.rs`，定义 `ComposeEngine` trait（`compose_page` / `compose_index_page` / `compose_system_page`）
- [x] 3.3 实现 `compose_leaf_pages()`：消费 `UnitResearch`，按 `section_plan` 展开正文，绑定 `EvidenceCluster` 中的 `SourceCitation`，输出 `PageDraft` + `PageDigest`
- [x] 3.4 实现 `compose_parent_pages()`：消费 `UnitResearch` + 子页 `PageDigest[]`，在 child_digest_slots 位置注入子页摘要，输出 `PageDraft` + `PageDigest`
- [x] 3.5 实现 `compose_index_pages()`：消费 `DomainResearch` + 域内子页 `PageDigest[]`，输出 `PageDraft`
- [x] 3.6 实现 `compose_system_pages()`：消费 `SystemResearch` + 域索引 `PageDigest[]`，输出 Overview / Architecture 的 `PageDraft`
- [x] 3.7 实现 citation 密度保证：每个 section 至少 2-3 条源码引用，整页不低于 10 条；低于阈值时从 `EvidenceCluster` 补充
- [x] 3.8 重构 `renderer.rs`：简化为纯 Markdown 组装器，只负责将 `PageDraft.sections` 渲染为带 managed marker 的 `.md` 文件，不再承担内容生成逻辑
- [x] 3.9 重构 `sections.rs`：移除按 page_type 硬编码 slot 的逻辑，section plan 完全由 research 层的 `PlannedSection` 驱动

## 4. Workflow 与 Runtime 适配

- [x] 4.1 重构 `workflows/init.rs`：主链从 `scan → module_tree → plan → render` 改为 `facts → knowledge_planning → research → compose → assemble`，对齐 [DESIGN-CORE2.0.md § init workflow](E:/project/!byAI/spec-wiki/DESIGN-CORE2.0.md)
- [x] 4.2 重构 `workflows/page_render.rs`：添加 `run_compose_pipeline()` 共享函数，重构 `rebuild.rs` 使用新 pipeline，修复 rebuild/sync 相关测试
- [x] 4.3 在 SQLite state 中新增 `knowledge_domains` / `knowledge_units` / `research_cache` / `page_digests` 表，对齐 [DESIGN-CORE2.0.md § 存储层](E:/project/!byAI/spec-wiki/DESIGN-CORE2.0.md)
- [x] 4.4 重构 `workflows/update.rs`：使用 `run_compose_pipeline()` 替代旧 `prepare_page_artifacts_with_llm`，修复全部 update 测试
- [x] 4.5 更新 `domain/change_set.rs`：用知识树 pipeline (`discover_knowledge_domains` → `plan_knowledge_units` → `build_knowledge_tree` → `plan_pages_from_knowledge_tree`) 替代旧 `plan_pages()`，确保页面 ID 与 init 一致
- [x] 4.6 清理旧的 `domain/context.rs` 中 `RepoDossier / ModuleDossier / TopicDossier / FamilyDossier / ChildPageRollup / PageComposePlan` 等类型，统一收编为 `UnitScope` + `UnitResearch` + `PageDigest`
- [x] 4.7 移除 Research 层的 deterministic fallback，LLM-required 改造：
  - [x] 4.7.1 删除 `research_engine.rs` 中 `research_system_deterministic` / `research_domain_deterministic` / `research_unit_deterministic` 三个公开函数，重构为 `StructuralResearchProvider`（实现 `ResearchProvider` trait）
  - [x] 4.7.2 在 `research_engine.rs` 中定义 `ResearchProvider` trait（`research_system` / `research_domain` / `research_unit` 返回 `Result`），作为 LLM research 调用的统一入口
  - [x] 4.7.3 `StructuralResearchProvider` 作为当前默认实现，LlmRuntime 实现待 LLM research prompt 完善后替换
  - [x] 4.7.4 重构 `run_compose_pipeline()`：签名改为接受 `&dyn ResearchProvider`，返回 `io::Result<ComposePipelineOutput>`；内部调用 `provider.research_system()` 等替代旧的 deterministic 调用
  - [x] 4.7.5 更新 `init.rs` / `rebuild.rs` / `update.rs` 中对 `run_compose_pipeline()` 的调用，传入 `StructuralResearchProvider`，用 `?` 传播错误
- [x] 4.8 中断-保存-恢复（Checkpoint & Resume）机制：
  - [x] 4.8.1 在 `domain/` 下新增 `checkpoint.rs`，定义 `PipelineCheckpoint` / `PipelineStage` 数据结构，对齐 [DESIGN-CORE2.0.md § 中断-保存-恢复](E:/project/!byAI/spec-wiki/DESIGN-CORE2.0.md)
  - [x] 4.8.2 在 SQLite state 中新增 `pipeline_checkpoint` 表（`checkpoint_id` / `facts_input_hash` / `interrupted_stage` / `interrupted_target_id` / `error_message` / `created_at`）+ CRUD 函数
  - [x] 4.8.3 在 `run_compose_pipeline()` 中：每完成一个 research/compose 步骤后将结果写入 `research_cache` / `page_drafts` 缓存；LLM 调用失败时保存 `PipelineCheckpoint` 后返回错误
  - [x] 4.8.4 在 `init.rs` / `rebuild.rs` / `update.rs` 的入口处：检查 `pipeline_checkpoint` 表，若存在有效检查点且 `facts_input_hash` 匹配则从中断处恢复（跳过已缓存的 research/compose）；pipeline 完成后清除检查点
  - [x] 4.8.5 计算 `facts_input_hash`：基于 `ScanReport` + `ModuleTree` 的 fingerprint，用于判断检查点是否可恢复
- [x] 4.9 清理 Steering 中废弃的 LLM 子开关和旧参数：
  - [x] 4.9.1 移除 `LlmConfig` 中的 `content_enrichment_enabled` / `session_enabled` / `uncertainty_gate_enabled` 字段及其 `RawLlmConfig` 对应项
  - [x] 4.9.2 移除 `LlmConfig` 中的 `page_enrichment_max_input_tokens` / `page_enrichment_parallel_requests` / `session_max_context_tokens` / `session_max_recent_turns` / `uncertainty_gate_max_input_tokens` / `uncertainty_gate_parallel_requests` 字段
  - [x] 4.9.3 新增 `LlmConfig` 字段 `max_research_calls: usize` 和 `max_compose_calls: usize`，替代旧的 `max_calls` 单一上限
  - [x] 4.9.4 更新 `normalize_llm_config()` 和 `apply_raw_llm_config()` 函数，移除对旧字段的处理
  - [x] 4.9.5 更新 `llm/mod.rs` 中对旧 `LlmConfig` 字段的引用

## 5. 数据模型清理与统一

- [x] 5.1 在 `domain/context.rs` 中移除 `PageContext` 的 `topic_dossier` / `family_dossier` / `research_result` / `compose_plan` / `research_session` 等字段，`PageContext` 简化为 compose 层的输入载体（从 KnowledgeUnit + UnitResearch 构建）
- [x] 5.2 在 `generation/planner.rs` 中移除 `PlannedPage` 的 `topic_kind` / `topic_key` / `family_kind` / `family_key` 等字段，改为从 `KnowledgeUnit.unit_type` 统一推导
- [x] 5.3 删除 `page_enrichment` 全部相关逻辑（`llm/mod.rs` 中的 `enrich_pages()`、`PageEnrichmentResult`），research 是唯一的 LLM 内容驱动器
- [x] 5.4 更新 Steering 配置解析（`domain/steering.rs`）：新增 `llm.max_research_calls` / `llm.max_compose_calls`（已在 4.9 中一并完成）

## 6. 自动化测试

- [x] 6.1 为 Knowledge Planning 层补齐 Rust 测试：`discover_knowledge_domains` 在 storybook fixture 上能发现 PluginEcosystem / MultiFramework / ApiReference / ConfigReference 等域，在 dagger fixture 上能发现 CoreRuntime / Framework / CompilerToolchain / ApiReference 等域
- [x] 6.2 为 Research 层补齐 Rust 测试：`ResearchProvider` 的 mock 实现验证三层调用顺序正确，DataSource 注入正确
- [x] 6.3 为 Compose 层补齐 Rust 测试：叶子优先处理顺序正确，parent page prompt 包含 child PageDigest，citation 密度达标
- [x] 6.4 为 init workflow 补齐端到端测试：四层 pipeline 完整跑通，输出 `.wiki/` 页面树与 `wiki.metadata.json`
- [x] 6.5 检查并修正本轮新增/修改代码中的注释，使其符合 [COMMENTING.md](E:/project/!byAI/spec-wiki/COMMENTING.md)
- [x] 6.6 为 Checkpoint & Resume 机制补齐测试：模拟 LLM 失败 → 检查点保存 → 恢复 → 跳过已完成步骤 → 正常完成

## 7. Storybook / Dagger 专项验证

- [x] 7.1 以 storybook 为样本运行新 pipeline init，验证知识域发现覆盖：至少发现 ConceptGuide / PluginEcosystem / MultiFramework / BuildSystem / TestingInfra / ConfigReference / ApiReference / ThemeSystem 等域
- [x] 7.2 以 dagger 为样本运行新 pipeline init，验证知识域发现覆盖：至少发现 CoreRuntime / Framework(Hilt) / PlatformBinding(Android) / CompilerToolchain / TestingInfra / ApiReference / ConceptGuide 等域
- [x] 7.3 对比 storybook 新生成页面树与 reference：折叠度（项目概述不再被 34 个 ref 页共享）、页面覆盖率（176 个 ref 页的命中率）、citation 密度（目标 >10/页）
- [x] 7.4 对比 dagger 新生成页面树与 reference：缺失页面数（目标从 31 降至 <10）、API/框架/测试/教程页面覆盖、citation 密度
- [x] 7.5 运行 storybook + dagger 的 lifecycle 验证（init → status → update → rebuild），验证知识树增量更新和 parent rebuild 传播正确
- [x] 7.6 更新 reference-project-reports，记录 2.0 pipeline 的页面覆盖、折叠度、citation 密度与仍存在的差距

## 8. 收尾

- [x] 8.1 更新 proposal.md 和 specs，反映从 family planner 到知识单元规划的架构升级
- [x] 8.2 运行 `.spec validate iteration-9-4-family-planner-and-research-first-composition`
- [x] 8.3 复核 proposal / specs / tasks 与实现一致
