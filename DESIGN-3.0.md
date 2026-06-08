# Repo Wiki Design 3.0

## 项目目标

面向任意本地代码目录，构建一套可持续更新、可被人和 Agent 共同消费的项目知识系统。

3.0 的核心转向不是“继续生成更多 page”，而是把 Repo Wiki 收束为：

```text
project knowledge runtime + auditable long-term memory
```

其中：

- page 是知识投影，不是知识主本体
- Agent 查询优先走索引、图关系和知识记录，而不是 page-first
- 团队声明的规范、约定、避坑、决策，需要进入正式知识层，而不是散落在临时文档里

## 项目定位

当前项目定位为 `Repo Wiki Core + Agents` 体系。core 的内部架构已经收束为四个 crate，宿主接入、bootstrap 与全局 CLI 形态则单独由 `Agents` 层承载，具体见 [DESIGN-AGENTS.md](./DESIGN-AGENTS.md)：

- `wiki-model`
- `wiki-index`
- `wiki-knowledge`
- `wiki-runtime`

其中：

- `wiki-model` 提供稳定共享模型
- `wiki-index` 负责代码事实与索引
- `wiki-knowledge` 负责把事实组织成知识
- `wiki-runtime` 负责运行时、工作流、查询路由、投影与宿主编排
- `Agents` 只负责接入宿主，不承载 Wiki 业务规则；宿主接入与 bootstrap 设计见 [DESIGN-AGENTS.md](./DESIGN-AGENTS.md)

## 设计文档索引

| 文档 | 内容 | 状态 |
|------|------|------|
| [DESIGN-RUNTIME.md](./DESIGN-RUNTIME.md) | 整个 wiki 系统的总运行架构、`.wiki/` 结构、query route、生命周期与恢复策略 | **当前** |
| [DESIGN-AGENTS.md](./DESIGN-AGENTS.md) | `spec-wiki` 的宿主接入、bootstrap、runtime forwarding 与宿主扩展模型 | **当前** |
| [.docs/roadmap/implementation-roadmap.md](./.docs/roadmap/implementation-roadmap.md) | 3.0 的拆包顺序、迁移路线、验收重点、**阶段性迭代参考** | 阶段材料 |
| [SCENE-1.md](./SCENE-1.md) | 第一版 9 个核心用户故事 | **当前** |
| [SCENE-2.md](./SCENE-2.md) | 第一版之外的重要扩展场景 | **当前** |
| [.archive/design/README.md](./.archive/design/README.md) | 旧 DESIGN 文档归档说明与映射关系 | 历史归档 |

## 核心设计原则

### 1. Repo Wiki 是知识运行时，不是 page 工厂

系统的主目标是建立可查询、可更新、可审计的知识层。page 仍然保留，但只作为稳定阅读投影和协作锚点。

### 2. 知识单元优先于页面类型

当前主链仍以 `KnowledgeDomain / KnowledgeUnit / KnowledgeTree` 为一等抽象。`overview / architecture / conventions / troubleshooting` 等页面语义都只是知识投影结果。

### 3. 知识分层必须分清

3.0 明确区分三类对象：

- `Code Facts / Index`
- `Derived Knowledge`
- `Declared Knowledge`

这三层不能混成一个“wiki 内容桶”。

### 4. 查询优先服务 Agent 快速定位

当用户或 Agent 查询方法、入口、影响范围时，优先返回文件、符号、图关系和知识记录，再决定是否补 page 投影。

### 5. 长期知识必须可审计

规范、约定、避坑、政策、决策等长期知识必须具备：

- 结构化记录
- 作用范围
- 状态与来源
- 可投影为人读入口

### 6. 参考实现只服务具体借鉴，不反向绑死当前设计

本项目在实现层会持续参考本地 upstream 仓库的实际源码，但当前设计真相仍以本仓库自身设计文档和场景文档为准。

约束：

- 参考仓库必须优先看实际源码实现，而不是只看 README、产品说明或二手总结
- 参考结论如果没有被重新验证到本地源码，不自动成立
- 旧设计文档中遗留的参考映射，如果不再适配 3.0 当前思想，可以直接推翻
- 参考仓库只提供“实现灵感”和“工程做法”，不直接决定本项目的产品边界和目录结构

补充边界：

