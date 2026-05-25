## Status

该重复 spec 已由 `.spec/changes/iteration-11-11-codex-claude-host-conformance/specs/global-cli-bootstrap/spec.md` 取代。

当前有效真相：

- Claude 使用 `.claude/skills/wiki-*/SKILL.md`
- Codex 使用 `.codex/skills/wiki-*/SKILL.md`
- 两者都不再保留 command / prompt 形态
- 两者都只显式暴露 `init / status / query / update`
- `query` 只薄消费稳定结构化字段
