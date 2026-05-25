## MODIFIED Requirements

### Requirement: 正式 Wiki 页面必须稳定映射到模块结构
系统 MUST 让正式 Wiki 页面与模块结构保持稳定映射，以便后续 `query`、`sync` 和 `update` 可以围绕层级化页面工作。模块页的关键源码表达 MUST 优先选择入口、依赖证据和核心实现文件，并抑制日志、锁文件、纯文档和低价值配置噪声。模块页的页面标识或路径必须由模块祖先链稳定导出，并记录基础 provenance。页面状态还 MUST 记录稳定 section 标识、section 级 hash 和 section 到源码/关系的映射，为迭代 3 的局部重生成提供内部边界。

#### Scenario: 写入模块页
- **WHEN** 系统为模块生成正式 Wiki 页面
- **THEN** 页面状态中必须记录对应模块标识
- **THEN** 页面来源中必须记录该页面依赖的源码集合
- **THEN** 同一模块在重复生成时必须使用稳定页面标识或路径

#### Scenario: 写入递归模块页
- **WHEN** 模块页对应多级嵌套模块
- **THEN** 系统必须为该页面写入稳定的父页面关系或祖先路径
- **THEN** 系统必须避免因 Windows 非法路径字符或同名模块导致落盘冲突

#### Scenario: 导出高信号关键源码
- **WHEN** 系统生成模块页摘要和 metadata 页面来源
- **THEN** 关键源码列表必须优先包含入口、依赖证据或核心实现文件
- **THEN** 关键源码列表不得被日志、锁文件、纯文档或低价值配置主导

#### Scenario: 重复生成页面时 section 身份稳定
- **WHEN** 同一页面在未发生对应 section 输入变化的情况下被重复生成
- **THEN** 页面中的 section 标识必须保持稳定
- **THEN** 未变化 section 的 hash 和 provenance 映射必须保持可复用

### Requirement: 运行时缓存必须与正式索引分层
系统 MUST 将运行时缓存写入 `.wiki/.cache/`，且缓存缺失时不应使正式 Wiki 页面或 metadata 失去有效性。WikiState 持久化文件 MUST 存放在 `.wiki/.cache/wiki-state.json`，与 scan cache 和 module tree cache 同层。为了支撑增量 runtime，`.wiki/.cache/` 还 MUST 提供 page context cache 和 page generation cache，并允许按页面精确失效。

#### Scenario: 缓存丢失但正式索引仍在
- **WHEN** `.wiki/.cache/` 被删除，但 `.wiki/*.md` 和 `wiki.metadata.json` 仍存在
- **THEN** 系统必须仍能执行 `status` 或 `query` 来理解正式索引
- **THEN** 系统必须将需要重建增量 runtime 的情况与正式索引缺失区分开

#### Scenario: WikiState 缓存布局
- **WHEN** init、update、sync 或 rebuild 完成后
- **THEN** `.wiki/.cache/` MUST 包含 `repo-scan.json`、`module-tree.json` 和 `wiki-state.json`
- **THEN** `.wiki/.cache/` MUST 包含 `page-contexts/` 和 `page-generation/` 两类每页缓存目录

#### Scenario: 局部更新只失效局部页面缓存
- **WHEN** 系统识别到受影响页面集合
- **THEN** 系统必须只失效这些页面对应的 page context / generation cache
- **THEN** 未受影响页面的 cache 文件不得被无条件重写
