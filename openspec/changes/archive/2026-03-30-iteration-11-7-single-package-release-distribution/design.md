## Context

当前实现已经把正式源码包收口到 `packages/spec-wiki`，但发布形态仍保留双包平台模型：主包通过 `optionalDependencies` 指向平台包，`resolveBinary()` 也优先从平台包路径解析 runtime。这个设计与当前产品目标不一致。

当前阶段只正式支持 Windows runtime 分发，因此本轮不扩展多平台发布抽象，只把发布真相收口到 Windows 单包。

## Decisions

### 决策 1：发布目录唯一真相是 `dist/spec-wiki`

根级 build 后只保留一个可发布目录：

```text
dist/spec-wiki/
  package.json
  bin/spec-wiki.js
  dist/index.js
  assets/**
  lib/x64-win32/wiki-runtime.exe
```

### 决策 2：发布 binary 一律使用 release profile

Rust runtime 的发布产物必须来自 release profile，而不是 debug profile。

release profile 当前收敛为：
- `opt-level = "s"`
- `lto = "thin"`
- `codegen-units = 1`
- `strip = "symbols"`
- `panic = "abort"`

### 决策 3：当前只做 Windows 单包，不保留平台包残骸

- 不再生成平台包 manifest
- staged `package.json` 不再带 `optionalDependencies`
- `resolveBinary()` 优先解析包内 `lib/x64-win32/wiki-runtime.exe`

## Risks

- 如果只改脚本，不同步 README 和测试，会继续出现“构建真相 / 文档真相 / 运行真相”分裂
- 如果仍保留平台包候选路径，会让单包模式与旧模式并存，后续更难排错
