# 测试项目集分析 — Iteration 6

## 运行概况

- `run-test-projects.mjs`：19/19 项目 init 成功，0 失败。
- `test-wiki-lifecycle.mjs`：305/305 检查通过（init → status → sync → query → update → rebuild）。
- `cargo test -p wiki-core`：全部通过，0 失败。

## 页面拓扑与合并效果

| 项目 | 页面数 | 模块数 | 关系数 | 备注 |
|------|--------|--------|--------|------|
| aLocal | 6 | 5 | 7 | 嵌套：nginx/conf 作为 nginx 子页面 |
| axum | 6 | 5 | 12 | 4 个 workspace crate 各有独立页面 |
| bat | 15 | 14 | 17 | 深层嵌套：tests/syntax-tests 下 3 层子页面 |
| chi | 3 | 1 | 2 | 单模块，无合并 |
| cobra | 3 | 1 | 2 | 单模块，无合并 |
| dagger | 27 | 26 | 119 | 大型 Java 多模块，全部保留独立页面 |
| django-ninja | 5 | - | - | Python 项目 |
| docker-mailserver | 3 | - | - | 基础设施项目 |
| fastapi | 7 | - | - | Python 项目 |
| gin | 3 | 1 | 2 | 单模块 Go 项目 |
| httpx | 4 | - | - | Python HTTP 客户端 |
| leakcanary | 9 | - | - | Android 多模块 |
| pinia | 15 | 14 | 26 | 嵌套：packages 下多层子页面，scripts 独立 |
| restaurant-app | 13 | 12 | 17 | 深层嵌套：src/backend/services 下多个微服务 |
| spec-wiki | 7 | 6 | 7 | 自身仓库：agents/codebuddy、crates/wiki-core、scripts |
| spring-petclinic | 3 | - | - | Java 单模块 |
| storybook | 67 | 66 | 385 | 超大 monorepo，层级化页面拓扑完整 |
| wot-starter | 7 | - | - | IoT 项目 |
| zustand | 3 | 2 | 2 | 小型 monorepo，src 模块独立 |

## 关键观察

### 页面拓扑稳定性
- 所有项目的 overview 和 architecture 页面始终生成。
- 嵌套模块（pinia、restaurant-app、storybook、bat）正确建立了层级父子关系，不再全部扁平挂在 overview 下。
- storybook（67 页）展示了深层嵌套的正确处理：code/addons/a11y 等路径正确映射为多级子页面。

### 合并策略效果
- 小型项目（chi、cobra、gin、zustand）的模块权重足够，未触发不必要的合并。
- 大型 monorepo（dagger 26 模块、storybook 66 模块）中所有 workspace 成员因权重加成保留独立页面。
- spec-wiki 自身：agents 和 crates 各有子模块页面，scripts 独立。

### Section 内容密度
- overview 页面包含技术栈和入口与构建 section。
- architecture 页面包含层级化模块结构文本。
- module 页面包含关键源码、依赖关系、子模块概述 section。
- 所有页面使用 managed section markers。

### SQLite 存储
- 全生命周期（init → update → sync → rebuild）中 DB 创建、读写、清理行为正确。
- rebuild 后页面数量与 init 一致。

### Steering 配置
- 未配置 steering 的项目使用默认值，行为正确。
- 内置 per-language 忽略规则自动排除 node_modules、__pycache__、.next 等。