Karpathy 的 `LLM Wiki` 可作为本项目的理念侧参考：二者都反对把知识系统退化为一次性 query 结果的临时拼接，而强调通过持续 ingest、增量修订与长期维护，让知识沉淀为能够随时间复利的正式资产。这个参考主要帮助说明“为什么需要长期知识层”，不直接决定当前系统的对象模型、包边界或 `.wiki/` 结构。

但 `spec-wiki` 并不是 page-first 的 `LLM Wiki` 变体。当前系统仍以 `facts / index` 为底座，以 `KnowledgeUnit` 为一等抽象，沿 `Facts -> Knowledge Planning -> Research -> Compose -> Assemble` 主链生成正式知识产物，并显式区分 `declared knowledge`、`derived knowledge`、`page projection` 与 `runtime cache`。因此，`LLM Wiki` 只能作为产品哲学与知识工作流灵感参考，不能作为 core 架构模板或页面语义设计依据。

## 四包架构

```text
crates/
  wiki-model
  wiki-index
  wiki-knowledge
  wiki-runtime
```

四包一句话总览：

- `wiki-model`：稳定共享对象语言
- `wiki-index`：代码事实与索引层
- `wiki-knowledge`：把事实组织成知识
- `wiki-runtime`：运行时与宿主编排壳

更细的包边界、运行协作和主链说明，见 [DESIGN-RUNTIME.md](./DESIGN-RUNTIME.md)。宿主接入、bootstrap 与扩展适配设计见 [DESIGN-AGENTS.md](./DESIGN-AGENTS.md)。

## 依赖方向

```text
wiki-model      <- 所有包依赖它
wiki-index      <- 依赖 wiki-model
wiki-knowledge  <- 依赖 wiki-model + wiki-index
wiki-runtime    <- 依赖 wiki-model + wiki-index + wiki-knowledge
```

系统同时存在两条正式主链：

- 生成链：`Facts -> Knowledge Planning -> Research -> Compose -> Assemble`
- 查询链：`symbol -> graph -> declared knowledge -> derived knowledge -> page`

具体运行顺序、query route 和生命周期细节，见 [DESIGN-RUNTIME.md](./DESIGN-RUNTIME.md)。

## 与用户场景的关系

3.0 的验收不再只看“能不能生成 `.wiki/*.md`”，而要对应 [SCENE-1.md](./SCENE-1.md) 和 [SCENE-2.md](./SCENE-2.md) 的真实使用场景。

核心对应关系：

- 项目理解、改动入口定位、影响分析：优先依赖 `wiki-index`
- 规则读取、知识更新、避坑沉淀、冲突治理、主动声明规范：优先依赖 `wiki-knowledge`
- 本地恢复、runtime 生命周期、查询路由与宿主接入：优先依赖 `wiki-runtime`

特别关注的核心场景包括：

- `init` 后快速理解项目
- Agent 开工前读取规范与约定
- Agent 开工前找到改动入口并分析影响范围
- 新功能后更新相关知识
- bug 修复后沉淀避坑记录
- 主动新增规范，例如注释规范
- 规范与现实代码冲突时显式治理
- A 提交后，B 拉代码可基于 `.wiki` 正式产物恢复本地 runtime

## 测试项目集

测试项目集继续沿用当前 `aLocal / spec-wiki / storybook / dagger / axum / bat / cobra / chi / gin / httpx / django-ninja / fastapi / spring-petclinic / leakcanary / zustand / pinia / wot-starter / docker-mailserver / restaurant-app`。

这组样本是 3.0 的验收基座，不只是脚本执行清单。

它们的职责：

- 验证四层架构在多语言、多结构、多规模仓库下是否成立
- 验证 `Facts -> Knowledge Planning -> Research -> Compose -> Assemble` 主链是否能稳定落地
- 验证 query、update、sync、rebuild 等 runtime 行为是否能跨项目复用
- 验证知识抽象是否足够通用，而不是只对少数仓库有效

实施边界：

- 样本用于验证抽象，不允许把样本仓库名、reference 标题或固定目录结构硬编码进 core
- `storybook` 与 `dagger` 继续作为页面质量和知识抽象的重点专项样本
- 其余项目用于拉开语言、结构、规模与工程形态覆盖面

样本来源与执行口径：

