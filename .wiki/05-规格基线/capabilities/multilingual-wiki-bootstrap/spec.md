---
title: Multilingual Wiki Bootstrap Specification
description: SpecWiki Lite 双语模板、bootstrap readiness 和迁移合同
updated: 2026-07-29
owner: product
---

# Multilingual Wiki Bootstrap

## Requirement: 配置选择 Wiki 语言

- **WHEN** 项目没有 `.wiki/config.yaml`
- **THEN** init MUST 创建 version 1 配置并使用中文 `zh`
- **WHEN** init 指定 `--language en`
- **THEN** 系统 MUST 生成英文目录、文件名和正文并持久化 `en`
- **WHEN** 配置非法
- **THEN** 系统 MUST 在写 Wiki 前失败

## Requirement: bootstrap 是显式状态

- **WHEN** 根 `INDEX.md` 仍含 package bootstrap marker
- **THEN** Wiki 静态结构 MAY ready，但项目级 ready MUST 为 false
- **WHEN** Codex 用正式首页替换任务页
- **THEN** status MUST 报告 `bootstrapPending: false`

## Requirement: 语言迁移保护用户内容

- **WHEN** 来源 scaffold/managed 仍等于 package 模板
- **THEN** update MAY 将登记资产迁移到目标语言
- **WHEN** scaffold 被修改、目标冲突或路径逃逸
- **THEN** 迁移 MUST 失败关闭且不得留下部分写入
- **WHEN** 修改过的旧英文 scaffold 已由用户显式配置为 `en`
- **THEN** update MUST 将旧路径作为用户内容保留，并安装不冲突的当前英文资产
- **WHEN** 页面未登记
- **THEN** 系统 MUST 保留且不得自动翻译
