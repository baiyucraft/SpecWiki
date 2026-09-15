---
title: External Cognition Bootstrap Capability
description: SpecWiki Lite 0.2.0 的 CodeGraph/AOCI 安装、readiness、数据库条件和安全合同
updated: 2026-09-15
owner: product
---

# External Cognition Bootstrap Capability

## Requirements

1. `init` 默认准备固定兼容版本 CodeGraph `1.6.0` 与 AOCI `0.1.0-rc12`；`--no-codegraph` / `--no-aoci` 只延后一次并保持 not ready。
2. `update --tools` 才修复或升级外部工具；普通 `update` 不产生工具副作用。
3. `status` 使用 CodeGraph 官方 JSON 与 AOCI `verify` / `check` / Guide，不以目录存在冒充健康。
4. 每个项目要求 CodeGraph 与 AOCI Code Cognition；只有声明数据库 source 后才要求 Database Cognition。
5. AOCI 归档先校验 SHA-256，安全提取并验证版本后原子发布；所有命令使用 argv、`shell:false`、timeout 和有界输出。
6. Lite 不拥有上游索引/状态机，不读取业务行或输出凭据，不把外部 runtime/local state 打包。
7. Skill 将 AOCI 长期语义证据与 CodeGraph 当前结构证据分别写入既有 `.spec` artifact，并与源码/测试交叉核验。

## Readiness

顶层 ready 同时要求 Wiki、Skills、active changes、CodeGraph 版本/MCP/索引健康，以及 AOCI 版本/init/治理对齐。声明数据库 source 后还要求 access 与 Database Cognition 对齐。未满足返回稳定 `nextActions` 和 exit `2`。
