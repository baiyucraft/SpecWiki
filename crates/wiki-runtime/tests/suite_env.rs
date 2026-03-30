use std::sync::Once;

static TEST_ENV_INIT: Once = Once::new();

pub fn init_structural_runtime_test_env() {
    TEST_ENV_INIT.call_once(|| {
        std::env::set_var("SPEC_WIKI_ALLOW_STRUCTURAL_RUNTIME", "1");
    });
}
