## 1. Contract

- [x] 1.1 为 `wiki-steering-config` 增加 delta spec，收紧 `.wiki/config.yaml`、user/repo/dev 优先级与显式开发模式边界
- [x] 1.2 检查 `proposal.md` / `design.md` / delta spec 的边界一致性，确保本 change 不扩张到 `wiki-index/query`、CLI forwarding 或宿主资产生成

## 2. Harness

- [x] 2.1 收口 `scripts/run-init-debug-trace.mjs`，改为复用 `scripts/testing/helpers.mjs` 的 `withTemporaryDevConfig`
- [x] 2.2 核对 `scripts/testing/helpers.mjs` 的注释和参数语义，确保 Windows 超时回收与临时 dev config 回滚语义清晰

## 3. Verification

- [x] 3.1 运行与本 change 直接相关的脚本 / 测试验证，确认调试脚本和共享 helper 没有再保留双份 dev config 逻辑
- [x] 3.2 检查涉及脚本注释是否符合 `COMMENTING.md`
