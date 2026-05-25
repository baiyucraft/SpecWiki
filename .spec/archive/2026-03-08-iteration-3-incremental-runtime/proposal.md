## Why

当前 `WikiState` 和 `MetadataMapper` 已经落地，但 `status` 仍然只做路径级脏检测，`update` 仍然直接重跑 `init`，`.wiki/.cache/` 里的 scan 和 module tree 也还没有真正参与局部重建。这会让迭代 2 的状态内核停留在“可读状态”而不是“可驱动增量 runtime”的阶段，并且会直接阻塞迭代 4 的 managed section 演进。

## What Changes

- 引入增量 runtime 所需的 `ChangeSet / AffectedSet` 内核，把源码变化稳定映射到受影响模块、页面和页面 section。
- 扩展 `WikiState` 和 cache 布局，让 runtime 能持久化页面 section 状态、页面输入指纹和局部上下文 / 生成缓存，而不是只有整仓级 scan/module-tree/state 三份缓存。
- 重写 `status / update / rebuild` 的运行语义：
  - `status` 基于状态内核和缓存布局准确区分 `fresh / stale / missing / needs_rebuild`
  - `update` 在可局部修复时只重建受影响页面，并保持未受影响页面不重写
  - `rebuild` 继续作为显式全量重建入口
- 把页面重生成的内部粒度收敛到 section，先建立稳定 section ID、section hash 和组装接口，为迭代 4 的 managed section 替换预留边界，但本迭代不实现人工内容保留。
- 补齐迭代 3 的验证面：单文件修改、新增/删除源码、cache 丢失回退、结构变更触发全量重建，以及对测试项目集执行 `init` 的对比分析要求。

## Capabilities

### New Capabilities
- `wiki-change-set-kernel`: 定义源码变化到模块/页面/section 影响面的计算、cache 失效规则和增量重建入口

### Modified Capabilities
- `repo-wiki-runtime`: 运行时 cache 从“可落盘”提升到“可复用”，并补齐 page context / generation 级缓存与 section 级页面状态
- `repo-wiki-workflow`: `status / update / rebuild` 的 requirement 从全量刷新语义升级为增量 runtime 语义
- `wiki-state-kernel`: WikiState requirement 扩展为承载 change propagation、section 状态和页面输入指纹
- `workflow-verification`: 验证 requirement 扩展为覆盖增量更新、结构变化和 cache 回退，而不只是“update 能跑通”

## Impact

- `crates/wiki-core/src/domain/`：`state.rs`、`change_set.rs` 需要扩展为真正的增量状态模型
- `crates/wiki-core/src/storage/`：需要新增或扩展 page context / generation cache 读写与 cache invalidation
- `crates/wiki-core/src/generation/`：`context.rs`、`sections.rs`、`renderer.rs` 需要引入 section-level 产物和装配接口
- `crates/wiki-core/src/workflows/`：`status.rs`、`update.rs`、`rebuild.rs` 需要按 change set 驱动；`init.rs` 需要补齐增量 runtime 的初始化状态
- `crates/wiki-core/tests/` 与 `scripts/tests/`：需要新增增量更新、结构变更和 cache 回退测试；同时按 AGENTS 要求补做测试项目集 `init` 分析
- Agent 层 JSON IPC 不预期发生 breaking change，但 `update` 返回的 `updated_pages` 将收敛为“本次实际触达的页面集合”，不再等价于全量重建输出
