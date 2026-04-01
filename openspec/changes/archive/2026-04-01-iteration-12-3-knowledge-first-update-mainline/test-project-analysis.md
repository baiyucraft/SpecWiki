# Full Init Baseline Guard

日期：2026-03-31

命令：

```text
node scripts/run-test-projects.mjs --no-build
```

## 结论

- 该结果是 baseline guard，不是本轮 primary gate。
- 当前 full init 项目集结果为：`19 total / 16 passed / 3 failed`。
- `16` 个项目进入脚本接受的诊断成功态，主状态均为 `runtime_incomplete`。
- `3` 个项目直接异常退出，未形成可诊断 `.wiki` 产物，需要单独排查。

## Passed Projects

- `aLocal`
- `axum`
- `bat`
- `chi`
- `cobra`
- `dagger`
- `django-ninja`
- `docker-mailserver`
- `gin`
- `httpx`
- `pinia`
- `restaurant-app`
- `spring-petclinic`
- `storybook`
- `wot-starter`
- `zustand`

## Failed Projects

- `fastapi`
  - 现象：`wiki-runtime exited with code 4294967295`
  - 复跑结果：稳定复现
  - 产物状态：`.wiki/`、cache DB、`wiki.metadata.json` 均未生成
- `leakcanary`
  - 现象：`wiki-runtime exited with code 4294967295`
  - 复跑结果：稳定复现
  - 产物状态：`.wiki/`、cache DB、`wiki.metadata.json` 均未生成
- `spec-wiki`
  - 现象：`wiki-runtime exited with code 4294967295`
  - 复跑结果：稳定复现
  - 产物状态：`.wiki/`、cache DB、`wiki.metadata.json` 均未生成

## Runtime Pattern

- 大多数项目当前不是 `fresh`，而是脚本接受的 `runtime_incomplete`。
- 常见 incomplete reason：
  - `workflow 仍在 research，尚未进入 compose`
  - `缺少 wiki.metadata.json`
  - `已有 knowledge/research 数据，但 assemble 尚未写出 metadata 与 Markdown`
- 这说明本轮状态机修正没有把项目集 baseline 打回 crash，但 provider-backed init 仍普遍停在 assemble 前。

## 与本 change 的关系

- `12.3` 当前已验证：
  - runtime baseline 的 `status/query/update/restore` 没有被本轮改坏
  - full init guard 仍能稳定给出诊断态结果，而不是统一崩溃
- `12.3` 当前仍未验证完成：
  - `AffectedKnowledgeScope` 驱动的 scoped research / compose / assemble
  - `storybook + dagger` 的真正 update-focused parent propagation / projection refresh 报告

## 下一步

- 优先继续推进 `2.1 / 3.1 / 3.2`，让 runtime 从“research 诊断态”进入可观察的 scoped update 主线。
- `fastapi / leakcanary / spec-wiki` 保持单独排查，不把它们混入本轮 primary gate。
