// 层级规划、模块归并和页面拓扑相关测试统一收口到一个 suite。
#[path = "hierarchy/hierarchy_noise_filter.rs"]
mod hierarchy_noise_filter;
#[path = "hierarchy/hierarchy_planning.rs"]
mod hierarchy_planning;
#[path = "hierarchy/key_source_selection.rs"]
mod key_source_selection;
#[path = "hierarchy/module_kind_classification.rs"]
mod module_kind_classification;
#[path = "hierarchy/page_identity_stability.rs"]
mod page_identity_stability;
#[path = "hierarchy/page_topology_integration.rs"]
mod page_topology_integration;
#[path = "hierarchy/planner_merge_strategy.rs"]
mod planner_merge_strategy;
