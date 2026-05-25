## Why

迭代 5 已经把 editable wiki runtime 落地（managed section marker、parser-first sync、user section 保留），但页面拓扑层仍然是迭代 1 的原始状态：`page_id` / `page_path` 的生成规则没有经过稳定性校准，planner 的页面合并策略、父子关系分配和标题规则都是最初版本，用户也没有任何手段影响页面规划结果。

这带来三个实际问题：

1. **页面拓扑不稳定**：同一仓库重复 `init` 或局部 `update` 后，核心页面的 path / page_id / parent 关系可能因为模块树微调而发生不必要的变化。迭代 5 引入的 user section 保留机制依赖 page_id 稳定——如果 page_id 频繁漂移，用户沉淀的手工内容会变成 orphan。
2. **页面规划质量不足**：当前 planner 对所有模块一视同仁地生成独立页面，没有合并小模块、拆分大模块或按重要性排序的能力。参考实现 `deepwiki-rs` 的 compose 层按 overview / architecture / workflow / key-modules / boundary / database 六类组织页面，`CodeWiki` 按模块复杂度区分 complex / leaf 两种生成策略。当前 planner 远不及这个水平。
3. **缺少用户引导面**：`deepwiki` 类产品已经证明纯启发式规则无法覆盖所有仓库结构，需要 steering 文件让用户声明忽略路径、入口提示、模块提升/降级和页面优先级。当前 wiki-core 没有任何 steering 配置入口。

迭代 4 已修复 scanner / hierarchy 事实层硬伤，迭代 5 已落地 editable runtime。现在是稳定页面拓扑和引入 steering 的最佳时机：事实层干净、editable runtime 已就绪，但页面规划层还没有经过质量收口。如果直接进入迭代 7（LLM 增强），LLM 生成的高质量内容会绑定到一批拓扑不稳定的页面上，后续修正 planner 时迁移成本更高。

## What Changes

- 稳定 `page_id` / `page_path` 生成规则：确保同一仓库在模块树微调（如新增/删除少量源文件）后，核心页面的 ID 和路径保持不变。引入 page identity 稳定性测试，覆盖"增删单文件后 page_id 不变"的不变量。
- 升级 planner 的页面合并与拆分策略：参考 `deepwiki-rs` 的六类页面组织和 `CodeWiki` 的 complex / leaf 模块区分，引入小模块合并（多个低信号模块合并到父模块页）、大模块拆分提示（为后续 LLM 增强预留子页面拆分点）和页面优先级排序。
- 丰富页面类型和 section 模板：当前只有 `overview / architecture / module` 三种页面类型，每种只有 3 个固定 section。参考 `deepwiki-rs` 的 workflow / boundary / database 页面类型和 `CodeWiki` 的递归子模块文档，扩展页面类型集合和 section 模板，让不同类型的页面有差异化的内容组织。
- 引入 repo 级 steering 配置（`.wiki/wiki.steering.yaml`）：让用户可以声明忽略路径、入口提示、模块提升/降级、页面优先级和自定义页面提示，而不是被迫改 core 代码。steering 配置在 `init / update / rebuild` 时被读取并影响 planner 决策。
- 稳定父子关系分配规则：当前所有模块页都挂在概述页下面，没有利用模块树的层级关系。升级为按模块树层级自动分配父子关系，让 Wiki 页面树与仓库模块树对齐。
- 校准 overview 和 architecture 页面的内容密度：当前这两个页面的 section 内容过于骨架化（只有 bullet list），参考 `deepwiki-rs` 的 overview_editor / architecture_editor 的输出结构，在 deterministic 层面提升信息密度。
- **把缓存和状态存储从散落的 JSON 文件迁移到 SQLite**：当前 `.wiki/.cache/` 下有 `wiki-state.json`、`repo-scan.json`、`module-tree.json` 以及 `page-contexts/`、`page-generation/` 两个目录下的大量 per-page JSON 文件。大型仓库（如 storybook 5000+ 文件）会产生数百个 JSON 文件，读写性能差、原子性无保障、文件锁冲突风险高。迁移到单个 `.wiki/.cache/wiki-cache.db` SQLite 数据库，统一所有缓存和状态的读写路径，为后续 LLM cache、query index 等新增缓存需求提供可扩展的存储基础。

