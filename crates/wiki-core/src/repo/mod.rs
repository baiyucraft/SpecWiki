//! `repo` 层负责从本地代码目录提取基础事实。
//! 这一层只做 deterministic 扫描和启发式拆分，不承担页面生成职责。

pub mod detectors;
pub mod fingerprint;
pub mod git;
pub mod hierarchy;
pub mod language_processors;
pub mod parsers;
pub mod scanner;
pub mod symbols;
