# 测试项目集分析 — Iteration 8

## 运行概况

- `cargo test -p wiki-core`：通过，包含这轮补上的 Kotlin/C# symbol raw capture 回归测试。
- `node scripts/run-test-projects.mjs leakcanary dagger restaurant-app`：3/3 项目重新 `init` 成功，先前回归样本已用修复后的 release binary 重建。
- `node scripts/collect-test-project-analysis.mjs`：已基于修复后的 `.wiki/` 与 SQLite runtime 重新采集，并继续与 `tmp/iteration-7-test-project-analysis.json` 做逐项目对比。
- 项目集总页面数：226，managed marker 覆盖 `226/226`，较迭代 7 的 215 增加 `11`。
- 项目集总 symbols：51,686，较迭代 7 的 51,300 变化 `+386`；当前另有 `146,997` 条 edges、`1,242` 个 communities、`144` 个 processes。
- 18/19 项目写入了非空 symbols，18/19 项目写入了非空 graph；graph query 在这 18 个项目里都能命中至少一种扩展结果。

## 本轮验证结论

- iter 8 的 `symbol-resolution` / `symbol-graph-analysis` 已经真正落到项目集：与迭代 7 只验证 symbol 定义层不同，这次已经能逐项目看到 edges / communities / processes 和 graph-aware query provenance。
- 之前项目集分析里暴露的 3 个语言回归已经修掉：`leakcanary`、`dagger`、`restaurant-app` 的 Kotlin/C# symbols 都恢复到了迭代 7 基线，同时 graph 计数也重新回到有效区间。
- 页面层没有被 graph 接入破坏。总页数从 `215` 增到 `226`，总 symbols 也从 `51,300` 增到 `51,686`；正向增长最明显的是 spec-wiki(+162)、wot-starter(+128)、storybook(+57)、aLocal(+52)。
- 当前最小的 symbol 负向差异只剩 bat(-32)、axum(0)、chi(0)，其中只有 `bat` 的 `-32` 还在观察范围内，已经不是整语言失效那一类问题。
- 报告生成链路本身也修了一处真实问题：SQLite group-by 结果此前被单值读取函数截断为末行，现已改为多行读取，项目级 language / label breakdown 已恢复可信。

## 项目逐项分析

### aLocal

- `init`：成功，`7` 页 / `5` 模块 / `112` 源文件，marker `7/7`；较迭代 7 页面 `+1`。
- `symbol`：`269` 个，`166` 个 exported，较迭代 7 变化 `+52`；当前语言分布为 python:182、typescript:87，主标签为 function:236、interface:15、method:14。
- `graph`：迭代 7 未写入 graph；当前写入 `597` 条 edges / `6` 个 communities / `8` 个 processes。用 `getLog` 验证，可扩展到 `10` 条图边、`2` 个社区。
- `query`：本轮用 `get_html` 验证，精确命中 `spider/modules/utils.py`，`matched_pages` 从迭代 7 的 `4` 变为 `6`。
- `reference`：有 reference，`86` 页；当前差距 `-79`，较迭代 7 的 `-80` 收敛 1 页。

### axum

- `init`：成功，`7` 页 / `5` 模块 / `440` 源文件，marker `7/7`；较迭代 7 页面 `+1`。
- `symbol`：`3,021` 个，`2,297` 个 exported，较迭代 7 持平；当前语言分布为 rust:3,021，主标签为 function:1,750、struct:363、impl:278。
- `graph`：迭代 7 未写入 graph；当前写入 `8,058` 条 edges / `72` 个 communities / `8` 个 processes。用 `as_mut` 验证，可扩展到 `77` 条图边、`4` 个社区。
- `query`：本轮用 `echo_app` 验证，精确命中 `axum/src/extract/ws.rs`，`matched_pages` 从迭代 7 的 `6` 变为 `7`。
- `reference`：有 reference，`111` 页；当前差距 `-104`，较迭代 7 的 `-105` 收敛 1 页。

### bat

- `init`：成功，`14` 页 / `14` 模块 / `904` 源文件，marker `14/14`；较迭代 7 页面 持平。
- `symbol`：`1,971` 个，`1,758` 个 exported，较迭代 7 变化 `-32`；当前语言分布为 rust:1,043、python:793、javascript:94，主标签为 function:1,543、class:129、impl:73。 对比迭代 7，已不见 kotlin:25、csharp:9。
- `graph`：迭代 7 未写入 graph；当前写入 `5,001` 条 edges / `52` 个 communities / `8` 个 processes。用 `detect` 验证，可扩展到 `346` 条图边、`1` 个社区。
- `query`：本轮用 `assert_c` 验证，精确命中 `tests/benchmarks/highlighting-speed-src/numpy_test_multiarray.py`，`matched_pages` 从迭代 7 的 `13` 变为 `14`。
- `reference`：有 reference，`79` 页；当前差距 `-65`，较迭代 7 的 `-65` 持平。

