## 1. OpenSpec 与文档

- [x] 1.1 更新 adapter-distribution change spec，明确发布目录改为 `dist/spec-wiki`，主包内直接携带 Windows runtime
- [x] 1.2 更新 README 中的 build / publish / runtime 路径说明，移除 `dist/npm/**` 与平台包描述

## 2. 构建与发布脚本

- [x] 2.1 在 workspace `Cargo.toml` 增加 `profile.release`，固定 `opt-level="s"`、`lto="thin"`、`codegen-units=1`、`strip="symbols"`、`panic="abort"`
- [x] 2.2 改造 `scripts/build-dist.mjs`，使用 release binary，并将待发布目录唯一收口到 `dist/spec-wiki`
- [x] 2.3 改造 `scripts/publish-packages.mjs`，只发布 `dist/spec-wiki`

## 3. 运行时解析与测试

- [x] 3.1 改造 `resolveBinary()`，优先解析包内 `lib/x64-win32/wiki-runtime.exe`，再回退工作区 release binary
- [x] 3.2 更新 distribution / resolveBinary 相关测试，验证单包结构和 runtime 路径
- [x] 3.3 执行相关 JS/Rust/build 测试，确认单包发布与 index-only 命令面可用
