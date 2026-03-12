# Repo Wiki Design

## 项目目标

面向任意本地代码目录，自动生成、持续更新、可被 agent 消费的项目 Wiki。

目标能力整体对齐以下产品方向：

- [Qoder Repo Wiki](https://docs.qoder.com/user-guide/repo-wiki)
- [灵码 Repo Wiki](https://help.aliyun.com/zh/lingma/user-guide/repo-wiki)

## 项目定位

本项目定位为 Repo Wiki 核心引擎，当前实现边界仍以 Windows + CodeBuddy Agent 为唯一正式宿主，但路线已经重新收敛：

- 迭代 11 的目标是让 CodeBuddy Agent 对新一轮 LLM / research session 能力稳定可用；
- 迭代 12 才开始解除对单一平台和单一宿主的依赖。

其中：

- `wiki-core` 承载仓库分析、Wiki 生成、持续更新与查询等核心能力
- `Agents` 负责接入宿主、触发调用、参数传递与结果呈现
- Git 只是可选元信息来源，不是核心功能的硬前置条件
- 当前正式落地边界仍是 Windows 平台
- 当前正式宿主仍是 CodeBuddy Agent
- 迭代 11 先把 CodeBuddy Agent 可用性做实
- 跨平台与跨 IDE / CLI / 多宿主扩展放在迭代 12 推进

## 设计文档索引

| 文档 | 内容 |
|------|------|
| [DESIGN-CORE.md](./DESIGN-CORE.md) | wiki-core 核心 pipeline、数据模型、工作流与 LLM 使用原则 |
| [DESIGN-ITER.md](./DESIGN-ITER.md) | 实施迭代规划、重排历史与各迭代详细定义 |
| [DESIGN-COMPARE.md](./DESIGN-COMPARE.md) | 与 GitNexus / deepwiki-rs / deepwiki-open / codewiki 的对比分析与借鉴策略 |

## 总体架构

```text
Host Agents
CodeBuddy / CLI / VS Code / JetBrains / ...
            |
            v
Wiki Core Engine
scan / model / generate / update / query / sync
            |
            v
Repo Wiki Runtime
.wiki/*.md + wiki.metadata.json + .cache/
```

职责划分：

- Host Agents
  - 负责接入宿主、触发调用、参数传递与结果展示
- Wiki Core Engine
  - 负责扫描仓库、生成 Wiki、持续更新、查询与同步
- Repo Wiki Runtime
  - 负责承载正式 Wiki 页面、索引和缓存

## 运行产物

目标仓库中的运行产物统一写入 `.wiki/`：

```text
.wiki/
├─ 项目概述.md
├─ xxxx.md
├─ wiki.metadata.json
└─ .cache/
```

约束：

- `.wiki/*.md` 是正式 Wiki 文档，可提交到 Git
- `wiki.metadata.json` 是正式索引，可提交到 Git，其结构以 `tmp/reference/aLocal/meta/repowiki-metadata.json` 为主要参考
- `.wiki/.cache/` 是运行时缓存，不上库

## 测试项目集

用于验证 scanner 和 hierarchy 在多语言、多框架、多规模下的表现。

- 默认样本仓库位于 `tmp/test/`
- `aLocal` 的真实源仓库位于 `E:\project\aLocal`
- `spec-wiki` 的真实源仓库就是当前仓库 `E:\project\!byAI\spec-wiki`
- 每轮迭代的 task 设计或测试时，都必须对该项目集全量执行 `init` 并分析结果；如果存在 reference（`tmp/reference/`），则必须同步对照 `.wiki/*.md` 和 `wiki.metadata.json`。

| 项目 | 类型 | 文件数 | 仓库 | 说明 |
|------|------|--------|------|------|
| aLocal | 参考项目 | 203 | E:\project\aLocal | 混合语言参考项目（排除 node_modules/dist 等） |
| spec-wiki | 自测项目 | 131 | E:\project\!byAI\spec-wiki | 本项目自身（git tracked） |
| axum | Rust Web | 515 | tokio-rs/axum | Tokio 生态框架 |
| bat | Rust CLI | 917 | sharkdp/bat | Cargo workspace |
| cobra | Go CLI | 93 | spf13/cobra | 扁平 Go 结构 |
| chi | Go 路由 | 122 | go-chi/chi | 轻量 Go web |
| gin | Go Web | 158 | gin-gonic/gin | 流行 Go 框架 |
| httpx | Python HTTP 客户端 | 152 | encode/httpx | pyproject.toml |
| django-ninja | Python Django | 334 | vitalik/django-ninja | Django 生态 |
| fastapi | Python FastAPI | 2942 | tiangolo/fastapi | 框架源码，docs 密集 |
| spring-petclinic | Java Spring | 154 | spring-projects/spring-petclinic | Maven + Spring Boot |
| leakcanary | Java Android | 785 | square/leakcanary | Gradle + Kotlin/Java |
| dagger | Java Gradle | 3458 | google/dagger | 大型 DI 框架 |
| zustand | React 状态库 | 170 | pmndrs/zustand | 纯 TS，小型 |
| pinia | Vue 状态库 | 357 | vuejs/pinia | pnpm workspace + Vue SFC |
| wot-starter | UniApp 启动模板 | 325 | wot-ui/wot-starter | Vue3 + TS + wot-design-uni |
| storybook | 前端 monorepo | 5446 | storybookjs/storybook | 大型 UI 工具，多 workspace 多入口 |
| docker-mailserver | 运维/产品化编译 | 454 | docker-mailserver/docker-mailserver | shell + nginx + Docker |
| restaurant-app | 多语言混合 | 1006 | chayxana/Restaurant-App | Java + Go + Next.js 微服务 |

覆盖范围：
- 语言/框架：Rust (bat, axum)、Go (cobra, chi, gin)、Python (httpx, django-ninja, fastapi)、Java (spring-petclinic, dagger, leakcanary)、TS/JS (zustand, pinia, storybook)、Vue (pinia, wot-starter)、React (zustand, storybook)、UniApp、Android、运维脚本、混合语言
- Manifest 类型：Cargo.toml、package.json、pyproject.toml、go.mod、pom.xml、build.gradle、Makefile、Dockerfile、nginx.conf、shell scripts、manifest.config.ts (uniapp)
- 规模分布：小型 (<200 文件) 7 个、中型 (200-500) 4 个、中大型 (500-1000) 3 个、大型 (>2000) 4 个
- 结构特征：workspace、monorepo、单体、混合语言、CI/CD 密集、文档密集、多入口、跨平台

## 实施迭代

详见 [DESIGN-ITER.md](./DESIGN-ITER.md)。

当前进度：迭代 9、9.1、9.2 已归档；当前 active change 暂空。路线调整后，迭代 11 负责 CodeBuddy Agent 可用化，迭代 12 再承担跨平台与跨宿主扩展；下一轮应优先聚焦页面质量、research session 深度和 evidence/source layer 精度，而不是继续只调 prompt。

## 待探究 TODO

- [ ] 明确 `community` 在本项目中的目标口径：是仅作为 `query / planner / workflow` 的辅助聚类信号，还是要追求接近 GitNexus/图分析工具的社区质量。
- [ ] 记录当前实现边界：现阶段是“加权聚类近似 + deterministic fallback”，不是严格意义上的 Leiden；后续讨论和验收都必须区分“已有 community 能力”和“是否已采用 Leiden”。
- [ ] 评估 Leiden 对当前项目的真实收益：
  - 是否能明显改善大型仓库中的弱桥接拆分效果，例如 `storybook`、`dagger`、`spec-wiki`
  - 是否能实际提升 `.wiki` 页面组织、workflow 页面信息密度和 `query` 的图上下文质量
- [ ] 评估 Leiden 的工程成本：
  - Rust 生态里是否有可接受的 Leiden 实现可直接复用
  - 若无现成实现，自研或移植是否会显著抬高维护成本、平台风险和调试复杂度
- [ ] 明确图输入规范，避免 Leiden 接入前后口径漂移：
  - 使用哪些边类型参与 community
  - 置信度如何映射为边权
  - 大图场景下哪些低质量边/低度节点应先降噪
- [ ] 明确稳定性要求：即使引入 Leiden，结果也必须尽量 deterministic，至少要控制随机种子、迭代上限和排序规则，避免页面/metadata 抖动。
- [ ] 设计 fallback 策略：Leiden 不可用、超时或结果退化时，必须无缝回退到当前 deterministic 近似实现，不能阻断 init/update/rebuild。
- [ ] 设计验证指标，而不是只看“算法名是否对”：
  - 社区数量是否更合理
  - 社区内聚度/跨社区桥接是否改善
  - 重复运行结果是否稳定
  - 19 个测试项目上的耗时是否可接受
- [ ] 在项目集上做 A/B 对比：
  - 当前实现 vs Leiden
  - 对比 `communities` 数量、分布、稳定性、query 命中质量、workflow 页面变化
- [ ] 设定最终决策门槛：
  - 若 Leiden 只能带来轻微聚类提升，但明显增加复杂度，则保留当前实现
  - 若 Leiden 在大仓库上能稳定改善 community 质量且成本可控，则进入后续迭代正式替换
