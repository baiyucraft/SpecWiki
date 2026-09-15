# AOCI Design Procedure

1. 读取 `research/aoci.md` 与最新完整 Overview，确认长期职责、关系和约束仍对齐。
2. 将需要保持或改变的系统语义映射到明确接口、ownership、失败和回滚边界。
3. 在 `AOCI-derived semantic constraints` 记录 confirmed semantics、planned semantic delta、Maintain timing 与数据库条件。
4. 如果 AOCI 与源码/测试冲突，记录两边证据并把治理漂移作为阻塞，不以手工改写 formal cognition 消除差异。
5. CodeGraph-derived 章节单独记录当前调用和影响；不要混成一份不可追溯的“工具分析”。
