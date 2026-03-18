
# 必须遵守

1. 用中文沟通；
2. 目前处于测试开发阶段，不需要考虑旧版本的兼容性，兼容性代码可以全部删除；
3. 在生成具体改动方案时，要召唤一个言辞犀利、标准很高的资深工程师 reviewer（gpt-5.4 xhigh subagent）。参考相关项目与现有代码，对方案进行全面评估，只有方案被审核通过了，才可以被实施；
4. 在沟通过程中，可以肆无忌惮的召唤 subagent 来协助你、替你头脑风暴、分析最佳解决方案。记住你不只是执行者，还是项目的管理主导者；

# 项目目标

目标是构建一个 Repo Wiki Core + Agents 体系：自动扫描代码仓库，生成并持续更新 `.wiki/`，让人和 Agent 都能共享同一层项目知识。

设计与边界优先看 [DESIGN2.0.md](E:/project/!byAI/spec-wiki/DESIGN2.0.md) 和 [DESIGN-CORE2.0.md](E:/project/!byAI/spec-wiki/DESIGN-CORE2.0.md)。
代码注释规范单独见 [COMMENTING.md](E:/project/!byAI/spec-wiki/COMMENTING.md)。

# 开发参考

开发过程中需要优先参考以下本地仓库路径，而不是只看远程项目名：

- `deepwiki-rs`本地路径：[tmp/upstream/deepwiki-rs](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs)
- `CodeWiki`本地路径：[tmp/upstream/codewiki/codewiki](E:/project/!byAI/spec-wiki/tmp/upstream/codewiki/codewiki)
- `deepwiki-open`[tmp/upstream/deepwiki-open](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-open)
- `GitNexus`本地路径：[tmp/upstream/GitNexus](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus)

引用这些参考实现时，以当前仓库的 [DESIGN2.0.md](E:/project/!byAI/spec-wiki/DESIGN2.0.md) 和 [DESIGN-CORE2.0.md](E:/project/!byAI/spec-wiki/DESIGN-CORE2.0.md) 为最终边界，不直接照搬其产品形态或目录结构。

# 当前核心设计约束

- 当前 `wiki-core` 主路径以 [DESIGN-CORE2.0.md](E:/project/!byAI/spec-wiki/DESIGN-CORE2.0.md) 为准：
  - `Facts -> Knowledge Planning -> Research -> Compose -> Assemble`
- “知识单元（KnowledgeUnit）”是一等抽象，“页面类型”是结果，不要再回到 `module/topic/family` 直接驱动一切的旧思路。
- `overview / architecture / module / topic / family-*` 这些页面语义如果需要新增或调整，必须先回答：
  - 它对应的 KnowledgeDomain / KnowledgeUnit 是什么
  - 它属于哪一层输出
- 生成链优先参考：
  - `CodeWiki` 的 `leaf-first + parent-consume-child`
  - `deepwiki-rs` 的 `research-first + compose consume research`
  - `GitNexus` 的厚 facts / 图事实层
- `deepwiki-open` 只用于 query / RAG / session / 消费层参考，不作为 core 生成主链模板。

# 当前验收约束

- 当前阶段的页面质量专项验收样本是：
  - `storybook`
  - `dagger`
- 这两个样本用于验证 core 抽象是否正确，不允许把样本仓库名、reference 标题或固定目录结构硬编码进 core 逻辑。
- 对这类样本仓库的结论，应沉淀为通用规则，例如：
  - `repo_archetype_signals`
  - `knowledge domain`
  - `knowledge unit`
  - `leaf decomposition policy`
  - `citation / diagram / compose policy`
- 不允许新增“为了某个样本过测试”的专有 planner / renderer 分支。

# 目录通配

- `crates/*`
  - Rust 包。`crates/wiki-core/**` 持有 repo facts、runtime、workflow、metadata、query。
- `agents/*`
  - 宿主接入层。只做参数收集、binary 调用、结果解析，不承载 Wiki 业务规则。
