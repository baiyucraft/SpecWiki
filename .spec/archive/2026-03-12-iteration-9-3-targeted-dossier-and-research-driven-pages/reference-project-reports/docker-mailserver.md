# docker-mailserver Reference 对比报告

生成页面：5 页
reference 页面：79 页
命中对比：21 页
缺失对比：58 页
运行模式：cold (cache_mode=clear)
LLM usage：requests=16, total_tokens=159573, page_research=7, page_enrichment=3

## 覆盖统计

- 专题页覆盖：generated 1 / reference 1（repo-archetype=2）
- evidence 落页：generated 3 / reference 79
- citation 密度：generated 1.4 / reference 66.9
- 图表达覆盖：generated 0 / reference 79
- page research 请求：7
- page enrichment 请求：3
- 已规划专题类型：专题页(1)
- 高频缺失专题：无

## 项目结论

- reference 文档树明显更细，当前 planner 仍以总览页和少量模块页为主
- 存在大量 reference 页面没有对应生成页，说明页面粒度和主题拆分不足
- 生成页普遍缺少 reference 那种源码引用/出处层
- evidence block 已进入页面，但覆盖率和密度仍低于 reference
- Mermaid/结构图表达仍然不足
- facts-driven 图输入尚未稳定覆盖到代表性页面
- 单页章节拆分比 reference 粗，主题混杂在同一页里
- 解释层正文密度仍低于 reference

## 多页折叠现象

- 系统架构.md 被 12 个 reference 页面共享映射
- 工作流与部署.md 被 7 个 reference 页面共享映射

## 额外生成页面

- 专题/repo-archetype/ops-runtime-部署与环境.md (部署与环境, 17 行)

## 逐文件对比

| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Evidence(ref/gen) | Mermaid(ref/gen) | 主要结论 |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| API参考.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 安全配置/基础安全设置/DKIM、DMARC与SPF认证/DKIM、DMARC与SPF认证.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 安全配置/基础安全设置/DKIM、DMARC与SPF认证/DKIM配置详解.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 安全配置/基础安全设置/DKIM、DMARC与SPF认证/DMARC配置详解.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 安全配置/基础安全设置/DKIM、DMARC与SPF认证/SPF配置详解.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 安全配置/基础安全设置/MTA-STS邮件传输安全.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 安全配置/基础安全设置/SSL_TLS证书配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 安全配置/基础安全设置/基础安全设置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 安全配置/基础安全设置/端口配置与网络安全.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 安全配置/安全监控与审计.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 安全配置/安全配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 安全配置/高级安全功能/Fail2Ban入侵防护.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 安全配置/高级安全功能/Rspamd高级配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 安全配置/高级安全功能/邮件加密.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 安全配置/高级安全功能/高级安全功能.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/开发工具.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/开发者指南.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/构建系统.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/测试框架.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/贡献指南.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 快速开始.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 故障排除.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 服务配置/Dovecot IMAP_POP3 服务器配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 服务配置/Postfix SMTP 服务器配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 服务配置/内容过滤服务配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 服务配置/服务配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 服务配置/配置覆盖与自定义.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 架构设计/数据流分析.md | 系统架构.md | 268/39 | 143/14 | 0/2 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、compose.yaml |
| 架构设计/整体架构概述.md | 系统架构.md | 402/39 | 219/14 | 0/2 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、compose.yaml、introduction.md、usage.md |
| 架构设计/架构设计.md | 系统架构.md | 334/39 | 149/14 | 0/2 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、security.md、compose.yaml、fetchmail-compose.yaml、environment.md、fail2ban.md |
| 架构设计/核心服务组件/Amavis 内容过滤.md | 系统架构.md | 285/39 | 124/14 | 18/2 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：environment.md |
| 架构设计/核心服务组件/ClamAV 病毒扫描.md | 系统架构.md | 281/39 | 97/14 | 0/2 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：compose.yaml、kubernetes.md、environment.md、clamd.conf、freshclam.conf |
| 架构设计/核心服务组件/Dovecot IMAP_POP3 服务器.md | 系统架构.md | 312/39 | 111/14 | 18/2 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、overview.md、ldap.md、oauth2.md、mail-sieve.md、ssl.md、auth-lua.md |
| 架构设计/核心服务组件/Fail2Ban 安全防护.md | 系统架构.md | 288/39 | 86/14 | 0/2 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：fail2ban.md、custom.conf |
| 架构设计/核心服务组件/OpenDKIM_OpenDMARC 邮件签名.md | 系统架构.md | 226/39 | 77/14 | 0/2 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：changelog.md、readme.md、dkim_dmarc_spf.md、opendkim.conf、opendmarc.conf |
| 架构设计/核心服务组件/Postfix SMTP 服务器.md | 系统架构.md | 362/39 | 123/14 | 0/2 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：compose.yaml、ldap.md、postfix.md、opendkim.conf、opendmarc.conf、supervisord.conf |
| 架构设计/核心服务组件/Rspamd 垃圾邮件检测.md | 系统架构.md | 338/39 | 116/14 | 0/2 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：environment.md、rspamd.md、actions.conf、antivirus.conf、composites.conf、greylist.conf、hfilter_group.conf、neural.conf |
| 架构设计/核心服务组件/核心服务组件.md | 系统架构.md | 354/39 | 83/14 | 0/2 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、compose.yaml、kubernetes.md、relay-hosts.md、dkim_dmarc_spf.md、environment.md、rspamd.md |
| 架构设计/部署架构.md | 系统架构.md | 299/39 | 111/14 | 0/2 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、compose.yaml、fetchmail-compose.yaml、relay-compose.yaml、ipv6.md、kubernetes.md、optional-config.md、environment.md |
| 环境配置/SSL_TLS证书配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 环境配置/环境变量详解.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 环境配置/环境配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 环境配置/端口和服务配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 环境配置/网络配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 示例用例/基础教程.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 示例用例/示例用例.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 示例用例/高级用例/IMAP文件夹管理.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 示例用例/高级用例/LDAP认证集成.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 示例用例/高级用例/Lua认证机制.md | 专题/repo-archetype/config-runtime-配置与运行时.md | 226/17 | 136/4 | 10/0 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：ldap.md、dovecot.md、environment.md、auth-lua.md、auth-ldap.conf、auth-lua-httpbasic.conf |
| 示例用例/高级用例/SMTP网络接口绑定.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 示例用例/高级用例/iOS邮件推送支持.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 示例用例/高级用例/仅转发邮件服务器.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 示例用例/高级用例/外部中继服务器.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 示例用例/高级用例/高级用例.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 认证系统/LDAP认证.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 认证系统/OAuth2认证.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 认证系统/主账号功能.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 认证系统/文件认证.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 认证系统/认证系统.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 认证系统/认证集成与最佳实践.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 部署运维/Docker部署.md | 工作流与部署.md | 273/44 | 79/15 | 0/1 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、compose.yaml、fetchmail-compose.yaml、relay-compose.yaml、environment.md、basic-installation.md、external-relay-only-mailserver.md、forward-only-mailserver-with-ldap-authentication.md |
| 部署运维/Kubernetes部署.md | 工作流与部署.md | 182/44 | 64/15 | 0/1 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：bug_report.yml、readme.md、compose.yaml、kubernetes.md、environment.md、crowdsec.md、mailserver-behind-proxy.md、env.md |
| 部署运维/备份恢复.md | 工作流与部署.md | 334/44 | 149/15 | 0/1 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、compose.yaml、full-text-search.md、update-and-cleanup.md、mail_crypt.md、ssl.md、faq.md |
| 部署运维/监控日志.md | 工作流与部署.md | 248/44 | 80/15 | 0/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、compose.yaml、ignore.conf、maillog.conf、supervisord.conf、logrotate.conf、rsyslog.conf |
| 部署运维/维护更新.md | 工作流与部署.md | 314/44 | 115/15 | 0/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、security.md、compose.yaml、update-and-cleanup.md |
| 部署运维/部署运维.md | 工作流与部署.md | 357/44 | 89/15 | 17/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、compose.yaml、fetchmail-compose.yaml、relay-compose.yaml、kubernetes.md、podman.md、environment.md、setup.sh.md |
| 项目概述.md | 项目概述.md | 255/59 | 77/22 | 0/4 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、compose.yaml、dkim_dmarc_spf.md、fail2ban.md、rspamd.md、introduction.md、usage.md、custom-commands.conf |
| 高级功能/IPv6支持.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/Kubernetes部署.md | 工作流与部署.md | 212/44 | 92/15 | 0/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、compose.yaml、kubernetes.md、environment.md、mailserver-behind-proxy.md |
| 高级功能/Podman支持.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/全文搜索.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/可选配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/系统维护.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/邮件获取.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/邮件转发/AWS SES 集成.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/邮件转发/Gmail SMTP 配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/邮件转发/中继主机配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/邮件转发/邮件转发.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/高级功能.md | 缺失 | - | - | - | - | 缺少对应生成页面 |

