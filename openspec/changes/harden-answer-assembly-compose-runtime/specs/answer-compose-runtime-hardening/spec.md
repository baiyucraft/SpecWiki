## ADDED Requirements

### Requirement: Answer runtime MUST rely on formal substrate and MUST NOT silently fall back to page text
系统 MUST 让 answer assembly 优先消费正式的 declared / derived / projection substrate。若 formal substrate 缺失、漂移或不可信，系统 MUST 显式进入 `degraded` 或 `refuse`；系统 MUST NOT 静默回退到 page fallback 或临时 cache 继续伪装为正常 answer。

#### Scenario: query 命中对象缺少稳定 formal substrate
- **WHEN** answer assembly 无法获得稳定的 declared / derived / projection substrate
- **THEN** 系统 MUST 返回显式 `degraded` 或 `refuse`
- **THEN** 系统 MUST NOT 以未标注的 page fallback 结果冒充正常 formal answer

### Requirement: Answer runtime MUST preserve supporting refs, trust, and provenance across restore and provider-backed compose
系统 MUST 在 cold restore、rebuild、provider-backed compose 与长流程 query 后保持 supporting refs、trust 与 provenance 的一致性。若这些字段无法稳定保留，系统 MUST 显式降级，而不是继续输出看似完整的 answer。

#### Scenario: restore 后 answer 再次命中同一 knowledge target
- **WHEN** runtime 在 restore 或 rebuild 后再次为同一 target 组装 answer
- **THEN** supporting refs、trust 与 provenance MUST 与 formal substrate 保持一致
- **THEN** 若一致性无法成立，系统 MUST 返回显式 degraded 或 refuse

### Requirement: Capability-specific assertions MUST reuse the existing quality gate surface
本 change 的验证 MUST 复用 `12-9-7` 已 formalize 的 gate surface 与 summary contract，只增加 answer / compose runtime 的 capability-specific assertions。系统 MUST NOT 新增 answer-specific gate transport、summary shape 或平行 primary gate。

#### Scenario: 团队为 answer hardening 增加验收脚本
- **WHEN** 团队为 answer / compose runtime 增加新的断言或 evidence
- **THEN** 这些断言 MUST 挂接到既有 quality gate surface 上
- **THEN** 系统 MUST NOT 引入第二套 answer 专用 gate transport 或 summary 协议
