# 测试项目集分析 — Iteration 7

## 运行概况

- `cargo build --release -p wiki-core`：已重建 release binary，避免项目集脚本误用旧产物。
- `node scripts/run-test-projects.mjs`：19/19 项目 `init` 成功，0 失败。
- `node scripts/test-wiki-lifecycle.mjs --phase bootstrap`：19/19 项目通过，`147/147`。
- `node scripts/test-wiki-lifecycle.mjs --phase steady`：19/19 项目通过，`249/249`。
- `node scripts/test-wiki-lifecycle.mjs --phase mutation`：19/19 项目通过，`228/228`。
- `node scripts/test-wiki-lifecycle.mjs --phase rebuild`：19/19 项目通过，`257/257`。
- 项目集总页面数：215，managed marker 覆盖 `215/215`。
- 项目集总 symbols：51,300；18/19 项目写入了非空 `symbols`，18/18 个有 symbols 的项目都能用代表性符号词拿到精确 `matched_symbols` 命中。
- 生命周期分阶段验证已覆盖 `init -> status -> sync -> query -> update -> rebuild` 全链路，并额外校验了 symbol snapshot 可读性、symbol count 稳定性和代表性 symbol query 命中。

## 本轮验证结论

- `symbols` / `symbols_fts` 已经真正写盘，不再是空 schema。大型项目如 `dagger`、`storybook`、`leakcanary` 都能稳定写入千级到万级符号。
- 项目集分析过程中暴露并修正了两个真实问题：一是测试脚本对 `init/update/rebuild` 的固定 `60s` 超时会把 `storybook` 误判失败，现已提升 heavy actions 默认超时到 `180s`；二是 Kotlin query 沿用了旧 grammar 的节点名，导致 `leakcanary` 近乎空跑，现已按 `tree-sitter-kotlin-ng` 的真实 `identifier` 字段修正并补单测。
- 生命周期分阶段验证又补出了两处实现细节并已修正：mutation 阶段必须优先 touch `source_states` 中已跟踪的源码文件，避免误触未纳入 runtime 的文件导致 `status` 仍是 `fresh`；Windows 下 `rebuild` 删除 runtime 和 SQLite 文件时需要对短暂文件锁做重试，避免 `os error 32` 误伤。
- 包装语言边界与设计一致：`React(.jsx/.tsx)` 正常走底层 JS/TS grammar，`Vue/Svelte` 仍然 fail-soft，并明确放到迭代 8 再补包装层。`wot-starter` 的 symbol 主要来自 TS/JS/Python 工具与 composables，而不是 `.vue` 本体；`docker-mailserver` 由于主要是 shell/config 资产，symbol 结果为空但 workflow 正常。
- 页面拓扑没有因为 symbol 层接入而被破坏。所有项目仍保留 overview / architecture / module 的主结构；与 reference 的差异仍主要在内容深度和页面组织策略，而不是 symbol 写盘引发的拓扑回退。
- 分阶段 lifecycle 结果证明 symbol snapshot 与 query contract 在生命周期内保持一致：有 symbols 的项目在 `steady / mutation / rebuild` 阶段都能维持稳定 symbol count 和 exact symbol hit；无 symbols 项目走显式 skip；real-repo 目标因 `.wiki` 路径不在 `repoRoot` 只跳过 symbol query，但 workflow/state 一致性仍被验证。

## 项目逐项分析

### aLocal

- `init`：成功，`6` 页 / `5` 模块 / `112` 源文件，marker `6/6`。
- `symbol`：`217` 个，`165` 个 exported；语言分布为 `python:182`、`typescript:35`，以 `function:188`、`method:14` 为主。
- `query`：用 `get_html` 验证，`matched_symbols=24`，精确命中 `spider/modules/utils.py`，并回填 `4` 个页面。
- `reference`：有 reference，`86` 页；当前 `6` 页仍是模块骨架页，差异来自 reference 的内容导向展开，不是 symbol 层回归。

### axum

- `init`：成功，`6` 页 / `5` 模块 / `440` 源文件，marker `6/6`。
- `symbol`：`3021` 个，全部来自 `rust`；主标签为 `function:1750`、`struct:363`、`impl:278`。
- `query`：用 `echo_app` 验证，精确命中 `axum/src/extract/ws.rs` 中同名函数，`matched_symbols=24`，回填 `6` 个页面。
- `reference`：有 reference，`111` 页；当前仍保持 crate 级骨架页，说明 symbol 层接入没有破坏既有模块边界。

### bat

