## Context

`iteration-12-4` 已将 `v0.2.0` 的正式发布语义推进到 `minimal formal knowledge runtime`，`iteration-12-5` 又已将 `sync / rebuild` 升级为正式 public surface。当前问题不在 runtime 能力不足，而在 public truth sources 仍然彼此漂移：

- UniSpec 主 spec 已按 `v0.2.0` 与 6 个公开动作叙述
- TypeScript 源码 public surface 已公开 6 个动作
- 英文 README 已偏向 `v0.2.0`
- 中文 README、release note、staged README、distribution spec 与 package version 仍残留 `v0.1.0 index-only`

因此这轮必须把“同一版本的公开真相源”一次性收齐，而不是继续逐文件零散修补。

## Goals / Non-Goals

**Goals:**
- 收口 `README.md`、`README-CN.md`、release note、`dist/spec-wiki/README.md`、package version 与主 spec 的 `v0.2.0` 口径
- 消除当前 public workflow surface 的 4 动作 / 6 动作漂移
- 让 `repo-wiki-workflow` 与 `adapter-distribution` 的 requirement 表述与当前版本事实一致
- 明确当前正式发布边界是 `minimal formal knowledge runtime`，不是 `v0.1.0 index-only`

**Non-Goals:**
- 不新增 runtime capability
- 不重做 packaging 流程或 publish 流程
- 不引入 `0.3.0` 的 declared knowledge / knowledge governance 目标
- 不修改 lifecycle、query payload 或宿主协议语义

## Decisions

### 1. 本轮只做 truth source closure，不重开 `v0.2.0` 立项
`12-4` 与 `12-5` 已经定义了 `v0.2.0` 的发布合同；本轮只能做 gap closure，不再以“重新定义 release surface”为目标。这样可以避免 proposal、spec 与已归档 change 再次重叠。

### 2. public truth sources 必须一次性收齐
以下对象在本轮必须被当成同一批 truth source 统一收口：

- `README.md`
- `README-CN.md`
- release note
- `dist/spec-wiki/README.md`
- `packages/spec-wiki/package.json`
- `.wiki/05-规格基线/capabilities/repo-wiki-workflow/spec.md`
- `.wiki/05-规格基线/capabilities/adapter-distribution/spec.md`

如果继续分轮逐个修改，仓库会持续处于“源码、文档、staged 产物互相矛盾”的状态。

### 3. 版本叙事与能力叙事必须同步修改
本轮不只改“文案中的动作列表”，还必须同步收口：

- 版本号和 release label
- runtime contract 是 `knowledge runtime` 还是 `index-only`
- staged package 的安装边界
- `dist/` 说明是否反映当前源码真相

否则会出现“6 动作已经公开，但 release note 仍声称非公开”的半同步状态。

### 4. 优先以正式 spec 与源码真相为锚点
当 README、release note、staged README 和旧 distribution contract 互相冲突时，本轮以当前正式主 spec 与源码 public surface 为主锚点，再回写所有其他文档与包元数据，而不是反过来让旧 release note继续主导版本叙事。

## Risks / Trade-offs

- [风险] 一次性改动所有 truth source 容易遗漏某个旧文档
  → Mitigation：先用 drift matrix 列清对象，再按清单逐项对齐并复核

- [风险] `repo-wiki-workflow` 与 `adapter-distribution` 仍可能存在标题/正文漂移
  → Mitigation：在 spec delta 中显式处理 requirement rename 或 wording closure，而不是只改正文

- [风险] package version 变更但 staged 产物说明未同步，导致新的发布漂移
  → Mitigation：将 package version、staged README 和 release note 视为同一任务批次收口
