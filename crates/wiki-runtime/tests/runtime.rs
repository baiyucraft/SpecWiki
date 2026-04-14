#[path = "suite_env.rs"]
mod suite_env;

#[ctor::ctor]
fn init_runtime_suite_env() {
    suite_env::init_structural_runtime_test_env();
}

// runtime、SQLite、query/update/rebuild 和 markdown merge 相关测试统一走 runtime suite。
#[path = "runtime/editable_runtime.rs"]
mod editable_runtime;
#[path = "runtime/knowledge_artifacts_roundtrip.rs"]
mod knowledge_artifacts_roundtrip;
#[path = "runtime/legacy_page_migration.rs"]
mod legacy_page_migration;
#[path = "runtime/managed_section_kernel.rs"]
mod managed_section_kernel;
#[path = "runtime/metadata_roundtrip.rs"]
mod metadata_roundtrip;
#[path = "runtime/progress_streaming.rs"]
mod progress_streaming;
#[path = "runtime/query_sync_rebuild.rs"]
mod query_sync_rebuild;
#[path = "runtime/section_template_integration.rs"]
mod section_template_integration;
#[path = "runtime/sqlite_lifecycle.rs"]
mod sqlite_lifecycle;
#[path = "runtime/sqlite_runtime_gates.rs"]
mod sqlite_runtime_gates;
#[path = "runtime/sqlite_storage.rs"]
mod sqlite_storage;
#[path = "runtime/state_kernel.rs"]
mod state_kernel;
#[path = "runtime/status_and_update.rs"]
mod status_and_update;
#[path = "runtime/test_support.rs"]
mod test_support;
