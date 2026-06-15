## 1. Managed Section Kernel

- [x] 1.1 在 `crates/wiki-core/src/` 中引入 managed section parse / merge 内核，定义 marker 协议、managed block、user block、page parse mode 和 merge plan。
- [x] 1.2 为 marker 模式与 legacy heading 模式实现统一页面解析入口，覆盖 managed marker、用户新增区段、managed drift 和页面顺序恢复。
- [x] 1.3 把页面组装入口改为”managed section 生成 + user section 回插”的统一接口，避免 workflow 自己手写字符串拼装。

## 2. State And Cache Model

- [x] 2.1 扩展 `crates/wiki-core/src/domain/state.rs`，让 `WikiSectionState` 承载 `managed`、observed `content_hash`、`generated_content_hash` 和 user section 锚点。
- [x] 2.2 扩展 `crates/wiki-core/src/storage/` 的 state / cache 读写，支持 editable runtime 所需的新 section 字段、格式版本和 legacy runtime 识别。
- [x] 2.3 校准 `MetadataMapper` 与相关导出路径，确保 sync 后的 summary、page content hash 与 metadata 一致，同时不泄漏内部 editable 字段。

## 3. Sync And Migration Workflow

- [x] 3.1 重写 `crates/wiki-core/src/workflows/sync.rs`，把整页 hash 回写升级为 parser-first sync，回写 page/section 状态、summary 和 metadata。
- [x] 3.2 实现 managed drift 检测与 legacy heading migration，确保迭代 3 页面在常见手工插入区段场景下可平滑进入 marker 格式。
- [x] 3.3 为 sync 补充 warning / 日志边界，明确 managed block 被手工改写、legacy 页面无法迁移等场景的反馈方式。

## 4. Editable Runtime Assembly

- [x] 4.1 调整 `init` 的页面写盘格式，确保首次生成就写出 managed section marker，而不是 plain Markdown section。
- [x] 4.2 重写 `update` 的局部重建路径，在页面 identity 稳定时只替换 managed sections，并保留已同步的 user sections。
- [x] 4.3 校准 `rebuild`，让它忽略旧 generation cache 的同时仍能复用同页 user sections，而不是把人工内容整页抹掉。

## 5. Automated Verification

- [x] 5.1 新增 parse / merge 单元测试，覆盖 marker 解析、legacy heading 迁移、user section 锚点恢复和 managed drift 检测。
- [x] 5.2 新增 workflow / integration 测试，验证”手工插入 section -> sync -> source change -> update”后 user content 仍保留，以及 rebuild 不会误删同页 user sections。
- [x] 5.3 新增 legacy 页面回归测试，覆盖无 marker 页面迁移、标题完整时成功迁移、标题损坏时 warning 边界。
- [x] 5.4 跑通 `cargo test` 与根级 `pnpm test`，修复 editable runtime 引入的回归。

## 6. Test Project Set Analysis

- [x] 6.1 对 `E:\\project\\aLocal` 执行 `init` 并分析当前 `.wiki` 结构与 `reference-aLocal` 的差异，确认 editable runtime 不与现有页面结构假设冲突。
- [x] 6.2 对当前仓库 `E:\\project\\!byAI\\spec-wiki` 执行 `init` / `sync` 分析，重点记录页面结构与 editable runtime 的兼容性，以及 legacy 页面迁移边界。
- [x] 6.3 对 `.wiki/06-设计文档/00-总体设计.md § 测试项目集` 中其余所有样本仓库执行 `init`，并把每个仓库的结构观察写回本 change 的分析材料。
- [x] 6.4 对存在 reference 的测试项目执行结果对照，覆盖 `reference-aLocal`、`reference-axum`、`reference-bat`、`reference-chi`、`reference-cobra`、`reference-dagger`、`reference-pinia`、`reference-restaurant-app`、`reference-storybook` 和 `reference-zustand`，明确页面组织和 `wiki.metadata.json` 字段差异。

## 7. Commenting Compliance

- [x] 7.1 按 `.wiki/02-开发指南/00-代码注释规范.md` 检查本轮新增或修改文件的注释，覆盖 parse / merge 内核、扩展后的状态模型、workflow 关键路径和测试场景注释。
