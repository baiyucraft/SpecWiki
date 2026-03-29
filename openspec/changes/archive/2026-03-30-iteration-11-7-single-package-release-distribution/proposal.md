## Why

当前 `spec-wiki` 的分发模型仍是 `dist/npm/spec-wiki` + 平台包双包发布，这与当前发布目标直接冲突：开发者希望 `pnpm run build` 后只得到一个可直接 publish 的 `dist/spec-wiki` 包，用户 `npm i -g spec-wiki` 后即可全局使用。

同时，当前 staging 仍默认收 debug `wiki-runtime`，导致单平台二进制体积与发布真相都失真。

## What Changes

- workspace 增加统一的 `profile.release`，以体积优先但不过度牺牲性能的配置收敛 runtime 发布产物
- `scripts/build-dist.mjs` 改为产出 release binary，并把待发布包唯一收口到 `dist/spec-wiki`
- Windows 单平台 runtime 随主包一起发布，路径固定为 `dist/spec-wiki/lib/x64-win32/wiki-runtime.exe`
- `scripts/publish-packages.mjs` 改为只发布 `dist/spec-wiki`
- `resolveBinary()` 与 distribution 测试、README 同步切到单包路径语义

## Impact

- 发布目录真相从 `dist/npm/**` 收口到 `dist/spec-wiki/**`
- 当前版本只继续正式支持 Windows 单包发布，不保留平台 npm 子包
- `spec-wiki` 主包 publish 后可直接被 `npm i -g spec-wiki` 安装并使用
