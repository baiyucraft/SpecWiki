## Context

当前仓库已经完成到迭代 5 的运行时主链：

- `Repository Scan -> ModuleTree -> Page Planner -> Wiki Assembler` 主路径稳定。
- `WikiState`、`ChangePlan`、page context / generation cache、section 粒度渲染、managed section marker、parser-first sync、user section 保留都已落地。
- 迭代 4 已修复 scanner 噪声过滤（fixture 排除、嵌套仓库排除、非代码产物目录排除）、单文件模块抑制、module kind 多维分类和关键源码选择信号。
- 迭代 5 已引入 managed section kernel、editable wiki runtime、legacy 页面迁移和 parser-first sync。

但页面规划层仍然是迭代 1 的原始状态：

- `planner.rs` 对所有模块一视同仁地生成独立页面，没有合并小模块或按重要性排序的能力。
- 所有模块页都直接挂在概述页下面（`parent_id = overview_id`），没有利用模块树的层级关系。
- 只有 `overview / architecture / module` 三种页面类型，每种只有 3 个固定 section，内容骨架化。
- `page_id` 由 `stable_id("page", ...)` 生成，但没有经过稳定性校准——模块树微调可能导致核心页面 ID 漂移。
- 用户没有任何手段影响页面规划结果。

设计约束来自四个方向：

- 项目边界：继续遵守 `Repository Scan -> ModuleTree -> Page Planner -> Wiki Assembler` 主路径，不能绕过 planner 直接拼页面。
- 参考实现：
  - `deepwiki-rs` 的 compose 层按 overview / architecture / workflow / key-modules / boundary / database 六类 editor 组织页面，每类 editor 有独立的 prompt 和输出结构。这说明页面类型应该更丰富，且不同类型的页面应该有差异化的 section 模板。
  - `CodeWiki` 按模块复杂度区分 complex / leaf 两种生成策略，complex 模块递归拆分子模块文档。这说明 planner 应该根据模块规模和复杂度做差异化处理。
  - `deepwiki-open` 的 `data_pipeline.py` 和 `websocket_wiki.py` 展示了 wiki 结构的消费层组织方式，但不直接影响本迭代的 planner 设计。
- `.wiki/06-设计文档/00-总体设计.md` 边界：本迭代只做页面拓扑稳定和 steering 配置，不进入 LLM 增强、Diagram 或消费层。
- 仓库现状：迭代 4 已修复 scanner / hierarchy 事实层硬伤，迭代 5 已落地 editable runtime。当前基线干净，可以专注于 planner 质量。

## Goals / Non-Goals

**Goals:**

- 稳定 `page_id` / `page_path` 生成规则，确保同一仓库在模块树微调（增删少量源文件）后核心页面的 ID 和路径保持不变。
- 升级 planner 的页面合并策略：低信号小模块合并到父模块页，避免生成大量只有几行内容的独立页面。
- 升级父子关系分配：按模块树层级自动分配页面父子关系，让 Wiki 页面树与仓库模块树对齐，而不是所有模块页都平铺在概述页下面。
- 扩展页面类型和 section 模板：参考 `deepwiki-rs` 的六类 editor，在 deterministic 层面丰富页面内容组织。
- 引入 repo 级 steering 配置（`.wiki/wiki.steering.yaml`），让用户可以声明忽略路径、模块提升/降级、页面优先级和自定义提示。
- 提升 overview 和 architecture 页面的 deterministic 内容密度。
- 补齐 page identity 稳定性测试、steering 配置测试和测试项目集分析。
- 在 tasks 设计和测试阶段都对 `.wiki/06-设计文档/00-总体设计.md § 测试项目集` 全量执行 `init` 分析，并在存在 reference 时做结构与 metadata 对照。

**Non-Goals:**

- 不引入 LLM、RAG、TOON 或 explanation layer 能力。本迭代的内容丰富仍然是 deterministic 的。
- 不做 Diagram Generator。
- 不改变 Agent 层 JSON IPC 协议或动作集合。
- 不做跨页面的 user content 自动迁移（page_id 变化时 user content 不承诺搬运）。
- 不做前端可视化编辑或宿主 UI 协议设计。
- 不在本迭代做 query 增强或 TOON 格式输出。
- SQLite 迁移不改变 `wiki.metadata.json` 的存在形式——它仍然是正式索引层的 JSON 文件，不入 DB。

