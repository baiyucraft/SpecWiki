## 1. SQLite 存储层迁移

- [x] 1.1 在 `crates/wiki-core/Cargo.toml` 中新增 `rusqlite` 依赖（启用 `bundled` feature），确认 Windows 下编译通过。
- [x] 1.2 新增 `crates/wiki-core/src/storage/sqlite_store.rs`，实现 DB 初始化（创建 `.wiki/.cache/wiki-cache.db`）、schema 创建（`kv_store`、`page_context_cache`、`page_generation_cache` 三张表）和连接管理。
- [x] 1.3 在 `sqlite_store.rs` 中实现 `kv_store` 的通用读写接口（`kv_get` / `kv_set`），用于存储 WikiState、ScanReport、ModuleTree 等全局 JSON 数据。
- [x] 1.4 在 `sqlite_store.rs` 中实现 `page_context_cache` 和 `page_generation_cache` 表的 CRUD 接口（write / read / remove），替代原来的 per-page JSON 文件读写。
- [x] 1.5 修改 `crates/wiki-core/src/storage/state_store.rs`，把 `write_state` / `read_state` / `load_or_rebuild_state` 从 JSON 文件读写迁移到 SQLite 读写，保持外部函数签名不变。
- [x] 1.6 修改 `crates/wiki-core/src/storage/cache_store.rs`，把 `write_scan_cache` / `read_scan_cache` / `write_module_tree_cache` / `read_module_tree_cache` / `write_page_context_cache` / `read_page_context_cache` / `write_page_generation_cache` / `read_page_generation_cache` / `remove_page_caches` 从 JSON 文件读写迁移到 SQLite 读写。
- [x] 1.7 修改 `has_cache_layout()` 和 `missing_incremental_cache_components()` 的检查逻辑，从文件系统检查迁移到 SQLite 表和键的存在性检查。
- [x] 1.8 修改 `crates/wiki-core/src/storage/wiki_fs.rs` 中的 `remove_runtime()`，改为删除 `wiki-cache.db` 文件（而不是清理多个 JSON 文件和目录）。
- [x] 1.9 为 SQLite 存储层补充单元测试，覆盖 DB 初始化、kv 读写、page cache 读写、事务原子性、DB 损坏回退等场景。

## 2. Steering 配置基础设施

- [x] 2.1 在 `crates/wiki-core/src/` 中新增 steering 配置模块，定义 `SteeringConfig` 结构体（version、ignore.global、ignore.\<language\>、modules.promote/demote、pages.priority/hints、merge_threshold），实现 YAML 反序列化和默认值填充。`ignore` 字段为两层结构：`global` 列表和按语言键名的列表（如 `rust`、`python`、`javascript`、`java`、`go`）。
- [x] 2.2 实现 `.wiki/wiki.steering.yaml` 的读取入口：文件存在时解析并校验，文件不存在时返回默认配置，格式非法时输出 warning 并回退到默认值。
- [x] 2.3 为 steering 配置补充单元测试，覆盖正常解析、缺失文件、格式非法、部分字段缺失、全局忽略路径、按语言忽略路径、语言不匹配时忽略路径不生效等场景。

## 3. Page Identity 稳定性

- [x] 3.1 修改 `crates/wiki-core/src/generation/planner.rs` 中 module 页面的 `page_id` 生成规则，锚定到模块 `root_paths[0]`（归一化后的相对路径），而不是模块名或发现顺序。overview 和 architecture 页面继续使用固定种子。
- [x] 3.2 修改 module 页面的 `page_path`（`.wiki/` 下的文件路径）生成规则，锚定到模块 root_path 的归一化形式，确保不包含 Windows 非法路径字符。
- [x] 3.3 新增 page identity 稳定性单元测试：验证增删少量源文件后核心页面 page_id 不变、模块名变化但 root_path 不变时 page_id 稳定。

## 4. Planner 合并策略与父子关系

- [x] 4.1 在 hierarchy 或 planner 层为每个模块引入"页面权重"评分函数，基于源码文件数量、是否有子模块、是否是 workspace 成员、是否有入口文件等因素计算。
- [x] 4.2 在 `planner.rs` 中实现小模块合并逻辑：权重低于阈值（默认 3）且无子模块的模块不生成独立页面，其内容标记为父模块页面的子模块概述。steering 配置的 promote/demote 覆盖合并决策。
- [x] 4.3 升级 `planner.rs` 中的父子关系分配：顶层模块页以 overview 为父页面，嵌套模块页的父页面是模块树中父模块对应的页面。父模块被合并时向上查找最近的有独立页面的祖先模块。
- [x] 4.4 让 `planner.rs` 的 `plan_pages()` 接受 `SteeringConfig` 参数，消费忽略路径（传递给 scanner）、模块提升/降级和合并阈值。
- [x] 4.5 新增 planner 合并策略和父子关系的单元测试，覆盖小模块合并、有子模块不合并、steering promote/demote 覆盖、嵌套父子关系分配。

