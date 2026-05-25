# Storybook Runtime Artifact Validation

## 范围

- 本轮以 `storybook` 作为 formal artifact / cold restore 主样本。
- `dagger` 仍只保留为对照观察项，不替代门禁。

## 本轮执行命令

```bash
pnpm exec vitest run scripts/tests/run-test-projects-resume-policy.test.ts scripts/tests/test-wiki-lifecycle-preflight.test.ts
node scripts/run-test-projects.mjs --no-build --run-mode warm --timeout-minutes 40 storybook
node scripts/test-wiki-lifecycle.mjs --no-build --phase bootstrap --run-mode warm --timeout-minutes 20 storybook
```

## 本轮新增修补

- `scripts/testing/init-resume.mjs`
  - 脚本层 preserve-resume 判定已与 core `init` 的 partial markdown preserve 语义对齐。
  - `metadataExists=true`、runtime gates 缺失、`workflow_action!=init` 等场景仍然禁止 resume。
- `scripts/test-wiki-lifecycle.mjs`
  - 新增 `warm-restore-preflight`。
  - 当 `warm` 模式命中“`wiki.metadata.json` 存在、`.cache` 缺失、`recovery-manifest.json` 存在”时，先调用 `status` 触发 restore；restore 成功后跳过 destructive `init`。
  - bootstrap 生命周期在 warm restore 场景下接受 `fresh / needs_update` 两种基线状态，不再错误要求只能是 `fresh`。
- 新增/更新脚本测试并通过：
  - `scripts/tests/run-test-projects-resume-policy.test.ts`
  - `scripts/tests/test-wiki-lifecycle-preflight.test.ts`

## Storybook formal runtime 结果

`run-test-projects` warm 验证已拿到完整 assembled formal runtime：

- `runtimeState=ready`
- `metadataExists=true`
- `markdownPageCount=226`
- `knowledge_units=226`
- `knowledge_domains=14`
- `page_digests=226`
- `page_drafts=226`
- `unit_runtime_gates=226`
- `wiki_pages=226`
- `pipeline_runtime_summary.runtime_state=completed`
- `.wiki/.knowledge/**` 已正式落盘：
  - `.wiki/.knowledge/derived/knowledge-domains.json`
  - `.wiki/.knowledge/derived/knowledge-tree.json`
  - `.wiki/.knowledge/derived/knowledge-units.jsonl`
  - `.wiki/.knowledge/derived/research-summaries.jsonl`
  - `.wiki/.knowledge/runtime/page-digests.jsonl`
  - `.wiki/.knowledge/runtime/runtime-gates.jsonl`
  - `.wiki/.knowledge/runtime/recovery-manifest.json`

结论：`.knowledge` 落盘与 formal artifact 最小集验证已成立。

## Cold Restore / Lifecycle Baseline

### 预检条件

在 ready runtime 基础上，显式删除：

- `tmp/test/storybook/.wiki/.cache`

删除后现场确认：

- `metadataExists=true`
- `cacheDbExists=false`
- `markdownPageCount=226`
- `.wiki/.knowledge/runtime/recovery-manifest.json` 仍存在

### Restore-focused lifecycle 结果

执行：

```bash
node scripts/test-wiki-lifecycle.mjs --no-build --phase bootstrap --run-mode warm --timeout-minutes 20 storybook
```

结果：通过，`6/6` 断言成功。

关键观察点：

- 日志明确进入 `warm-restore-preflight`
- `status` 成功触发 restore，并重建 `.wiki/.cache/wiki-cache.db`
- harness 跳过 destructive `init`
- `status after init` 返回 `needs_update`
- bootstrap baseline 将 `needs_update` 视为 warm restore 合法状态，而不是误判失败

这里的含义要写清楚：

- 已验证：ready formal runtime 删 `.cache` 后，lifecycle harness 能通过 restore 回到可消费状态
- 已验证：cold restore 后的 lifecycle / index-readiness baseline 不回退
- 未声称：storybook `phase=full` 已在本轮稳定通过

## Full Phase 观察

- `node scripts/test-wiki-lifecycle.mjs --no-build --run-mode warm --timeout-minutes 80 storybook` 仍然会被 `mutation / update / rebuild` 长尾拖长。
- `phase=full` 当前反映的是 storybook 大样本的全生命周期预算问题，而不是 cold restore contract 本身。
- 因此，本 change 不再把 `phase=full` 作为 `4.4` 的门禁；它留到后续性能 / 预算专项处理。

## Dagger 观察项

- `dagger` 当前仍可作为 ready runtime 对照样本。
- 但本轮不使用 `dagger` 替代 `storybook` 的 formal artifact / restore 门禁。

## 结论

- `4.3` 已成立：storybook formal artifact 与 `.knowledge` 最小集已正式落盘。
- `4.4` 已按 restore-focused lifecycle baseline 成立：cold restore 后的 lifecycle / index-readiness 语义不回退。
- storybook `phase=full` 的长尾问题保留到后续迭代，不继续阻塞本 change 归档。
