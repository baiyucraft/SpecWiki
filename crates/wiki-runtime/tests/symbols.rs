#[path = "suite_env.rs"]
mod suite_env;

#[ctor::ctor]
fn init_symbols_suite_env() {
    suite_env::init_structural_runtime_test_env();
}

// symbol parsing、resolution 和 graph analysis 相关测试统一走 symbols suite。
#[path = "symbols/symbol_graph_analysis.rs"]
mod symbol_graph_analysis;
#[path = "symbols/symbol_parsing.rs"]
mod symbol_parsing;
#[path = "symbols/symbol_resolution.rs"]
mod symbol_resolution;
