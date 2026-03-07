pub mod app;
pub mod domain;
pub mod generation;
pub mod repo;
pub mod storage;
pub mod transport;

pub fn workspace_name() -> &'static str {
    "wiki-core"
}