## 逐文件详情

### API参考.md

- reference 标题：API参考
- 生成页：无
- 问题：缺少对应生成页面

### 安全配置/基础安全设置/DKIM、DMARC与SPF认证/DKIM、DMARC与SPF认证.md

- reference 标题：DKIM、DMARC与SPF认证
- 生成页：无
- 问题：缺少对应生成页面

### 安全配置/基础安全设置/DKIM、DMARC与SPF认证/DKIM配置详解.md

- reference 标题：DKIM配置详解
- 生成页：无
- 问题：缺少对应生成页面

### 安全配置/基础安全设置/DKIM、DMARC与SPF认证/DMARC配置详解.md

- reference 标题：DMARC配置详解
- 生成页：无
- 问题：缺少对应生成页面

### 安全配置/基础安全设置/DKIM、DMARC与SPF认证/SPF配置详解.md

- reference 标题：SPF配置详解
- 生成页：无
- 问题：缺少对应生成页面

### 安全配置/基础安全设置/MTA-STS邮件传输安全.md

- reference 标题：MTA-STS邮件传输安全
- 生成页：无
- 问题：缺少对应生成页面

### 安全配置/基础安全设置/SSL_TLS证书配置.md

- reference 标题：SSL/TLS证书配置
- 生成页：无
- 问题：缺少对应生成页面

### 安全配置/基础安全设置/基础安全设置.md

- reference 标题：基础安全设置
- 生成页：无
- 问题：缺少对应生成页面

### 安全配置/基础安全设置/端口配置与网络安全.md

- reference 标题：端口配置与网络安全
- 生成页：无
- 问题：缺少对应生成页面

### 安全配置/安全监控与审计.md

- reference 标题：安全监控与审计
- 生成页：无
- 问题：缺少对应生成页面

### 安全配置/安全配置.md

- reference 标题：安全配置
- 生成页：无
- 问题：缺少对应生成页面

### 安全配置/高级安全功能/Fail2Ban入侵防护.md

- reference 标题：Fail2Ban入侵防护
- 生成页：无
- 问题：缺少对应生成页面

### 安全配置/高级安全功能/Rspamd高级配置.md

- reference 标题：Rspamd高级配置
- 生成页：无
- 问题：缺少对应生成页面

### 安全配置/高级安全功能/邮件加密.md

- reference 标题：邮件加密
- 生成页：无
- 问题：缺少对应生成页面

### 安全配置/高级安全功能/高级安全功能.md

- reference 标题：高级安全功能
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/开发工具.md

