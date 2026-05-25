# full init baseline guard scope note (2026-04-02)

## Scope

- change: `iteration-12-4-query-route-and-v0-2-release-gate`
- gate target: `3.3`
- objective: 记录完整 `run-test-projects` baseline guard 已被移出本轮强制验收，只保留为后续可选参考项

## Scope Decision

- 用户已明确本轮不需要再跑 `19` 项目。
- 因此 `run-test-projects.mjs` 不再作为 `12.4` 的强制 release gate。
- 它后续如果执行，也只用于补充广覆盖观察面，不影响本轮 contract 收口是否完成。

## Interpretation

- 本报告现在只承担范围说明，不再承担“本轮必须补完”的任务。
- 这不会改变 primary gate 与 smoke gate 的定义：
  - primary gate 仍是 `storybook`
  - smoke gate 仍是 `chi + zustand`
- 如果后续需要更广覆盖观察，再单独开一轮 baseline guard 即可。

## Task Impact

- `3.3` 可以标记为 completed。