- 默认样本仓库位于 `tmp/test/`
- `aLocal` 的真实源仓库位于 `E:\project\aLocal`
- `spec-wiki` 的真实源仓库就是当前仓库 `E:\project\!byAI\spec-wiki`
- 每轮相关任务设计、实现或测试时，都应对该项目集执行 `init`；有 reference 的项目还要对照 `.wiki/*.md` 与 `wiki.metadata.json`

| 项目 | 类型 | 文件数 | 仓库 | 说明 |
|------|------|--------|------|------|
| `aLocal` | 参考项目 | 203 | `E:\project\aLocal` | 混合语言参考项目，排除 `node_modules / dist` 等噪声目录 |
| `spec-wiki` | 自测项目 | 131 | `E:\project\!byAI\spec-wiki` | 本项目自身，验证真实演进中的自举能力 |
| `axum` | Rust Web | 515 | `tokio-rs/axum` | Tokio 生态框架 |
| `bat` | Rust CLI | 917 | `sharkdp/bat` | Cargo workspace |
| `cobra` | Go CLI | 93 | `spf13/cobra` | 扁平 Go 结构 |
| `chi` | Go 路由 | 122 | `go-chi/chi` | 轻量 Go web |
| `gin` | Go Web | 158 | `gin-gonic/gin` | 流行 Go 框架 |
| `httpx` | Python HTTP 客户端 | 152 | `encode/httpx` | `pyproject.toml` 项目 |
| `django-ninja` | Python Django | 334 | `vitalik/django-ninja` | Django 生态 |
| `fastapi` | Python FastAPI | 2942 | `tiangolo/fastapi` | 框架源码，docs 密集 |
| `spring-petclinic` | Java Spring | 154 | `spring-projects/spring-petclinic` | Maven + Spring Boot |
| `leakcanary` | Java Android | 785 | `square/leakcanary` | Gradle + Kotlin/Java |
| `dagger` | Java Gradle | 3458 | `google/dagger` | 大型 DI 框架，runtime-heavy 专项样本 |
| `zustand` | React 状态库 | 170 | `pmndrs/zustand` | 纯 TS，小型 |
| `pinia` | Vue 状态库 | 357 | `vuejs/pinia` | `pnpm` workspace + Vue SFC |
| `wot-starter` | UniApp 启动模板 | 325 | `wot-ui/wot-starter` | Vue3 + TS + `wot-design-uni` |
| `storybook` | 前端 monorepo | 5446 | `storybookjs/storybook` | 大型 UI 工具，多 workspace、多入口，docs-heavy 专项样本 |
| `docker-mailserver` | 运维/产品化编译 | 454 | `docker-mailserver/docker-mailserver` | shell + nginx + Docker |
| `restaurant-app` | 多语言混合 | 1006 | `chayxana/Restaurant-App` | Java + Go + Next.js 微服务 |

覆盖范围：

- 语言与框架：Rust、Go、Python、Java、TS/JS、Vue、React、UniApp、Android、运维脚本、混合语言
- Manifest 类型：`Cargo.toml`、`package.json`、`pyproject.toml`、`go.mod`、`pom.xml`、`build.gradle`、`Makefile`、`Dockerfile`、`nginx.conf`、shell scripts、`manifest.config.ts`
- 规模分布：小型 `<200 文件`、中型 `200-500`、中大型 `500-1000`、大型 `>2000`
- 结构特征：workspace、monorepo、单体、混合语言、CI/CD 密集、文档密集、多入口、跨平台

与实施脚本的关系：

- 项目集跑批与生命周期验证的具体执行方式，见 [.docs/roadmap/implementation-roadmap.md](./.docs/roadmap/implementation-roadmap.md)
- 日常测试约束以 [AGENTS.md](./AGENTS.md) 中的测试与变更章节为准

## 当前阶段的设计约束

- 不回到 page-first 的旧路径
- 当前阶段不拆 `wiki-page / wiki-storage / wiki-llm / wiki-query`
- 不把样本仓库经验硬编码进 core 逻辑
- 不允许把 declared knowledge、derived knowledge、facts index 混成一层
- 不允许让 `wiki-runtime` 再次变回“万物中心大包”
- 不允许新增只为某个参考仓库过样例的特化分支
- 不允许把未经重新验证的旧参考结论继续当成当前设计依据

## 一句话总结

```text
Repo Wiki 3.0 的目标不是“生成更多页面”，
而是把 page 收为投影，把知识做成运行时，把规范和经验做成可审计的长期记忆。
```