- reference 标题：开发工具
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/开发者指南.md

- reference 标题：开发者指南
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/构建系统.md

- reference 标题：构建系统
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/测试框架.md

- reference 标题：测试框架
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/贡献指南.md

- reference 标题：贡献指南
- 生成页：无
- 问题：缺少对应生成页面

### 快速开始.md

- reference 标题：快速开始
- 生成页：无
- 问题：缺少对应生成页面

### 故障排除.md

- reference 标题：故障排除
- 生成页：无
- 问题：缺少对应生成页面

### 服务配置/Dovecot IMAP_POP3 服务器配置.md

- reference 标题：Dovecot IMAP/POP3 服务器配置
- 生成页：无
- 问题：缺少对应生成页面

### 服务配置/Postfix SMTP 服务器配置.md

- reference 标题：Postfix SMTP 服务器配置
- 生成页：无
- 问题：缺少对应生成页面

### 服务配置/内容过滤服务配置.md

- reference 标题：内容过滤服务配置
- 生成页：无
- 问题：缺少对应生成页面

### 服务配置/服务配置.md

- reference 标题：服务配置
- 生成页：无
- 问题：缺少对应生成页面

### 服务配置/配置覆盖与自定义.md

- reference 标题：配置覆盖与自定义
- 生成页：无
- 问题：缺少对应生成页面

### 架构设计/数据流分析.md

- reference 标题：数据流分析
- 生成页：系统架构.md（系统架构）
- 匹配分数：82
- 页面类型：architecture / architecture
- 行数：268 / 39
- 段落行数：143 / 14
- Evidence：0 / 2
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：readme.md、compose.yaml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、compose.yaml

### 架构设计/整体架构概述.md

- reference 标题：整体架构概述
- 生成页：系统架构.md（系统架构）
- 匹配分数：86
- 页面类型：architecture / architecture
- 行数：402 / 39
- 段落行数：219 / 14
- Evidence：0 / 2
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：readme.md、compose.yaml、introduction.md、usage.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、compose.yaml、introduction.md、usage.md

### 架构设计/架构设计.md

- reference 标题：架构设计
- 生成页：系统架构.md（系统架构）
- 匹配分数：84
- 页面类型：architecture / architecture
- 行数：334 / 39
- 段落行数：149 / 14
- Evidence：0 / 2
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：readme.md、security.md、compose.yaml、fetchmail-compose.yaml、environment.md、fail2ban.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、security.md、compose.yaml、fetchmail-compose.yaml、environment.md、fail2ban.md

### 架构设计/核心服务组件/Amavis 内容过滤.md

- reference 标题：Amavis 内容过滤
- 生成页：系统架构.md（系统架构）
- 匹配分数：84
- 页面类型：architecture / architecture
- 行数：285 / 39
- 段落行数：124 / 14
- Evidence：18 / 2
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：environment.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：environment.md

### 架构设计/核心服务组件/ClamAV 病毒扫描.md

- reference 标题：ClamAV 病毒扫描
- 生成页：系统架构.md（系统架构）
- 匹配分数：84
- 页面类型：architecture / architecture
- 行数：281 / 39
- 段落行数：97 / 14
- Evidence：0 / 2
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：compose.yaml、kubernetes.md、environment.md、clamd.conf、freshclam.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：compose.yaml、kubernetes.md、environment.md、clamd.conf、freshclam.conf

### 架构设计/核心服务组件/Dovecot IMAP_POP3 服务器.md

- reference 标题：Dovecot IMAP/POP3 服务器
- 生成页：系统架构.md（系统架构）
- 匹配分数：84
- 页面类型：architecture / architecture
- 行数：312 / 39
- 段落行数：111 / 14
- Evidence：18 / 2
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：readme.md、overview.md、ldap.md、oauth2.md、mail-sieve.md、ssl.md、auth-lua.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、overview.md、ldap.md、oauth2.md、mail-sieve.md、ssl.md、auth-lua.md