## Decisions

### 决策 1：page_id 锚定到模块 root_path 而不是模块发现顺序

当前 `page_id` 由 `stable_id("page", &page.relative_path)` 生成，而 `relative_path` 又依赖模块名和模块发现顺序。如果模块树微调导致某个模块的名字或路径变化，page_id 就会漂移。

新策略：

- overview 和 architecture 页面的 page_id 继续使用固定种子（`stable_id("page", "overview")`、`stable_id("page", "architecture")`），保持绝对稳定。
- module 页面的 page_id 锚定到模块的 `root_paths[0]`（归一化后的相对路径），而不是模块名或发现顺序。这样即使模块名因为 kind 分类调整而变化，只要模块的根路径不变，page_id 就不变。
- page_path（`.wiki/` 下的文件路径）同样锚定到模块 root_path 的归一化形式，而不是模块名。

备选方案：

- 方案 A：继续用模块名生成 page_id
  - 否决原因：模块名可能因为 kind 分类或 steering 配置而变化，导致 page_id 不稳定。
- 方案 B：用模块 ID 生成 page_id
  - 否决原因：模块 ID 本身也是从 root_path 生成的，直接用 root_path 更直观。

### 决策 2：引入小模块合并策略，而不是为每个模块都生成独立页面

当前 planner 为模块树中的每个非根模块都生成一个独立页面。对于大型仓库，这会产生大量只有几行内容的小页面（如只包含一两个配置文件的辅助模块）。

新策略：

- 引入模块"页面权重"评分，基于：源码文件数量、是否有子模块、是否是 workspace 成员、是否有入口文件、模块 kind 是否为核心类型。
- 权重低于阈值的模块不生成独立页面，其内容合并到父模块页面中（作为父模块页的一个额外 section）。
- 阈值可通过 steering 配置调整。
- 合并后的模块仍然保留在 `WikiState.modules` 中，只是不再有独立的 `WikiPageState`。

参考实现对照：

- `CodeWiki` 的 `is_complex_module()` 函数根据组件数量和核心组件比例区分 complex / leaf 模块，complex 模块递归生成子文档，leaf 模块只生成简单文档。
- `deepwiki-rs` 的 `key_modules_insight_editor` 只为"关键模块"生成深度文档，其余模块在 architecture 页面中概述。

备选方案：

- 方案 A：继续为每个模块生成独立页面
  - 否决原因：大型仓库会产生过多低价值页面，稀释 Wiki 的信息密度。
- 方案 B：只为顶层模块生成页面，子模块全部合并
  - 否决原因：过于激进，会丢失重要子模块的独立可见性。

### 决策 3：按模块树层级分配页面父子关系，而不是全部平铺

当前所有模块页都以 overview 页为父页面。这导致 Wiki 页面树是扁平的，无法反映仓库的层级结构。

新策略：

- overview 页是根页面（无父页面）。
- architecture 页是 overview 的子页面。
- 顶层模块页是 overview 的子页面。
- 嵌套模块页的父页面是其在模块树中的父模块对应的页面。
- 如果父模块因为合并策略没有独立页面，则向上查找最近的有独立页面的祖先模块。

这样 Wiki 页面树与模块树的层级关系自然对齐。

备选方案：

- 方案 A：继续全部平铺在 overview 下
  - 否决原因：无法反映仓库层级结构，大型仓库的页面列表会很长。
- 方案 B：引入中间"分组页面"（如"核心模块"、"工具模块"）
  - 否决原因：分组规则不确定，容易引入不稳定的页面拓扑。留给后续 steering 配置或 LLM 增强。

### 决策 4：扩展页面类型为 overview / architecture / module / workflow，而不是引入全部六类

`deepwiki-rs` 有六类 editor（overview / architecture / workflow / key-modules / boundary / database）。但当前 deterministic 层面能稳定产出的只有前四类：

- `overview`：项目概述，包含项目事实、技术栈、入口点。
- `architecture`：系统架构，包含模块结构、跨模块关系、架构提示。
- `module`：模块页面，包含模块说明、模块事实、关键源码、依赖关系。
- `workflow`：新增。当仓库存在明确的工作流线索（如 CI/CD 配置、Makefile、Docker 编排）时，生成工作流页面。

