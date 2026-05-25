## ADDED Requirements

### Requirement: 发布包不得继续携带与真实 bootstrap 漂移的占位 skill 模板
正式发布的 `spec-wiki` 主包 MUST NOT 再包含仅用于早期模板实验、且未被当前 bootstrap 逻辑消费的占位 skill Markdown 模板。若 skill 内容已经由共享 renderer 动态生成，发布包 MUST 只携带真实会被消费的资产，而不是额外保留一套带占位 frontmatter 的平行文案源。

#### Scenario: staging 产物不再包含占位 skill 模板
- **WHEN** 开发者执行 staging / distribution 流程
- **THEN** 生成的主包 MUST NOT 包含未被 bootstrap 使用的 `assets/skills/*.md` 占位模板
- **THEN** distribution tests MUST 以新的真实产物边界为准