### 架构设计/核心服务组件/Fail2Ban 安全防护.md

- reference 标题：Fail2Ban 安全防护
- 生成页：系统架构.md（系统架构）
- 匹配分数：82
- 页面类型：architecture / architecture
- 行数：288 / 39
- 段落行数：86 / 14
- Evidence：0 / 2
- Mermaid：3 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：fail2ban.md、custom.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：fail2ban.md、custom.conf

### 架构设计/核心服务组件/OpenDKIM_OpenDMARC 邮件签名.md

- reference 标题：OpenDKIM/OpenDMARC 邮件签名
- 生成页：系统架构.md（系统架构）
- 匹配分数：84
- 页面类型：architecture / architecture
- 行数：226 / 39
- 段落行数：77 / 14
- Evidence：0 / 2
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：changelog.md、readme.md、dkim_dmarc_spf.md、opendkim.conf、opendmarc.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：changelog.md、readme.md、dkim_dmarc_spf.md、opendkim.conf、opendmarc.conf

### 架构设计/核心服务组件/Postfix SMTP 服务器.md

- reference 标题：Postfix SMTP 服务器
- 生成页：系统架构.md（系统架构）
- 匹配分数：84
- 页面类型：architecture / architecture
- 行数：362 / 39
- 段落行数：123 / 14
- Evidence：0 / 2
- Mermaid：4 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：compose.yaml、ldap.md、postfix.md、opendkim.conf、opendmarc.conf、supervisord.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：compose.yaml、ldap.md、postfix.md、opendkim.conf、opendmarc.conf、supervisord.conf

### 架构设计/核心服务组件/Rspamd 垃圾邮件检测.md

- reference 标题：Rspamd 垃圾邮件检测
- 生成页：系统架构.md（系统架构）
- 匹配分数：84
- 页面类型：architecture / architecture
- 行数：338 / 39
- 段落行数：116 / 14
- Evidence：0 / 2
- Mermaid：4 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：environment.md、rspamd.md、actions.conf、antivirus.conf、composites.conf、greylist.conf、hfilter_group.conf、neural.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：environment.md、rspamd.md、actions.conf、antivirus.conf、composites.conf、greylist.conf、hfilter_group.conf、neural.conf

### 架构设计/核心服务组件/核心服务组件.md

- reference 标题：核心服务组件
- 生成页：系统架构.md（系统架构）
- 匹配分数：82
- 页面类型：architecture / architecture
- 行数：354 / 39
- 段落行数：83 / 14
- Evidence：0 / 2
- Mermaid：3 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：readme.md、compose.yaml、kubernetes.md、relay-hosts.md、dkim_dmarc_spf.md、environment.md、rspamd.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、compose.yaml、kubernetes.md、relay-hosts.md、dkim_dmarc_spf.md、environment.md、rspamd.md

### 架构设计/部署架构.md

- reference 标题：部署架构
- 生成页：系统架构.md（系统架构）
- 匹配分数：82
- 页面类型：architecture / architecture
- 行数：299 / 39
- 段落行数：111 / 14
- Evidence：0 / 2
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：readme.md、compose.yaml、fetchmail-compose.yaml、relay-compose.yaml、ipv6.md、kubernetes.md、optional-config.md、environment.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、compose.yaml、fetchmail-compose.yaml、relay-compose.yaml、ipv6.md、kubernetes.md、optional-config.md、environment.md

### 环境配置/SSL_TLS证书配置.md

- reference 标题：SSL/TLS证书配置
- 生成页：无
- 问题：缺少对应生成页面

### 环境配置/环境变量详解.md

- reference 标题：环境变量详解
- 生成页：无
- 问题：缺少对应生成页面

### 环境配置/环境配置.md

- reference 标题：环境配置
- 生成页：无
- 问题：缺少对应生成页面

### 环境配置/端口和服务配置.md