## Capabilities

### New Capabilities
- `wiki-steering-config`: 定义 `.wiki/wiki.steering.yaml` 的 schema、读取、校验和默认值，以及 steering 配置如何影响 planner 决策（忽略路径、入口提示、模块提升/降级、页面优先级、自定义页面提示）。忽略路径支持全局和按语言两层配置，scanner 同时内置 per-language 默认忽略规则。
- `page-topology-stability`: 定义 page_id / page_path 的稳定性不变量、identity 锚定规则和微调容忍策略，确保模块树局部变化不会导致核心页面 ID 漂移。
- `sqlite-cache-storage`: 定义 `.wiki/.cache/wiki-cache.db` 的 SQLite schema、表结构、读写接口和迁移策略，统一替代当前散落的 JSON 文件缓存（wiki-state、repo-scan、module-tree、page-context、page-generation）。

### Modified Capabilities
- `repo-wiki-workflow`: `init / update / rebuild` 需要读取 steering 配置并传递给 planner；`update` 的增量路径需要感知 page identity 稳定性规则。
- `repo-hierarchy-model`: planner 的页面合并/拆分策略、父子关系分配和优先级排序需要升级；页面类型集合和 section 模板需要扩展。
- `repo-wiki-runtime`: 页面文件的命名规则和目录结构可能因为父子关系升级而变化；steering 配置文件成为 runtime 的一部分。
- `workflow-verification`: 验证 requirement 需要覆盖 page identity 稳定性、steering 配置生效、页面合并/拆分和父子关系正确性。

## Impact

- `crates/wiki-core/src/generation/planner.rs`：页面规划逻辑重写，引入合并/拆分策略、优先级排序、steering 配置消费和层级化父子关系分配。
- `crates/wiki-core/src/generation/sections.rs`：扩展页面类型集合和 section 模板，丰富 overview / architecture 页面的 deterministic 内容。
- `crates/wiki-core/src/generation/context.rs`：页面上下文构建可能需要适配新的页面类型和合并后的模块范围。
- `crates/wiki-core/src/domain/stable_id.rs`：page_id 生成规则可能需要引入锚定策略，确保微调稳定性。
- `crates/wiki-core/src/workflows/`：`init.rs`、`update.rs`、`rebuild.rs` 需要读取 steering 配置并传递给 planner。
- `crates/wiki-core/src/storage/`：整体重构——`state_store.rs` 和 `cache_store.rs` 从 JSON 文件读写迁移到 SQLite 读写；新增 `sqlite_store.rs` 承载 DB 初始化、schema 迁移和统一读写接口；`metadata_store.rs` 保持不变（`wiki.metadata.json` 仍是正式索引层，不入 DB）。新增 steering 配置的读取路径。
- `crates/wiki-core/Cargo.toml`：新增 `rusqlite` 依赖（bundled feature，不需要系统级 SQLite 安装）。
- `crates/wiki-core/tests/`：新增 page identity 稳定性测试、steering 配置测试、页面合并/拆分测试、SQLite 存储层读写测试。
- 不涉及 Agent 层 JSON IPC 协议变更；metadata 导出字段保持兼容，但页面数量和层级关系会因为合并/拆分而变化。
- `.wiki/.cache/` 目录结构变化：从多个 JSON 文件和子目录收敛为单个 `wiki-cache.db` 文件（加上 `wiki.metadata.json` 仍保留在 `.wiki/` 根目录）。旧 JSON 缓存在首次 `init` 或 `rebuild` 后自动清理。