### chi

- `init`：成功，`5` 页 / `3` 模块 / `157` 源文件，marker `5/5`；较迭代 7 页面 持平。
- `symbol`：`458` 个，`320` 个 exported，较迭代 7 持平；当前语言分布为 go:458，主标签为 function:225、method:127、type:58。
- `graph`：迭代 7 未写入 graph；当前写入 `1,358` 条 edges / `4` 个 communities / `8` 个 processes。用 `paginate` 验证，可扩展到 `1` 条图边、`1` 个社区。
- `query`：本轮用 `paginate` 验证，精确命中 `_examples/rest/main.go`，`matched_pages` 从迭代 7 的 `3` 变为 `4`。
- `reference`：有 reference，`65` 页；当前差距 `-60`，较迭代 7 的 `-60` 持平。

### cobra

- `init`：成功，`4` 页 / `2` 模块 / `101` 源文件，marker `4/4`；较迭代 7 页面 持平。
- `symbol`：`625` 个，`482` 个 exported，较迭代 7 持平；当前语言分布为 go:625，主标签为 function:420、method:168、type:22。
- `graph`：迭代 7 未写入 graph；当前写入 `2,301` 条 edges / `1` 个 communities / `8` 个 processes。用 `genZshComp` 验证，可扩展到 `13` 条图边、`1` 个社区。
- `query`：本轮用 `findFlag` 验证，精确命中 `completions.go`，`matched_pages` 从迭代 7 的 `2` 变为 `3`。
- `reference`：有 reference，`38` 页；当前差距 `-34`，较迭代 7 的 `-34` 持平。

### dagger

- `init`：成功，`28` 页 / `26` 模块 / `3,449` 源文件，marker `28/28`；较迭代 7 页面 `+1`。
- `symbol`：`16,995` 个，`11,770` 个 exported，较迭代 7 持平；当前语言分布为 java:13,032、kotlin:3,951、python:12，主标签为 method:8,829、class:2,778、function:1,596。 这轮先前暴露的 Kotlin/C# 覆盖回归已经恢复到迭代 7 基线。
- `graph`：迭代 7 未写入 graph；当前写入 `75,683` 条 edges / `453` 个 communities / `8` 个 processes。用 `addAll` 验证，可扩展到 `117` 条图边、`1` 个社区。
- `query`：本轮用 `factoryOf` 验证，精确命中 `dagger-compiler/main/java/dagger/internal/codegen/xprocessing/XTypeNames.kt`，`matched_pages` 从迭代 7 的 `4` 变为 `7`。
- `reference`：有 reference，`65` 页；当前差距 `-37`，较迭代 7 的 `-38` 收敛 1 页。

### django-ninja

- `init`：成功，`5` 页 / `4` 模块 / `296` 源文件，marker `5/5`；较迭代 7 页面 持平。
- `symbol`：`1,821` 个，`1,648` 个 exported，较迭代 7 持平；当前语言分布为 python:1,821，主标签为 function:1,387、class:434。
- `graph`：迭代 7 未写入 graph；当前写入 `2,849` 条 edges / `31` 个 communities / `8` 个 processes。用 `filter` 验证，可扩展到 `882` 条图边、`1` 个社区。
- `query`：本轮用 `clean_db` 验证，精确命中 `tests/test_pagination_cursor.py`，`matched_pages` 从迭代 7 的 `2` 变为 `3`。
- `reference`：无 reference。

### docker-mailserver

- `init`：成功，`4` 页 / `2` 模块 / `252` 源文件，marker `4/4`；较迭代 7 页面 持平。
- `symbol`：`0` 个，`0` 个 exported，较迭代 7 持平；当前语言分布为 无，主标签为 无。
- `graph`：迭代 7 没有 graph 持久化；当前项目仍未生成可用 graph 结果。
- `query`：当前没有可用代表性 symbol；但页面、metadata 与 runtime 仍能正常生成。
- `reference`：无 reference。

### fastapi

- `init`：成功，`7` 页 / `5` 模块 / `2,889` 源文件，marker `7/7`；较迭代 7 页面 `+1`。
- `symbol`：`5,267` 个，`5,053` 个 exported，较迭代 7 持平；当前语言分布为 python:5,242、javascript:25，主标签为 function:4,565、class:689、method:13。
- `graph`：迭代 7 未写入 graph；当前写入 `4,869` 条 edges / `94` 个 communities / `8` 个 processes。用 `get_db` 验证，可扩展到 `127` 条图边、`11` 个社区。
- `query`：本轮用 `add_task` 验证，精确命中 `fastapi/background.py`，`matched_pages` 从迭代 7 的 `5` 变为 `6`。
- `reference`：无 reference。

