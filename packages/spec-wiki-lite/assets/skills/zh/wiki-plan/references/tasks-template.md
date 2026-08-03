# <change-id> 实现任务

implementation-ready: false
implementation-mode: tdd

## Red

- [ ] R1 <新增测试；映射 UT/ST/成功标准>
- [ ] R2 运行 focused tests，记录相关 Red 失败。

## Green

- [ ] G1 <实现最小完整主链>
- [ ] G2 <实现失败关闭、ownership、path safety 或 rollback>
- [ ] G3 运行 focused tests，记录 Green。

## Refactor 与验证

- [ ] F1 <类型、错误、重复逻辑和文档收口>
- [ ] F2 运行 package/root tests、lint、typecheck、build、pack、Wiki 和 diff gates。
- [ ] F3 更新证据并准备 full review。

## 成功标准映射

| 成功标准 | ST/UT | Task | 命令/证据 |
| --- | --- | --- | --- |
| <criterion> | ST-01 / UT-01 | G1 | <command/path> |

> 只有用户已授权实现且所有 planning blocker 解决后，才把 `implementation-ready` 改为 true。