- `scripts/*.mjs`
  - 根级编排脚本。只负责 build / test / publish / dist 组装。
- `scripts/tests/*.test.ts`
  - 根级整体测试。只放跨包、staging、e2e、工作区级检查。
- `dist/**`
  - 发布产物目录，不写业务逻辑。
- `*.config.mjs` / `tsconfig*.json`
  - 共享工具链配置。TypeScript 与 ESLint 统一从根目录继承。

# 分层规则

- 命名统一使用 `Agents`，不要回退到 `Adapters`。
- `.wiki/*.md`、`.wiki/wiki.metadata.json`、`.wiki/.cache/**` 是三层 runtime，职责不能混用。
- `Facts -> Knowledge Planning -> Research -> Compose -> Assemble` 是当前 core 的主路径，新增实现不要绕过这条链路直接拼页面。
- `PageContext`、`PlannedPage`、`renderer` 都应服务于 KnowledgeUnit 主线，避免重新长出独立的旧页面语义层。
- `crates/*/package.json`
  - 子包自管 `build / test`。
- `agents/*/package.json`
  - 子包自管 `build / test`。
- 根级 `package.json`
  - 只保留总入口：`build`、`lint`、`test`、`publish`。

# 实施约定

- 优先修改已有模块，不平行复制实现。
- `*.ts` 使用根级 `tsconfig.base.json` 和 `eslint.config.mjs`。
- 注释以“帮助读懂”为目标，具体格式和粒度统一遵守 [COMMENTING.md](E:/project/!byAI/spec-wiki/COMMENTING.md)。
- 每轮 OpenSpec tasks 设计或测试时，都需要检查注释是否符合 [COMMENTING.md](E:/project/!byAI/spec-wiki/COMMENTING.md) 的要求（单独建立一个task）。
- 迭代归档时要commit一次
- 当前阶段不要求保留旧实现兼容层；如果 2.0 主线已经成立，旧字段、旧流程、旧 fallback 可以直接删除。
- 优先把局部修补收回通用抽象；不要把样本仓库经验直接落成硬编码规则。

# 测试与变更

- Rust 测试放 `crates/*/tests/`，Agent 测试放 `agents/*/src/*.test.ts`，跨模块测试放 `scripts/tests/*.test.ts`。
- 任何行为变化都要补对应层级的测试。
- 先看 `openspec/**` 当前 change；需求或设计变了，先改 OpenSpec，再改代码。
- 每轮 OpenSpec tasks 测试阶段，运行 `node scripts/run-test-projects.mjs` 批量对 19 个测试项目执行 `init`（也可指定项目：`node scripts/run-test-projects.mjs axum chi`）。
  - 测试项目集详见 [DESIGN2.0.md § 测试项目集](E:/project/!byAI/spec-wiki/DESIGN2.0.md)，覆盖 Rust/Go/Python/Java/TS/Vue/React/Android/运维等场景。
  - aLocal 和 spec-wiki 指向真实仓库 init 后拷贝回来，其余直接用 release 二进制 JSON IPC。
  - 脚本会保留 `.wiki/` 目录（已有则先删再重建），跑完后可直接检查 `tmp/test/*/.wiki/`。
  - 有 reference 的项目（`tmp/reference/*`）需对比页面结构和 `wiki.metadata.json`，发现差异后调整实现，并输出 `test-project-analysis.md`。
- 全生命周期验证运行 `node scripts/test-wiki-lifecycle.mjs`（也可指定项目），覆盖 init → status → sync → query → update → 模拟源码变更 → rebuild 全链路，验证 JSON 响应、marker 覆盖率和状态流转。
- 测试报告分析放在 `changes/**/reference-project-reports/*.md`。
- 当前做页面质量/结构专项时，优先跑 `storybook + dagger`，再决定是否回到全量 19 项目。
- 本地 provider / retry / timeout / backoff 等调试参数，测试时优先以 [wiki.dev.yaml](E:/project/!byAI/spec-wiki/wiki.dev.yaml) 为准。
