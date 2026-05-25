## Why

当前迭代 1 已经具备可用的结构 baseline，但还存在两个会直接影响可读性和可验证性的尾项：模块页“关键源码”仍会混入日志、锁文件、低价值配置等噪声文件，跨模块关系也仍然偏保守，导致模块页和 query 的结构信息不够有说服力。

现在发起这一 change，是为了在不进入 `WikiState` 或 LLM 迭代的前提下，把迭代 1 的输出质量再收紧一轮，并引入固定的 `aLocal -> tmp -> reference-zh` 对照流程，避免 tasks 和实现脱离真实产物差异。

## What Changes

- 收紧模块页 `关键源码` 的信号选择规则，过滤日志、锁文件、纯文档和低价值配置噪声。
- 补强前后端混合仓库和基础设施目录下的跨模块关系提取，让模块页与 query 更稳定表达依赖和被依赖关系。
- 收紧模块页摘要生成规则，使“模块角色 / 关键源码 / 依赖模块 / 被依赖模块”围绕高信号 facts 组织，而不是机械拼接。
- 保持 `query` 的结构优先路径，但让命中原因与 provenance 更贴合新的关系与关键源码信号。
- 把 `E:\\project\\aLocal` 的 `init` 产物快照纳入 baseline 验收流程，要求输出落入 `tmp/` 并与 [tmp/reference-zh](E:/project/!byAI/spec-wiki/tmp/reference-zh) 对照后再回调任务或实现。

## Capabilities

### New Capabilities
- 无

### Modified Capabilities
- `repo-hierarchy-model`: 收紧跨模块关系提取要求，使其在混合前后端与基础设施仓库中保留更稳定的结构依赖线索。
- `repo-wiki-runtime`: 收紧模块页与 metadata 的关键源码表达要求，使其必须抑制低信号文件噪声。
- `repo-wiki-workflow`: 收紧 `init` 和 `query` 的 baseline 输出要求，使其必须围绕高信号模块事实组织页面和命中说明。
- `workflow-verification`: 扩展 baseline 验收要求，使其包含 `aLocal` 产物快照写入 `tmp/` 并与 `tmp/reference-zh` 的对照步骤。

## Impact

- 受影响代码集中在 [crates/wiki-core/src/repo](E:/project/!byAI/spec-wiki/crates/wiki-core/src/repo)、[crates/wiki-core/src/generation](E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation)、[crates/wiki-core/src/workflows](E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows) 和 [crates/wiki-core/tests](E:/project/!byAI/spec-wiki/crates/wiki-core/tests)。
- `.wiki` 页面数量和页面层级不追求继续扩张，但模块页摘要、关系表达和 query 结构结果会更稳定。
- 验收流程会新增一个围绕 `E:\\project\\aLocal` 的快照对照步骤，产物放入 [tmp](E:/project/!byAI/spec-wiki/tmp) 下做人工和测试双重核验。
