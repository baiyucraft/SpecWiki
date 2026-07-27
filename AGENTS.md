# SpecWiki Lite Agent 入口

## 强约束

1. 使用中文沟通。
2. 需求或设计变化先更新 `.spec/changes/**`，再修改实现。
3. 当前处于测试开发阶段，不保留旧 package、命令、native core 或多宿主兼容层。
4. 参考 upstream 时必须标明来源、目标落点，以及直接迁移、改写或仅借鉴。
5. `.spec/archive/**` 是历史证据，除非用户明确要求，不批量改写。
6. 完成迭代时必须执行测试、lint、build、pack evidence、`git diff --check`，并归档 change。

## 产品边界

- package 与命令：`spec-wiki-lite@0.1.0` / `spec-wiki-lite`。
- 唯一宿主：Codex；Skills 只写入 `.agents/skills/wiki-*`。
- 正式内容：`.wiki/**/*.md`；导航入口为每级 `INDEX.md`。
- 工作流状态：`.spec/changes/**`，归档：`.spec/archive/**`。
- TypeScript core：`core/assets`、`core/wiki`、`core/change`、`core/path`。
- 不建立代码索引、知识图谱、映射数据库或隐藏 Wiki 中间层。

## 必读入口

| 事项 | 入口 |
| --- | --- |
| 总体设计 | [.wiki/06-设计文档/00-总体设计.md](./.wiki/06-设计文档/00-总体设计.md) |
| Lite 内核 | [.wiki/06-设计文档/01-Lite内核设计.md](./.wiki/06-设计文档/01-Lite内核设计.md) |
| Codex Skills | [.wiki/06-设计文档/02-Agents设计.md](./.wiki/06-设计文档/02-Agents设计.md) |
| CLI | [.wiki/04-对外方法/00-CLI.md](./.wiki/04-对外方法/00-CLI.md) |
| 测试与验收 | [.wiki/02-开发指南/01-测试与验收.md](./.wiki/02-开发指南/01-测试与验收.md) |
| 长期知识 | [.wiki/INDEX.md](./.wiki/INDEX.md) |

## 开工顺序

```text
AGENTS.md
  -> .spec/changes/**
  -> .wiki/INDEX.md
  -> 相关设计 / capability
  -> packages/spec-wiki-lite/src / tests
```

使用 `apply_patch` 做手工编辑。工作区可能包含用户改动，只提交当前任务拥有的文件。