- reference 标题：端口和服务配置
- 生成页：无
- 问题：缺少对应生成页面

### 环境配置/网络配置.md

- reference 标题：网络配置
- 生成页：无
- 问题：缺少对应生成页面

### 示例用例/基础教程.md

- reference 标题：基础教程
- 生成页：无
- 问题：缺少对应生成页面

### 示例用例/示例用例.md

- reference 标题：示例用例
- 生成页：无
- 问题：缺少对应生成页面

### 示例用例/高级用例/IMAP文件夹管理.md

- reference 标题：IMAP文件夹管理
- 生成页：无
- 问题：缺少对应生成页面

### 示例用例/高级用例/LDAP认证集成.md

- reference 标题：LDAP认证集成
- 生成页：无
- 问题：缺少对应生成页面

### 示例用例/高级用例/Lua认证机制.md

- reference 标题：Lua认证机制
- 生成页：专题/repo-archetype/config-runtime-配置与运行时.md（配置与运行时）
- 匹配分数：84
- 页面类型：topic / topic
- 行数：226 / 17
- 段落行数：136 / 4
- Evidence：10 / 0
- Mermaid：4 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：ldap.md、dovecot.md、environment.md、auth-lua.md、auth-ldap.conf、auth-lua-httpbasic.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：ldap.md、dovecot.md、environment.md、auth-lua.md、auth-ldap.conf、auth-lua-httpbasic.conf

### 示例用例/高级用例/SMTP网络接口绑定.md

- reference 标题：SMTP网络接口绑定
- 生成页：无
- 问题：缺少对应生成页面

### 示例用例/高级用例/iOS邮件推送支持.md

- reference 标题：iOS邮件推送支持
- 生成页：无
- 问题：缺少对应生成页面

### 示例用例/高级用例/仅转发邮件服务器.md

- reference 标题：仅转发邮件服务器
- 生成页：无
- 问题：缺少对应生成页面

### 示例用例/高级用例/外部中继服务器.md

- reference 标题：外部中继服务器
- 生成页：无
- 问题：缺少对应生成页面

### 示例用例/高级用例/高级用例.md

- reference 标题：高级用例
- 生成页：无
- 问题：缺少对应生成页面

### 认证系统/LDAP认证.md

- reference 标题：LDAP认证
- 生成页：无
- 问题：缺少对应生成页面

### 认证系统/OAuth2认证.md

- reference 标题：OAuth2认证
- 生成页：无
- 问题：缺少对应生成页面

### 认证系统/主账号功能.md

- reference 标题：主账号功能
- 生成页：无
- 问题：缺少对应生成页面

### 认证系统/文件认证.md

- reference 标题：文件认证
- 生成页：无
- 问题：缺少对应生成页面

### 认证系统/认证系统.md

- reference 标题：认证系统
- 生成页：无
- 问题：缺少对应生成页面

### 认证系统/认证集成与最佳实践.md

- reference 标题：认证集成与最佳实践
- 生成页：无
- 问题：缺少对应生成页面

### 部署运维/Docker部署.md

- reference 标题：Docker部署
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：86
- 页面类型：workflow / workflow
- 行数：273 / 44
- 段落行数：79 / 15
- Evidence：0 / 1
- Mermaid：3 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：readme.md、compose.yaml、fetchmail-compose.yaml、relay-compose.yaml、environment.md、basic-installation.md、external-relay-only-mailserver.md、forward-only-mailserver-with-ldap-authentication.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、compose.yaml、fetchmail-compose.yaml、relay-compose.yaml、environment.md、basic-installation.md、external-relay-only-mailserver.md、forward-only-mailserver-with-ldap-authentication.md

### 部署运维/Kubernetes部署.md

