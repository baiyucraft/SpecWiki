# dagger 专项报告

## 执行范围
- `node scripts/run-test-projects.mjs storybook dagger`
- `node scripts/test-wiki-lifecycle.mjs dagger --phase full --run-mode warm --jobs 1 --no-build --timeout-minutes 180`

## 观察结论
- `run-test-projects` 与 lifecycle `full/warm` 均通过，`write_facts_snapshot` 出现在 `knowledge_planning` 之前。
- `full/warm` 下 `init -> status -> sync -> query -> update -> touch/update -> rebuild -> status` 全链路通过，`status` 在 init 与 rebuild 后都保持 `fresh`。
- 产物与查询口径稳定：`symbols=17054`，`edges=75773`，`communities=982`，`processes=8`。
- no-op update 与 scoped update 后，`symbol / edge / communities / processes` 计数都保持稳定；graph query 与 provenance 也未回退。

## 对 iteration-11 的判断
- 已验证：`dagger` 上 module/source/symbol/edge snapshot 与 index-first query 全链路成立。
- 已验证：此前 scoped update 会把 `communities / processes` 清成 `0 / 0` 的断点已修复，rebuild 不再是唯一恢复路径。
