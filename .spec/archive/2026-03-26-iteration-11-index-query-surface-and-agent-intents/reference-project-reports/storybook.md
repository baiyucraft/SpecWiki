# storybook 专项报告

## 执行范围
- `node scripts/run-test-projects.mjs storybook dagger`
- `node scripts/test-wiki-lifecycle.mjs storybook --phase bootstrap --run-mode warm --jobs 1 --no-build --timeout-minutes 180`
- `node scripts/test-wiki-lifecycle.mjs storybook --phase mutation --run-mode warm --jobs 1 --no-build --timeout-minutes 180`
- `node scripts/test-wiki-lifecycle.mjs storybook --phase full --run-mode warm --jobs 1 --no-build --timeout-minutes 180`

## 观察结论
- `run-test-projects` 与 lifecycle `full/warm` 均通过，`write_facts_snapshot` 明确出现在 `build_module_tree` 之后、`build_contexts / knowledge_planning` 之前。
- `full/warm` 下 `init -> status -> sync -> query -> update -> touch/update -> rebuild -> status` 全链路通过，`status` 在 init 与 rebuild 后都保持 `fresh`。
- 冷/暖产物口径稳定：`pages=226`，`symbols=6790`，`edges=26064`，`communities=794`，`processes=8`。
- scoped update 后 `communities / processes` 不再掉到 `0 / 0`；symbol query 与 graph query 持续命中，说明 index-first 查询与 graph-derived analysis 在复杂仓库上都保持稳定。

## 对 iteration-11 的判断
- 已验证：facts snapshot 提前提交后，`storybook` 的 `modules / symbols / edges / communities / processes` 在完整 lifecycle 中持续可用。
- 已验证：query 不再因 downstream incomplete 或 scoped update 回退到“无 facts / 无 graph-derived context”的旧行为。
