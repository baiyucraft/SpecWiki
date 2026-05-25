## ADDED Requirements

### Requirement: dossier 必须支持 family-scoped 输入与非源码 surface
系统 MUST 让 `RepoDossier`、`FamilyDossier`、`ModuleDossier` 和 `TopicDossier` 同时支持源码片段与非源码 surface。dossier 除现有 `targeted_snippets` 外，还 MUST 支持 `docs_anchors`、`public_api_surfaces`、`config_surfaces`、`type_surfaces`、`child_page_results` 和 `family_scoped_evidence`，并允许这些对象进入 research 与 compose 主链。

#### Scenario: family 页构建包含 docs/API/config 的 dossier
- **WHEN** 系统为 family index 或 family child 页构建 dossier
- **THEN** dossier MUST 能同时包含 docs anchors、public API、config surface 和相关一手源码材料
- **THEN** 这些输入 MUST 进入稳定 identity 与 input hash

#### Scenario: 非源码 surface 不得退化为路径字符串清单
- **WHEN** dossier 包含 docs/API/config/type surface
- **THEN** 每个 surface 条目 MUST 至少带稳定标识、来源路径和摘要/锚点信息
- **THEN** 系统不得只把这些对象压平成路径列表再交给 research

### Requirement: 叶子页面的 dossier 必须优先消费一手材料
系统 MUST 对叶子 family 页、叶子模块页和高置信主题页优先提供一手材料输入，而不是优先提供父页摘要。叶子页 dossier MUST 优先包含完整或定点的一手源码、类型/API surface、配置入口和 docs anchors；父页 dossier 再优先消费这些叶子页的结构化结果。

#### Scenario: 叶子 family child 页优先拿到一手材料
- **WHEN** 某个 family child 被识别为叶子页
- **THEN** 其 dossier MUST 优先包含一手源码片段、API/config/docs anchors
- **THEN** 系统不得先只给它父页摘要再让模型反推具体实现

#### Scenario: 父页 dossier 优先消费子页结果
- **WHEN** family index、overview、architecture 或父模块页存在已生成的子页结果
- **THEN** 父页 dossier MUST 优先吸收这些子页的结构化结果
- **THEN** 父页不得重复重扫相同的一手材料作为主要输入
### Requirement: dossier 必须支持面向 leaf 单元的拆分输入
系统 MUST 允许 family child 下的 docs/API/config/type surface 继续被拆成 leaf 单元，并让这些 leaf 单元成为独立 dossier/research 的输入对象。系统不得把所有 surface 永远压回一个 `FamilyDossier` 再让单页吸收。

#### Scenario: docs/API/config/type surface 被拆成 leaf dossier 输入
- **WHEN** 某个 family child 下存在多个稳定 surface 子簇
- **THEN** 系统 MUST 为这些子簇构造对应的 leaf research 输入
- **THEN** 叶子输入 MUST 保留稳定 identity、关键来源和摘要，而不是只保留路径字符串
