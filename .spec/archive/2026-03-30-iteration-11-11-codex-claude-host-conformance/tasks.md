## 1. UniSpec

- [x] 1.1 为 `global-cli-bootstrap` 补充 Claude / Codex 的 `v0.1.0` 显式暴露动作与 query thin-consumption 要求
- [x] 1.2 修正 `global-cli-bootstrap` 中过时的宿主路径表述与双 skill 真相

## 2. 宿主资产

- [x] 2.1 收敛 Claude / Codex 的显式 action 集，只保留 `init / status / query / update`
- [x] 2.2 删除 Claude command 与 Codex prompt 形态，并把旧受管残留纳入清理范围
- [x] 2.3 更新 Claude / Codex skill 正文，使其与 CodeBuddy action skill 使用同一 section 骨架
- [x] 2.4 同步更新静态模板资产，避免发布包里的参考模板继续漂移

## 3. 测试与注释

- [x] 3.1 补充和更新测试，验证 Claude / Codex 的 skill 落点、正文骨架、query 边界与旧资产清理
- [x] 3.2 检查本轮新增和修改注释，确保符合 `.wiki/02-开发指南/00-代码注释规范.md`
