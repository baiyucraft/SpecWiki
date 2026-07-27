---
title: Staged Spec Workflow
description: TypeScript change stage、artifact、校验和原子归档合同
updated: 2026-07-28
owner: product
---

# Staged Spec Workflow

## Purpose

以文件系统中的可审计 artifact 驱动 proposal、design、tests、implementation、review、verification 和 archive。

## Requirements

### Requirement: 单一 stage evaluator

#### Scenario: 查看或校验 change

- **WHEN** 执行 `status`、`show` 或 `validate`
- **THEN** 三者消费同一 stage 闭集、artifact registry 和 blocking issue 规则

### Requirement: 验证证据必须完整

#### Scenario: 准备归档

- **WHEN** change 处于 verification
- **THEN** review 与 test 报告必须同时声明 `scope: full` 和 pass 结果

### Requirement: 归档不覆盖

#### Scenario: dated target 已存在

- **WHEN** 执行 `archive <change-id>`
- **THEN** 拒绝操作并保持 active change 与已有 target 不变

### Requirement: Multi-change 保持一致

#### Scenario: child 归档

- **WHEN** child 与 parent 声明一致且验证通过
- **THEN** child target、parent metadata 和 split marker 同步；任一步失败则全部回滚
