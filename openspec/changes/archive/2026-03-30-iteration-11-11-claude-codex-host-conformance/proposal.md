## Status

该 change 目录是早期重复草稿，已经由 `iteration-11-11-codex-claude-host-conformance` 取代。

当前 Claude / Codex 宿主的正式真相以替代 change 为准：

- Claude 与 Codex 都走 repo 内 action skill
- Claude 不再保留 command
- Codex 不再保留 prompt
- 两者都只显式暴露 `init / status / query / update`
- `query` 只薄消费 runtime 的稳定结构化字段

这个重复目录不再作为有效规范来源。
