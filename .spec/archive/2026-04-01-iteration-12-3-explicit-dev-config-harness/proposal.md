## Why

`wiki-runtime` 当前已经把 repo 级共享配置、用户级默认配置和显式开发模式下的 `wiki.dev.yaml` 叠加逻辑落进实现里了，但仓库里还残留两类没有被单独收口的 follow-up：

- `wiki-steering-config` 主 spec 还在混用旧的 `.wiki/wiki.steering.yaml` 路径与“无条件读取 `wiki.dev.yaml`”的旧表述
- debug / lifecycle 侧脚本对临时 dev config 的处理还没有完全收口到共享 helper，容易继续漂移出第二套语义

如果不把这两点单独收口，后续很容易再次把它们和 `wiki-index/query`、`spec-wiki CLI/e2e` 或更大的 Agents/bootstrap 主题混到一起。

## What Changes

- 收紧 `wiki-steering-config` 合同，明确 repo 级共享配置路径是 `.wiki/config.yaml`
- 明确配置优先级为 `~/.spec-wiki/config.yaml < .wiki/config.yaml < wiki.dev.yaml`，且最右侧只在显式开发模式开启时生效
- 明确 `init` 对用户级配置模板和 YAML 校验的最小合同
- 收口调试脚本对临时 dev config 的消费方式，复用共享 helper，而不是在单脚本里再实现一份副本逻辑
- 本 change 不扩张到 `wiki-index/query`、`spec-wiki` CLI forwarding、workspace 包迁移或宿主资产边界

## Capabilities

### Modified Capabilities

- `wiki-steering-config`: 收紧 repo/user/dev 三层配置合同与显式开发模式边界

## Impact

- 重点影响 UniSpec 契约：
  - `.wiki/05-规格基线/capabilities/wiki-steering-config/spec.md`
- 重点影响脚本 / 验证：
  - `scripts/run-init-debug-trace.mjs`
  - `scripts/testing/helpers.mjs`