不在本迭代引入的页面类型：

- `boundary`：需要 LLM 辅助判断 API 边界和外部集成点，留给迭代 7。
- `database`：需要 LLM 辅助解析 SQL schema 和 ORM 模型，留给迭代 7。

备选方案：

- 方案 A：只保留现有三种页面类型
  - 否决原因：workflow 页面可以从 deterministic 事实（CI 配置、Makefile、Docker 文件）稳定生成，不需要 LLM。
- 方案 B：引入全部六种页面类型
  - 否决原因：boundary 和 database 需要 LLM 辅助，超出本迭代范围。

### 决策 5：section 模板按页面类型差异化扩展

当前每种页面类型只有 3 个固定 section，内容骨架化。扩展为：

**overview 页面 sections：**
- 简介（项目一句话描述 + 主要技术栈）
- 项目事实（文件统计、语言分布、workspace 结构）
- 技术栈（manifest 解析出的依赖和框架）
- 入口与构建（入口文件、构建命令、运行方式）
- 关键信息（高信号 summary inputs）

**architecture 页面 sections：**
- 架构概览（模块层级概述）
- 模块结构（模块树的文本化表达，含层级缩进）
- 跨模块关系（关系边的结构化列表）
- 架构提示（从模块树推断的架构模式）

**module 页面 sections：**
- 模块说明（模块角色、kind、根路径）
- 关键源码（入口文件、核心实现文件列表）
- 依赖关系（依赖模块和被依赖模块）
- 模块事实（文件统计、技术标签）
- 子模块概述（如果有被合并的子模块，在此列出）

**workflow 页面 sections：**
- 工作流概述（CI/CD、构建、部署的整体描述）
- 构建流程（Makefile / package.json scripts / Cargo 命令）
- CI/CD 配置（GitHub Actions / GitLab CI 等）
- 容器化（Dockerfile / docker-compose）

### 决策 6：steering 配置使用 `.wiki/wiki.steering.yaml`，而不是仓库根目录的独立文件

steering 配置放在 `.wiki/` 目录下，与 Wiki runtime 产物在同一层级。这样：

- 用户可以选择是否把 steering 配置提交到 Git（与 `.wiki/*.md` 一样）。
- 不污染仓库根目录。
- steering 配置的生命周期与 Wiki runtime 一致。

配置 schema：

```yaml
# .wiki/wiki.steering.yaml
version: 1

# 额外忽略路径（追加到 scanner 内置规则之上）
ignore:
  # 全局忽略（所有语言生效）
  global:
    - "docs/**"
    - "examples/**"
    - "benchmarks/**"
  # 按语言忽略（仅对该语言的仓库生效，语言标识与 scanner 检测到的主语言一致）
  rust:
    - "target/**"
    - "benches/**"
  python:
    - ".venv/**"
    - "__pycache__/**"
  javascript:
    - "dist/**"
    - "coverage/**"
  java:
    - "build/**"
    - ".gradle/**"

# 模块提升/降级
modules:
  promote:
    - path: "internal/core"
      reason: "核心业务逻辑"
  demote:
    - path: "scripts"
      reason: "辅助脚本，不需要独立页面"

# 页面优先级调整
pages:
  priority:
    - path: "src/domain"
      boost: 2
  # 自定义页面提示（供后续 LLM 增强使用）
  hints:
    - page_type: "overview"
      hint: "这是一个微服务架构项目"

# 小模块合并阈值（默认 3）
merge_threshold: 3
```

备选方案：

- 方案 A：放在仓库根目录（如 `.wikirc.yaml`）
  - 否决原因：污染仓库根目录，且与 Wiki runtime 产物分离。
- 方案 B：放在 `wiki.metadata.json` 中
  - 否决原因：metadata 是导出格式，不应承载用户输入配置。

### 决策 7：steering 配置缺失时使用合理默认值，不阻塞 pipeline

steering 配置是可选的。当 `.wiki/wiki.steering.yaml` 不存在时：

- 所有 steering 字段使用默认值（空忽略列表、无提升/降级、默认合并阈值）。
- pipeline 正常运行，行为与迭代 5 一致。
- 不输出 warning 或提示用户创建 steering 文件。

这确保了向后兼容：现有仓库不需要任何额外操作就能继续使用。