## 5. 页面类型扩展与 Section 模板丰富

- [x] 5.1 在 `planner.rs` 中引入 `workflow` 页面类型：当仓库存在 CI/CD 配置（`.github/workflows/`）、Makefile 或 Dockerfile 时生成 workflow 页面。
- [x] 5.2 扩展 `crates/wiki-core/src/generation/sections.rs` 中的 section 模板：overview 页面增加技术栈、入口与构建 section；architecture 页面增加架构提示 section 并把模块结构改为层级化文本表达；module 页面增加关键源码列表、依赖关系、子模块概述 section；新增 workflow 页面的 section 模板。
- [x] 5.3 调整 `crates/wiki-core/src/generation/context.rs` 中的页面上下文构建，为新增的 section 提供所需的结构化事实（技术栈列表、入口文件列表、模块树文本化、CI/CD 文件列表等）。
- [x] 5.4 确保 `section_titles_for_page_type()` 与新增的 section 标题保持同步，以支持 legacy 页面迁移和 managed section 识别。

## 6. Workflow 集成

- [x] 6.1 修改 `crates/wiki-core/src/workflows/init.rs`，在 pipeline 开始时读取 steering 配置，将额外忽略路径传递给 scanner，将 steering 配置传递给 planner。
- [x] 6.2 修改 `crates/wiki-core/src/workflows/update.rs`，在增量路径中读取 steering 配置并传递给 planner，确保 steering 配置变化能在下一次 update 中生效。
- [x] 6.3 修改 `crates/wiki-core/src/workflows/rebuild.rs`，在 rebuild 时读取 steering 配置并传递给 planner。
- [x] 6.4 修改 `crates/wiki-core/src/repo/scanner.rs`，让 `scan_repo()` 接受额外忽略路径参数（来自 steering 配置的 `ignore.global` + 当前语言匹配的 `ignore.<language>`），在扫描阶段追加排除。
- [x] 6.5 在 `scanner.rs` 中内置 per-language 默认忽略规则（Rust: `target/`；JS/TS: `node_modules/`、`dist/`、`.next/`、`.nuxt/`；Python: `__pycache__/`、`.venv/`、`*.egg-info/`；Java: `build/`、`.gradle/`；Go: `vendor/`），根据 scanner 检测到的仓库主语言自动激活，steering 用户配置追加到内置规则之上。

## 7. 自动化验证

- [x] 7.1 新增 SQLite 存储层集成测试，验证完整 init → update → sync → rebuild 生命周期中 DB 的创建、读写和清理行为正确。
- [x] 7.2 新增 steering 配置集成测试，验证全局忽略路径生效、按语言忽略路径在语言匹配时生效且不匹配时不生效、scanner 内置 per-language 默认忽略规则自动激活、模块提升/降级生效、合并阈值生效。
- [x] 7.3 新增页面拓扑集成测试，验证嵌套模块的父子关系按模块树层级分配、小模块被合并到父模块页面、合并后模块仍保留在 WikiState.modules 中。
- [x] 7.4 新增 section 模板集成测试，验证 overview 页面包含技术栈 section、architecture 页面包含层级化模块结构、workflow 页面在有 CI/CD 线索时生成。
- [x] 7.5 跑通 `cargo test` 与根级 `pnpm test`，修复本迭代引入的回归。

## 8. 测试项目集分析

- [x] 8.1 运行 `node scripts/run-test-projects.mjs` 对 `DESIGN.md § 测试项目集` 的完整项目集执行 `init`，重点关注页面拓扑稳定性、合并策略效果、父子关系正确性和 section 内容密度。
- [x] 8.2 对存在 reference 的测试项目（aLocal、axum、bat、chi、cobra、dagger、pinia、restaurant-app、storybook、zustand）执行结果对照，覆盖页面组织和 `wiki.metadata.json` 字段差异。
- [x] 8.3 对当前仓库 `E:\project\!byAI\spec-wiki` 执行 `init` 分析，重点记录页面合并效果和父子关系。
- [x] 8.4 运行 `node scripts/test-wiki-lifecycle.mjs` 验证全生命周期（init → status → sync → query → update → rebuild），确认 SQLite 存储、steering 配置和新 planner 不破坏已有 workflow 语义。
- [x] 8.5 把每个仓库的结构观察和差异分析写回本 change 的 `test-project-analysis.md`。

## 9. 注释合规检查

- [x] 9.1 按 `COMMENTING.md` 检查本轮新增或修改文件的注释，覆盖 SQLite 存储模块、steering 配置模块、planner 合并策略、section 模板扩展、workflow 集成和测试场景注释。