- `init`：成功，`14` 页 / `14` 模块 / `904` 源文件，marker `14/14`。
- `symbol`：`2003` 个，`1766` 个 exported；这是项目集中最强的多语言压力样本之一，当前已覆盖 `rust / python / javascript / kotlin / typescript / csharp / c / php / cpp / go / java`。
- `query`：用 `assert_c` 验证，精确命中 `tests/benchmarks/highlighting-speed-src/numpy_test_multiarray.py` 中符号，`matched_symbols=24`，回填 `13` 个页面。
- `reference`：有 reference，`79` 页；当前 `14` 页仍偏模块/目录导向，且测试夹具子树依旧很重，但 symbol 提取没有引入新噪声。

### chi

- `init`：成功，`5` 页 / `3` 模块 / `157` 源文件，marker `5/5`。
- `symbol`：`458` 个，全部来自 `go`；以 `function:225`、`method:127`、`type:58` 为主。
- `query`：用 `paginate` 验证，精确命中 `_examples/rest/main.go`，`matched_symbols=1`，回填 `3` 个页面。
- `reference`：有 reference，`65` 页；当前仍是轻量 Go 项目的骨架页布局，没有因为 symbol 层扩张成内容页。

### cobra

- `init`：成功，`4` 页 / `2` 模块 / `101` 源文件，marker `4/4`。
- `symbol`：`625` 个，全部来自 `go`；以 `function:420`、`method:168` 为主。
- `query`：用 `findFlag` 验证，精确命中 `completions.go`，`matched_symbols=1`，回填 `2` 个页面。
- `reference`：有 reference，`38` 页；当前结果仍是小型库的模块骨架，差异集中在内容深度。

### dagger

- `init`：成功，`27` 页 / `26` 模块 / `3449` 源文件，marker `27/27`。
- `symbol`：`16995` 个，`11770` 个 exported；语言分布为 `java:13032`、`kotlin:3951`、`python:12`，说明 Kotlin 现在已经真实落盘，不再空跑。
- `query`：用 `factoryOf` 验证，精确命中 `dagger-compiler/main/java/dagger/internal/codegen/xprocessing/XTypeNames.kt`，`matched_symbols=1`，回填 `4` 个页面。
- `reference`：有 reference，`65` 页；当前 `27` 页仍与 Gradle 模块边界对齐，symbol 层没有破坏多模块拓扑。

### django-ninja

- `init`：成功，`5` 页 / `4` 模块 / `296` 源文件，marker `5/5`。
- `symbol`：`1821` 个，全部来自 `python`；以 `function:1387`、`class:434` 为主。
- `query`：用 `clean_db` 验证，精确命中 `tests/test_pagination_cursor.py`，`matched_symbols=1`，回填 `2` 个页面。
- `reference`：无 reference。

### docker-mailserver

- `init`：成功，`4` 页 / `2` 模块 / `252` 源文件，marker `4/4`。
- `symbol`：`0` 个；这与仓库主体语言偏 shell/config/infra 的现实一致，符合“只支持新版 grammar 已覆盖语言”的当前边界。
- `query`：无代表性 symbol 可验证，但 `.wiki` 页面、metadata 和 SQLite runtime 都正常生成。
- `reference`：无 reference。

### fastapi

- `init`：成功，`6` 页 / `5` 模块 / `2889` 源文件，marker `6/6`。
- `symbol`：`5267` 个，`5053` 个 exported；主语言是 `python:5242`，另有少量 `javascript:25`。
- `query`：用 `add_task` 验证，精确命中 `fastapi/background.py`，`matched_symbols=24`，回填 `5` 个页面。
- `reference`：无 reference。

### gin

- `init`：成功，`7` 页 / `5` 模块 / `114` 源文件，marker `7/7`。
- `symbol`：`1659` 个，全部来自 `go`；以 `function:882`、`method:402`、`type:205` 为主。
- `query`：用 `function` 验证，精确命中 `recovery.go` 中同名定义，`matched_symbols=24`，回填 `2` 个页面；说明 symbol FTS 与结构化回填已打通。
- `reference`：无 reference。

### httpx

- `init`：成功，`4` 页 / `3` 模块 / `115` 源文件，marker `4/4`。
- `symbol`：`1241` 个，全部来自 `python`；以 `function:1134`、`class:107` 为主。
- `query`：用 `compress` 验证，精确命中 `tests/test_decoders.py`，`matched_symbols=1`，回填 `2` 个页面。
- `reference`：无 reference。

### leakcanary

- `init`：成功，`9` 页 / `8` 模块 / `747` 源文件，marker `9/9`。
- `symbol`：`7714` 个，其中 `kotlin:7683`、`java:31`；这次项目集分析直接暴露了 Kotlin query 与 `tree-sitter-kotlin-ng` grammar 不对齐的问题，修正后已能稳定提取 `property / function / class / enum / interface`。
- `query`：用 `openFile` 验证，精确命中 `LeakCanaryFileProvider.kt`，`matched_symbols=1`，回填 `3` 个页面。
- `reference`：无 reference。