### 决策 8：把缓存和状态存储从散落的 JSON 文件迁移到单个 SQLite 数据库

当前 `.wiki/.cache/` 下的存储布局：

```text
.wiki/.cache/
├─ wiki-state.json          # WikiState 主模型
├─ repo-scan.json           # 扫描缓存
├─ module-tree.json         # 模块树缓存
├─ page-contexts/           # 每页上下文缓存（N 个 JSON 文件）
│   ├─ <page-id-1>.json
│   └─ <page-id-2>.json
└─ page-generation/         # 每页生成缓存（N 个 JSON 文件）
    ├─ <page-id-1>.json
    └─ <page-id-2>.json
```

问题：

- 大型仓库（如 storybook 5000+ 文件）会产生数十个 per-page JSON 文件，文件系统 I/O 开销大。
- 多个 JSON 文件之间没有原子性保障——写入 wiki-state.json 成功但 page-generation 写入失败时，runtime 处于不一致状态。
- 后续迭代 7 引入 LLM cache、explanation cache 等新缓存类型时，继续用 JSON 文件会让 `.cache/` 目录更加膨胀。
- `deepwiki-rs` 使用 `CacheManager` 统一管理缓存，`deepwiki-open` 使用 `data_pipeline` 统一数据流——都指向"统一存储入口"的方向。

新策略：

- 引入单个 SQLite 数据库文件 `.wiki/.cache/wiki-cache.db`，统一承载所有缓存和状态数据。
- 使用 `rusqlite` crate（bundled feature），不需要系统级 SQLite 安装，Windows 兼容性好。
- 初始化时启用 WAL journal mode（`PRAGMA journal_mode=WAL`），提升并发读性能，为后续 MCP server 或多进程消费预留能力。
- 表结构设计：

```sql
-- 键值存储表，承载 wiki-state、repo-scan、module-tree 等全局数据
CREATE TABLE kv_store (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

-- 每页上下文缓存
CREATE TABLE page_context_cache (
    page_id    TEXT PRIMARY KEY,
    input_hash TEXT NOT NULL,
    context    TEXT NOT NULL  -- JSON 序列化的 PageContext
);

-- 每页生成缓存
CREATE TABLE page_generation_cache (
    page_id      TEXT PRIMARY KEY,
    input_hash   TEXT NOT NULL,
    content_hash TEXT NOT NULL,
    sections     TEXT NOT NULL  -- JSON 序列化的 Vec<SectionDraft>
);
```

- `kv_store` 表用于存储全局数据：`wiki-state`（WikiState JSON）、`repo-scan`（ScanReport JSON）、`module-tree`（ModuleTree JSON）。
- `page_context_cache` 和 `page_generation_cache` 表替代原来的 per-page JSON 文件目录。
- 所有写入操作使用 SQLite 事务，保证原子性。
- `wiki.metadata.json` 不入 DB，仍然作为正式索引层的 JSON 文件保留在 `.wiki/` 根目录。
- `.wiki/*.md` 页面文件不入 DB，仍然作为正式 Wiki 文档保留。

迁移策略：

- 新增 `sqlite_store.rs` 模块，提供 DB 初始化、schema 创建和统一读写接口。
- 修改 `state_store.rs` 和 `cache_store.rs`，把所有 JSON 文件读写替换为 SQLite 读写。
- `has_cache_layout()` 改为检查 `wiki-cache.db` 是否存在且 schema 完整。
- `load_or_rebuild_state()` 的回退路径保持不变：优先从 DB 读 WikiState，不存在时从 `wiki.metadata.json` 重建。
- 首次 `init` 或 `rebuild` 后，旧的 JSON 缓存文件自动清理。
- `remove_runtime()` 改为删除 `wiki-cache.db` 文件。

备选方案：

- 方案 A：继续使用 JSON 文件，只做目录结构优化
  - 否决原因：无法解决原子性问题，后续新增缓存类型时仍会膨胀。
- 方案 B：使用 sled 或 redb 等嵌入式 KV 存储
  - 否决原因：SQLite 生态更成熟、调试更方便（可以用 sqlite3 CLI 直接查看）、`rusqlite` bundled 模式跨平台兼容性好。
