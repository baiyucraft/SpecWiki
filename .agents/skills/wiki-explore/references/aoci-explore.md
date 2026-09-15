# AOCI Explore Procedure

1. 先读取官方 Guide 和完整 Overview，确认当前 Code Cognition 是否可依赖。
2. 用 `verify` / `check` 识别治理漂移；未对齐时遵循官方 Maintain/Recovery，不自行推导 AOCI 状态。
3. 只提取与 change 有关的职责、强关系、约束、已确认语义和未知项。
4. 写入 `research/aoci.md`，记录 Overview/Attestation 来源、时间、事实、推断、冲突和降级。
5. 不复制完整 Overview，不读取数据库业务行，不保存 credential 值。
6. AOCI 结果用于长期语义；当前符号、调用链、影响和测试仍由 CodeGraph 与源码/测试核验。
