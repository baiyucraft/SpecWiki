## Why

`iteration-12-4` 与 `iteration-12-5` 已经把 `v0.2.0` 的正式合同推进到 `minimal formal knowledge runtime` 与 6 个公开 workflow，但当前仓库的 README、中文文档、release note、distribution contract、版本号和 staged 产物说明仍存在明显漂移。结果是 spec、源码、文档和发布面彼此打架，仓库虽然已经接近 `v0.2.0` 能力边界，却还不能作为口径一致的可发布版本。

这轮 change 不重新定义 `v0.2.0`，只收口已有合同的 public truth sources，消除当前 release drift。

## What Changes

- 统一 `README.md`、`README-CN.md`、release note、主包帮助文本与 staged README 的 `v0.2.0` 口径
- 对齐当前正式 public workflow surface 为 `init / status / update / query / sync / rebuild`
- 对齐当前正式 runtime contract 为 `minimal formal knowledge runtime`，移除残留的 `v0.1.0 index-only` 叙述
- 收口 `repo-wiki-workflow` 与 `adapter-distribution` 的版本/发布表述，避免 spec 标题、正文与 distribution contract 继续漂移
- 明确本轮只做 release drift closure，不新增 runtime 能力、不扩 public surface

## Capabilities

### New Capabilities
- None

### Modified Capabilities
- `repo-wiki-workflow`: 收口正式发布叙述与 public workflow contract 的文档/标题漂移
- `adapter-distribution`: 收口 staged package 与发布分发合同的版本口径与产物说明

## Impact

- `README.md`
- `README-CN.md`
- `RELEASE-v0.1.0.md` 或其替代的 `v0.2.0` release 文档
- `dist/spec-wiki/README.md`
- `packages/spec-wiki/package.json`
- `.wiki/05-规格基线/capabilities/repo-wiki-workflow/spec.md`
- `.wiki/05-规格基线/capabilities/adapter-distribution/spec.md`
