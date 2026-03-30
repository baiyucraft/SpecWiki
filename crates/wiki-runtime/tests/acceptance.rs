#[path = "suite_env.rs"]
mod suite_env;

#[ctor::ctor]
fn init_acceptance_suite_env() {
    suite_env::init_structural_runtime_test_env();
}

// 按主题聚合 acceptance / CLI 合约相关 integration tests，避免 tests 根目录继续平铺增长。
#[path = "acceptance/baseline_acceptance.rs"]
mod baseline_acceptance;
#[path = "acceptance/command_contract.rs"]
mod command_contract;
#[path = "acceptance/init_generates_wiki.rs"]
mod init_generates_wiki;
