# 测试项目集分析 — Iteration 6.5

## 说明

以下结论基于 2026-03-10 已完成的验证结果，以及当前保留在 `tmp/test/*/.wiki` 与 `tmp/reference/*` 的产物复核。
本次整理没有重新运行脚本，只是把已有结果按归档的迭代 6 口径补齐到本 change 下。

## 运行概况

- `run-test-projects.mjs`：19/19 项目 `init` 成功，0 失败。
- `test-wiki-lifecycle.mjs`：305/305 检查通过（`init → status → sync → query → update → rebuild`）。
- `cargo test -p wiki-core`：全部通过，0 失败。

## 项目级结果复核

| 项目 | 页面数 | 模块数 | 关系数 | 备注 |
|------|--------|--------|--------|------|
| aLocal | 6 | 5 | 7 | 业务仓库维持紧凑层级；相对 reference 仍明显收敛 |
| axum | 6 | 5 | 12 | Rust workspace crate 继续保留独立页面 |
| bat | 14 | 14 | 16 | 深层目录仍能形成层级页面；较 reference 更紧凑 |
| chi | 4 | 1 | 3 | 小型 Go 项目，页面数受控，没有被 docs 噪声放大 |
| cobra | 4 | 1 | 3 | 小型 Go 项目，拓扑稳定 |
| dagger | 27 | 26 | 119 | 大型 Java 多模块，模块页保留完整 |
| django-ninja | 5 | 4 | 4 | Python 项目，模块/关系规模稳定 |
| docker-mailserver | 4 | 2 | 3 | 基础设施仓库，页面结构保持紧凑 |
| fastapi | 7 | 6 | 6 | Python 服务型仓库，模块拆分正常 |
| gin | 4 | 1 | 3 | 单模块 Go 项目，无异常膨胀 |
| httpx | 4 | 3 | 3 | docs 密集仓库仍保持少量高信号页面 |
| leakcanary | 9 | 8 | 15 | Android 多模块层级保持稳定 |
| pinia | 15 | 14 | 26 | `packages/*` 层级和独立页面保留 |
| restaurant-app | 14 | 12 | 18 | 前后端/服务目录结构仍可拆出多级模块页 |
| spec-wiki | 7 | 6 | 7 | 自身仓库页面规模稳定，主模块结构未被打散 |
| spring-petclinic | 4 | 2 | 3 | Java 单体项目，页面保持紧凑 |
| storybook | 67 | 66 | 385 | 超大 monorepo 仍可完整表达深层模块关系 |
| wot-starter | 7 | 6 | 6 | IoT/前端混合仓库，模块拆分正常 |
| zustand | 3 | 2 | 2 | 极小 monorepo 继续维持最小页面集 |

## 与归档 Iteration 6 对照

- 关键样本的拓扑类型没有变化：`axum`、`dagger`、`pinia`、`storybook`、`spec-wiki` 仍分别表现为 workspace/大型多模块/包仓库/超大 monorepo/自身仓库的稳定层级输出。
- 个别项目相对归档迭代 6 出现了 `±1` 级别的页面或关系浮动，例如 `bat`、`chi`、`cobra`、`gin`、`restaurant-app`、`spring-petclinic`。从生命周期脚本与包内测试结果看，这些更像当前主干 planner 或模板演进后的自然差异，而不是 SQLite 行式状态、`FilePurpose` 或 FTS 接入导致的 runtime 断裂。
- reference 项目的页面数量仍显著高于当前输出，例如 `storybook` 当前为 `67` 页，而 reference `content/` 为 `176` 页；这延续了迭代 6 已确认的产品边界：当前实现选择更紧凑的层级 wiki，而不是追求与 reference 的页数一致。

## 本轮关注点复核

### SQLite 关系型状态与缓存

- 19 个项目 `init` 全部成功，说明关系型状态表、`scan_cache`、page cache 和 FTS 初始化没有引入跨语言失败。
- 生命周期脚本 305/305 通过，说明 `status`、`sync`、`update`、`rebuild` 都能持续消费 SQLite runtime，而不是只在 `init` 阶段可用。
- 当前保留产物的页面数、模块数、关系数分布与归档迭代 6 同阶，没有出现明显“状态丢失后大面积降页”或“关系装配失败后大量归零”的迹象。

### `FilePurpose` 与扫描边界

- `httpx`、`django-ninja`、`wot-starter` 这类 docs 较多或目录类型混合的仓库，页面规模依旧受控，说明更细的文件角色分类没有让低信号文件重新主导页面规划。
- `storybook`、`pinia`、`restaurant-app` 这类大型或多包仓库仍保留深层模块结构，说明 `scan.ignore` / `scan.include` 与 `FilePurpose` 的组合没有误伤核心源码目录。

### Query / FTS5

- 生命周期中的 `query` 步骤在完整项目集上通过，未出现 FTS 表初始化、索引刷新或结构化结果合并导致的脚本级失败。
- 本轮变更重心是把 SQLite 变成事实主存储并接入 BM25；从 retained outputs 和生命周期结果看，查询能力增强没有破坏现有 `status / update / rebuild` 语义。

## Reference 对照备注

- `aLocal`、`axum`、`bat`、`chi`、`cobra`、`dagger`、`pinia`、`restaurant-app`、`storybook`、`zustand` 的 retained `.wiki/` 与 `wiki.metadata.json` 都存在，可用于对照 reference 的页面组织方式和 metadata 字段形态。
- 本轮没有观察到脚本级 reference 对照失败；同时也没有迹象表明 SQLite / FTS / scanner 新实现把页面拓扑拉回到扁平或失去模块层级。
- reference 仍主要作为页面组织和 metadata 字段的参照物，而不是页数或模块数的逐项 gold baseline。

## 结论

- 迭代 6.5 的核心改动集中在 runtime 存储、扫描角色分类、FTS 检索和 steering 扫描边界；从保留产物和既有验证结果看，没有引入测试项目集层面的结构性回归。
- 相比归档的迭代 6，本轮更像是在既有页面拓扑之下替换和加固 runtime 内核，而不是重新改变页面产品形态。