### gin

- `init`：成功，`7` 页 / `5` 模块 / `114` 源文件，marker `7/7`；较迭代 7 页面 持平。
- `symbol`：`1,659` 个，`1,262` 个 exported，较迭代 7 持平；当前语言分布为 go:1,659，主标签为 function:882、method:402、type:205。
- `graph`：迭代 7 未写入 graph；当前写入 `3,243` 条 edges / `22` 个 communities / `8` 个 processes。用 `mapURI` 验证，可扩展到 `3` 条图边、`1` 个社区。
- `query`：本轮用 `function` 验证，精确命中 `recovery.go`，`matched_pages` 从迭代 7 的 `2` 变为 `5`。
- `reference`：无 reference。

### httpx

- `init`：成功，`5` 页 / `3` 模块 / `115` 源文件，marker `5/5`；较迭代 7 页面 `+1`。
- `symbol`：`1,241` 个，`1,033` 个 exported，较迭代 7 持平；当前语言分布为 python:1,241，主标签为 function:1,134、class:107。
- `graph`：迭代 7 未写入 graph；当前写入 `2,370` 条 edges / `19` 个 communities / `8` 个 processes。用 `netloc` 验证，可扩展到 `3` 条图边、`1` 个社区。
- `query`：本轮用 `compress` 验证，精确命中 `tests/test_decoders.py`，`matched_pages` 从迭代 7 的 `2` 变为 `5`。
- `reference`：无 reference。

### leakcanary

- `init`：成功，`10` 页 / `8` 模块 / `747` 源文件，marker `10/10`；较迭代 7 页面 `+1`。
- `symbol`：`7,714` 个，`470` 个 exported，较迭代 7 持平；当前语言分布为 kotlin:7,683、java:31，主标签为 property:4,696、function:2,039、class:587。 这轮先前暴露的 Kotlin/C# 覆盖回归已经恢复到迭代 7 基线。
- `graph`：迭代 7 未写入 graph；当前写入 `6,890` 条 edges / `241` 个 communities / `8` 个 processes。用 `minRun` 验证，可扩展到 `5` 条图边、`1` 个流程、`1` 个社区。
- `query`：本轮用 `openFile` 验证，精确命中 `leakcanary/leakcanary-android-core/src/main/java/leakcanary/internal/LeakCanaryFileProvider.kt`，`matched_pages` 从迭代 7 的 `3` 变为 `3`。
- `reference`：无 reference。

### pinia

- `init`：成功，`16` 页 / `14` 模块 / `376` 源文件，marker `16/16`；较迭代 7 页面 `+1`。
- `symbol`：`417` 个，`229` 个 exported，较迭代 7 变化 `+19`；当前语言分布为 typescript:397、javascript:20，主标签为 function:208、method:161、interface:46。
- `graph`：迭代 7 未写入 graph；当前写入 `1,525` 条 edges / `13` 个 communities / `8` 个 processes。用 `toYAML` 验证，可扩展到 `2` 条图边、`1` 个流程、`1` 个社区。
- `query`：本轮用 `copyFile` 验证，精确命中 `packages/online-playground/vite.config.ts`，`matched_pages` 从迭代 7 的 `4` 变为 `4`。
- `reference`：有 reference，`61` 页；当前差距 `-45`，较迭代 7 的 `-46` 收敛 1 页。

### restaurant-app

- `init`：成功，`15` 页 / `13` 模块 / `1,095` 源文件，marker `15/15`；较迭代 7 页面 持平。
- `symbol`：`1,756` 个，`1,706` 个 exported，较迭代 7 持平；当前语言分布为 csharp:1,119、go:248、typescript:244，主标签为 method:534、property:310、class:230。 这轮先前暴露的 Kotlin/C# 覆盖回归已经恢复到迭代 7 基线。
- `graph`：迭代 7 未写入 graph；当前写入 `896` 条 edges / `103` 个 communities / `8` 个 processes。用 `rocket` 验证，可扩展到 `1` 条图边、`1` 个社区。
- `query`：本轮用 `get_dish` 验证，精确命中 `src/backend/services/catalog-api/src/seeder/seed.rs`，`matched_pages` 从迭代 7 的 `8` 变为 `14`。
- `reference`：有 reference，`127` 页；当前差距 `-112`，较迭代 7 的 `-112` 持平。

### spec-wiki