- 方案 C：只抽象存储接口（trait），不实际引入 SQLite
  - 否决原因：抽象层增加复杂度但不解决实际问题，且后续仍需要做迁移。

## Risks / Trade-offs

- [page_id 锚定规则变化可能导致现有 user content 变成 orphan] → 本迭代的 page_id 变化是一次性的迁移。在 `update` 中，如果检测到旧 page_id 不存在但新 page_id 对应同一 root_path，尝试从旧页面恢复 user sections。迁移后 page_id 将保持稳定。
- [小模块合并可能导致某些用户期望看到独立页面的模块被合并] → 通过 steering 配置的 `promote` 字段让用户可以强制保留独立页面。默认合并阈值设为保守值（3 个源文件以下才合并）。
- [workflow 页面类型的 deterministic 内容可能信息密度不高] → 本迭代只在存在明确工作流线索时才生成 workflow 页面，不强制生成。内容密度的实质性提升留给迭代 7 的 LLM 增强。
- [steering 配置的 schema 可能在后续迭代需要扩展] → 使用 `version: 1` 字段，后续扩展时可以做 schema 迁移。当前只实现最小必要字段。
- [父子关系按模块树层级分配后，页面树可能很深] → 对于超过 3 层的嵌套，考虑在 metadata 导出时提供扁平化视图。但页面文件本身仍然按层级组织。
- [SQLite 引入新的编译依赖] → 使用 `rusqlite` 的 `bundled` feature，自带 SQLite 源码编译，不需要系统级安装。Windows 上需要 C 编译器（MSVC 已满足）。二进制体积增加约 1-2MB，可接受。
- [SQLite 迁移与 page topology 同时推进增加 scope] → SQLite 迁移是存储层的独立变更，与 planner 逻辑解耦。先做 SQLite 迁移（替换读写接口），再做 planner 升级，两者不交叉。
- [旧 JSON 缓存与新 SQLite 缓存的过渡期] → 首次 `init` 或 `rebuild` 会创建新 DB 并清理旧 JSON 文件。`load_or_rebuild_state()` 的回退路径保持不变（从 `wiki.metadata.json` 重建），所以即使 DB 不存在也能恢复。

## Migration Plan

1. 先引入 `rusqlite` 依赖和 `sqlite_store.rs` 模块，实现 DB 初始化、schema 创建和基础读写接口。
2. 把 `state_store.rs` 和 `cache_store.rs` 的 JSON 文件读写逐步替换为 SQLite 读写，保持外部接口签名不变。
3. 升级 `stable_id` 和 planner 的 page_id / page_path 生成规则，确保新规则稳定。
4. 引入 steering 配置的读取和 schema 校验。
5. 升级 planner 的合并策略、父子关系分配和页面类型扩展。
6. 扩展 section 模板，丰富各类页面的 deterministic 内容。
7. 校准 workflow 文件（init / update / rebuild）以消费 steering 配置和新 planner 输出。
8. 补齐测试与测试项目集分析。

回滚策略：

- SQLite 迁移：删除 `.wiki/.cache/wiki-cache.db` 后执行 `rebuild` 即可重建。`wiki.metadata.json` 始终存在，`load_or_rebuild_state()` 可以从中恢复。
- steering 配置是可选的，删除 `.wiki/wiki.steering.yaml` 即可回退到默认行为。
- page_id 变化是一次性迁移，如果需要回滚，执行 `rebuild` 即可重建所有页面。
- 小模块合并策略可以通过 steering 配置的 `merge_threshold: 0` 禁用。

## Open Questions

- workflow 页面是否应该在本迭代就引入，还是留给迭代 7 与 LLM 增强一起做？当前倾向于本迭代引入 deterministic 骨架，迭代 7 再用 LLM 丰富内容。
- 小模块合并的默认阈值应该是多少？需要在测试项目集上实验后确定。当前暂定 3（源文件数 ≤ 3 且无子模块的模块被合并）。
- 是否需要为合并后的模块在父模块页面中生成专门的"子模块概述" section？当前设计包含此 section，但如果实现复杂度过高可以简化为只在模块事实中列出。
- SQLite 的 WAL 模式是否需要在本迭代启用？WAL 可以提升并发读性能，但当前 wiki-core 是单进程同步调用，暂时不需要。后续如果引入 MCP server 或多进程消费，再启用。