- reference 标题：Kubernetes部署
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：86
- 页面类型：workflow / workflow
- 行数：182 / 44
- 段落行数：64 / 15
- Evidence：0 / 1
- Mermaid：3 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：bug_report.yml、readme.md、compose.yaml、kubernetes.md、environment.md、crowdsec.md、mailserver-behind-proxy.md、env.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：bug_report.yml、readme.md、compose.yaml、kubernetes.md、environment.md、crowdsec.md、mailserver-behind-proxy.md、env.md

### 部署运维/备份恢复.md

- reference 标题：备份恢复
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：88
- 页面类型：workflow / workflow
- 行数：334 / 44
- 段落行数：149 / 15
- Evidence：0 / 1
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：readme.md、compose.yaml、full-text-search.md、update-and-cleanup.md、mail_crypt.md、ssl.md、faq.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、compose.yaml、full-text-search.md、update-and-cleanup.md、mail_crypt.md、ssl.md、faq.md

### 部署运维/监控日志.md

- reference 标题：监控日志
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：88
- 页面类型：workflow / workflow
- 行数：248 / 44
- 段落行数：80 / 15
- Evidence：0 / 1
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：readme.md、compose.yaml、ignore.conf、maillog.conf、supervisord.conf、logrotate.conf、rsyslog.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、compose.yaml、ignore.conf、maillog.conf、supervisord.conf、logrotate.conf、rsyslog.conf

### 部署运维/维护更新.md

- reference 标题：维护更新
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：92
- 页面类型：workflow / workflow
- 行数：314 / 44
- 段落行数：115 / 15
- Evidence：0 / 1
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：readme.md、security.md、compose.yaml、update-and-cleanup.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、security.md、compose.yaml、update-and-cleanup.md

### 部署运维/部署运维.md

- reference 标题：部署运维
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：94
- 页面类型：workflow / workflow
- 行数：357 / 44
- 段落行数：89 / 15
- Evidence：17 / 1
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：readme.md、compose.yaml、fetchmail-compose.yaml、relay-compose.yaml、kubernetes.md、podman.md、environment.md、setup.sh.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、compose.yaml、fetchmail-compose.yaml、relay-compose.yaml、kubernetes.md、podman.md、environment.md、setup.sh.md

### 项目概述.md

- reference 标题：项目概述
- 生成页：项目概述.md（项目概述）
- 匹配分数：568
- 页面类型：overview / overview
- 行数：255 / 59
- 段落行数：77 / 22
- Evidence：0 / 4
- Mermaid：3 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：readme.md、compose.yaml、dkim_dmarc_spf.md、fail2ban.md、rspamd.md、introduction.md、usage.md、custom-commands.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、compose.yaml、dkim_dmarc_spf.md、fail2ban.md、rspamd.md、introduction.md、usage.md、custom-commands.conf

### 高级功能/IPv6支持.md

- reference 标题：IPv6支持
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/Kubernetes部署.md

- reference 标题：Kubernetes部署
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：86
- 页面类型：workflow / workflow
- 行数：212 / 44
- 段落行数：92 / 15
- Evidence：0 / 1
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：readme.md、compose.yaml、kubernetes.md、environment.md、mailserver-behind-proxy.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、compose.yaml、kubernetes.md、environment.md、mailserver-behind-proxy.md

### 高级功能/Podman支持.md

- reference 标题：Podman支持
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/全文搜索.md

- reference 标题：全文搜索
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/可选配置.md

- reference 标题：可选配置
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/系统维护.md

- reference 标题：系统维护
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/邮件获取.md

- reference 标题：邮件获取
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/邮件转发/AWS SES 集成.md

- reference 标题：AWS SES 集成
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/邮件转发/Gmail SMTP 配置.md

- reference 标题：Gmail SMTP 配置
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/邮件转发/中继主机配置.md

- reference 标题：中继主机配置
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/邮件转发/邮件转发.md

- reference 标题：邮件转发
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/高级功能.md

- reference 标题：高级功能
- 生成页：无
- 问题：缺少对应生成页面