- `init`：成功，`8` 页 / `6` 模块 / `163` 源文件，marker `8/8`；较迭代 7 页面 `+1`。
- `symbol`：`1,032` 个，`831` 个 exported，较迭代 7 变化 `+162`；当前语言分布为 rust:934、javascript:86、typescript:12，主标签为 function:775、struct:106、module:71。
- `graph`：迭代 7 未写入 graph；当前写入 `4,333` 条 edges / `8` 个 communities / `8` 个 processes。用 `sample` 验证，可扩展到 `11` 条图边、`1` 个流程、`1` 个社区。
- `query`：本轮用 `callCore` 验证，精确命中 `scripts/testing/helpers.mjs`，`matched_pages` 从迭代 7 的 `3` 变为 `3`。
- `reference`：无 reference。

### spring-petclinic

- `init`：成功，`4` 页 / `2` 模块 / `123` 源文件，marker `4/4`；较迭代 7 页面 持平。
- `symbol`：`217` 个，`161` 个 exported，较迭代 7 持平；当前语言分布为 java:217，主标签为 method:164、class:44、constructor:6。
- `graph`：迭代 7 未写入 graph；当前写入 `367` 条 edges / `6` 个 communities / `8` 个 processes。用 `findPet` 验证，可扩展到 `9` 条图边、`1` 个社区。
- `query`：本轮用 `findById` 验证，精确命中 `src/main/java/org/springframework/samples/petclinic/owner/OwnerRepository.java`，`matched_pages` 从迭代 7 的 `3` 变为 `3`。
- `reference`：无 reference。

### storybook

- `init`：成功，`68` 页 / `66` 模块 / `5,471` 源文件，marker `68/68`；较迭代 7 页面 `+1`。
- `symbol`：`6,780` 个，`6,185` 个 exported，较迭代 7 变化 `+57`；当前语言分布为 typescript:6,451、javascript:329，主标签为 function:4,000、method:1,451、interface:1,001。
- `graph`：迭代 7 未写入 graph；当前写入 `26,064` 条 edges / `89` 个 communities / `8` 个 processes。用 `delete` 验证，可扩展到 `42` 条图边、`1` 个社区。
- `query`：本轮用 `addStats` 验证，精确命中 `code/core/src/core-server/utils/summarizeStats.ts`，`matched_pages` 从迭代 7 的 `4` 变为 `12`。
- `reference`：有 reference，`176` 页；当前差距 `-108`，较迭代 7 的 `-109` 收敛 1 页。

### wot-starter

- `init`：成功，`8` 页 / `6` 模块 / `283` 源文件，marker `8/8`；较迭代 7 页面 `+1`。
- `symbol`：`255` 个，`154` 个 exported，较迭代 7 变化 `+128`；当前语言分布为 typescript:204、javascript:44、python:7，主标签为 function:141、method:83、interface:26。
- `graph`：迭代 7 未写入 graph；当前写入 `215` 条 edges / `21` 个 communities / `8` 个 processes。用 `imgTap` 验证，可扩展到 `3` 条图边、`1` 个社区。
- `query`：本轮用 `callback` 验证，精确命中 `src/uni_modules/mp-html/components/mp-html/mp-html.vue`，`matched_pages` 从迭代 7 的 `3` 变为 `5`。
- `reference`：无 reference。

### zustand

- `init`：成功，`4` 页 / `2` 模块 / `186` 源文件，marker `4/4`；较迭代 7 页面 `+1`。
- `symbol`：`188` 个，`47` 个 exported，较迭代 7 持平；当前语言分布为 typescript:183、javascript:5，主标签为 function:160、interface:16、method:10。
- `graph`：迭代 7 未写入 graph；当前写入 `378` 条 edges / `7` 个 communities / `8` 个 processes。用 `reviver` 验证，可扩展到 `3` 条图边、`1` 个流程、`1` 个社区。
- `query`：本轮用 `external` 验证，精确命中 `rollup.config.mjs`，`matched_pages` 从迭代 7 的 `2` 变为 `4`。
- `reference`：有 reference，`97` 页；当前差距 `-93`，较迭代 7 的 `-94` 收敛 1 页。

## 收口

- iter 8 现在不只是“graph 能跑起来”，而是已经在项目集层面同时满足了 symbol 持久化、graph 分析、graph-aware query 和页面回填四条链路。
- 本轮修复后，之前最关键的 Kotlin/C# regressions 已经收口，因此当前项目集对 iter 8 的结论应当从“能力成立但覆盖率回退”更新为“能力成立且覆盖率总体不低于 iter 7”。
- 报告已同步写入根目录 `test-project-analysis.md` 和当前 change 目录 `openspec/changes/iteration-8-symbol-resolution-graph-search/test-project-analysis.md`。
