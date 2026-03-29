## ADDED Requirements

### Requirement: compact query 的 symbol hit 必须提供可直接定位的行号提示
系统 MUST 为默认 compact query payload 中的 `symbol` hit 提供最小行号提示。若底层 facts 中存在 symbol 定义行号，`symbol` hit MUST 至少向外暴露起始行，并 SHOULD 同时暴露结束行。

#### Scenario: symbol hit 输出 path line hint
- **WHEN** query 命中了某个 symbol 且 facts 中存在该 symbol 的起始行
- **THEN** compact `symbol` hit 的 `location` MUST 使用 `path:line` 形式
- **THEN** 结果 SHOULD 同时包含 `line_start`
- **THEN** 若存在结束行，结果 SHOULD 同时包含 `line_end`

#### Scenario: 旧数据或空行号回退为纯 path
- **WHEN** 当前 symbol 命中缺少有效行号
- **THEN** compact `symbol` hit MUST 回退为纯 `path`
- **THEN** 系统 MUST NOT 输出无意义的 `:0`
