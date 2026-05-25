## ADDED Requirements

### Requirement: 宿主必须消费稳定的最小 answer surface
系统 MUST 为宿主与 Agent 暴露稳定的最小 answer surface，而不是让不同宿主各自定义 answer 壳。该 surface MUST 至少包含 `answer_mode`、`answer_trust`、`recommended_action`、`provenance` 与 `supporting_refs`，并允许附带最终 answer 内容或等价文本字段。

#### Scenario: 宿主直接消费 answer surface
- **WHEN** 宿主收到一次 formal answer 响应
- **THEN** 宿主 MUST 能直接读取 `answer_mode`、`answer_trust`、`recommended_action`、`provenance` 与 `supporting_refs`
- **THEN** 宿主 MUST 不需要依赖内部 runtime 私有对象才能理解这次 answer

#### Scenario: Agent 消费同一套 answer surface
- **WHEN** Agent 通过同一宿主桥接消费 answer
- **THEN** 系统 MUST 继续返回同一套最小 answer surface
- **THEN** 系统 MUST 不得为不同调用方平行发明两套 answer contract