### pinia

- `init`：成功，`15` 页 / `14` 模块 / `376` 源文件，marker `15/15`。
- `symbol`：`398` 个，语言分布为 `typescript:378`、`javascript:20`；以 `function:190`、`method:161`、`interface:45` 为主。
- `query`：用 `copyFile` 验证，精确命中 `packages/online-playground/vite.config.ts`，`matched_symbols=1`，回填 `4` 个页面。
- `reference`：有 reference，`61` 页；当前 `15` 页仍沿 workspace/module 边界组织，差异主要是 reference 有更多概念页与 API 页。

### restaurant-app

- `init`：成功，`15` 页 / `13` 模块 / `1095` 源文件，marker `15/15`。
- `symbol`：`1756` 个，`1706` 个 exported；语言分布为 `csharp:1119`、`go:248`、`typescript:244`、`java:84`、`rust:58`、`javascript:3`，是本轮最典型的多语言服务样本。
- `query`：用 `get_dish` 验证，精确命中 `src/backend/services/catalog-api/src/seeder/seed.rs`，`matched_symbols=24`，回填 `8` 个页面。
- `reference`：有 reference，`127` 页；当前 `15` 页仍保持微服务模块骨架，symbol 接入没有让页面边界回退。

### spec-wiki

- `init`：成功，`7` 页 / `6` 模块 / `149` 源文件，marker `7/7`。
- `symbol`：`870` 个，语言分布为 `rust:792`、`javascript:66`、`typescript:12`；以 `function:644`、`struct:87`、`module:61` 为主。
- `query`：用 `callCore` 验证，精确命中 `scripts/test-helpers.mjs`，`matched_symbols=1`，回填 `3` 个页面。
- `reference`：无 reference。

### spring-petclinic

- `init`：成功，`4` 页 / `2` 模块 / `123` 源文件，marker `4/4`。
- `symbol`：`217` 个，全部来自 `java`；以 `method:164`、`class:44` 为主。
- `query`：用 `findById` 验证，精确命中 `OwnerRepository.java`，`matched_symbols=1`，回填 `3` 个页面。
- `reference`：无 reference。

### storybook

- `init`：成功，`67` 页 / `66` 模块 / `5471` 源文件，marker `67/67`。
- `symbol`：`6723` 个，`6158` 个 exported；以 `typescript:6422`、`javascript:301` 为主。
- `query`：用 `addStats` 验证，精确命中 `code/core/src/core-server/utils/summarizeStats.ts`，`matched_symbols=1`，回填 `4` 个页面。
- `reference`：有 reference，`176` 页；当前依然是大型 monorepo 的层级模块页。项目集分析还证明测试脚本必须给 heavy actions 更长超时，否则会被 `init` 耗时误伤。

### wot-starter

- `init`：成功，`7` 页 / `6` 模块 / `283` 源文件，marker `7/7`。
- `symbol`：`127` 个，主要来自 `typescript:109`、`javascript:11`、`python:7`；当前 `.vue` 脚本块仍未纳入 symbol parsing，因此结果集中在 composables、工具脚本和辅助代码。
- `query`：用 `useTheme` 验证，精确命中 `src/composables/useTheme.ts`，`matched_symbols=1`，回填 `3` 个页面。
- `reference`：无 reference。

### zustand

- `init`：成功，`3` 页 / `2` 模块 / `186` 源文件，marker `3/3`。
- `symbol`：`188` 个，语言分布为 `typescript:183`、`javascript:5`；以 `function:160`、`interface:16` 为主。
- `query`：用 `external` 验证，精确命中 `rollup.config.mjs` 中同名定义，`matched_symbols=1`，回填 `2` 个页面。
- `reference`：有 reference，`97` 页；当前仍是小型库骨架页，差异来自 reference 的概念与 API 深挖。

## 收口

- 当前实现已经满足“独立 symbol parsing 层 + `symbols/symbols_fts` 真写盘 + `query` 显式返回 `matched_symbols`”这一迭代 7 最小边界。
- 真实项目集验证证明 `Kotlin / Java / Rust / Go / Python / TypeScript / JavaScript / C# / C / C++ / PHP` 均已在项目集或夹具中得到有效解析；`Vue/Svelte` 仍然按设计 fail-soft，并留到迭代 8。
- 生命周期阶段脚本已经拆分并完成全量验证，因此 `5.4` 已收口；后续验证可直接复用 `bootstrap / steady / mutation / rebuild` 四个 phase 或对应薄包装脚本。
