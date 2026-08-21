---
title: Agents 设计
description: Codex-only 双语 Skills、模板、阶段路由和资产边界
updated: 2026-08-03
owner: architecture
---

# Agents 设计

SpecWiki Lite 只支持 Codex。八个稳定英文标识的 Skills 安装到 `.agents/skills`：

```text
wiki-continue
wiki-explore
wiki-propose
wiki-design
wiki-plan
wiki-apply
wiki-review
wiki-archive
```

`wiki-continue` 以 `status/show/strict validate` 为权威：有 active change 时按 stage、artifact、parent/child order 与 dependencies 路由；没有 active change但 `wiki.bootstrapPending` 为 true 时路由 `wiki-explore`。其余 Skills 分别负责 exploration、proposal、design、plan、apply、review 与 archive，不允许调度入口直接推进 stage 或签发证据。

实现模式由 `wiki-plan` 在生成 tasks 前询问一次，可选 `tdd` 或 `direct`；当前请求已明确模式时不重复询问。tasks 记录稳定字段 `implementation-mode`。`tdd` 采用 Red → Green → Refactor，`direct` 采用 Implement → Verify → Refactor；合法 mode 的 tasks 直接路由 `wiki-apply`，缺失或非法 mode 回到 `wiki-plan`。两种模式都保留用户授权、strict validate、测试、review、verification 和 archive 门禁。

`.wiki/config.yaml` 的 `wiki.language` 同时选择 Wiki 与 Skill 正文，默认 `zh`，可选 `en`。Skill id、目录、reference 文件名、CLI、artifact 文件名和 metadata 字段不翻译。package 每种语言登记 24 个 Skill 文件：8 个 `SKILL.md` 与 16 个 references。

```text
packages/spec-wiki-lite/assets/skills/
├── zh/wiki-*/{SKILL.md,references/**}
└── en/wiki-*/{SKILL.md,references/**}

.agents/skills/wiki-*/
├── SKILL.md
└── references/**
```

references 分配如下：explore 提供 research；propose 提供 research/proposal；design 提供 research/design；plan 提供 system-tests、unit-tests、tasks 和工具中立的 browser automation；review 提供两份正式报告模板及通用、frontend、Go、Java、Python review standards。continue/apply/archive 只增强主 Skill。

Skill 登记文件属于 package-owned：普通 `init/update` 即覆盖缺失、旧版本、用户修改或语言不一致的登记文件；用户新增的未登记文件保留。`status.skills[].installed` 只有在该 Skill 的目标语言 `SKILL.md` 与全部 references 都存在且逐字匹配 package 时才为 true。

Skills 可以按需读取源码与 Wiki；项目存在 `.codegraph/` 时优先使用 CodeGraph MCP/CLI 做定位、调用关系和影响分析，工具不可用时退回普通源码读取。CodeGraph 仅是外部只读分析工具，Skills 不得建立持久化代码索引或第二套 metadata schema。浏览器自动化只是一种可选证据方式，不要求特定 runner，也不由 Lite 提供浏览器环境。

资产真相位于 `packages/spec-wiki-lite/assets/skills/{zh,en}/**`。`init/update` 通过同一原子同步器写入 repo-local `.agents/skills`，不生成 `.codex`、hooks、settings 或其他宿主投影。

参考来源为本机 `E:/project/!byAI/UniSpec` 的 `@uni-sw/unispec@0.1.0`。目标落点是上述 Lite 阶段约束、artifact templates、browser automation 与 review standards；采用方式是基于 Lite 产品合同的结构与行为改写，不直接复制源码、产品命名、runtime、多宿主或其他宿主投影内容。
