use serde::{Deserialize, Serialize};
use wiki_index::TopicSeed;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoContext {
    pub repo_summary_inputs: Vec<String>,
    pub top_modules: Vec<String>,
    pub key_entry_points: Vec<String>,
    pub global_relations: Vec<String>,
    pub tech_stack: Vec<String>,
    #[serde(default)]
    pub graph_hotspots: Vec<String>,
    #[serde(default)]
    pub detected_processes: Vec<String>,
    #[serde(default)]
    pub community_labels: Vec<String>,
    #[serde(default)]
    pub cycle_warnings: Vec<String>,
    #[serde(default)]
    pub root_topics: Vec<TopicSeed>,
    #[serde(default)]
    pub process_topics: Vec<TopicSeed>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModuleContext {
    pub module_id: String,
    pub role_hints: Vec<String>,
    pub public_surface: Vec<String>,
    pub dependencies: Vec<String>,
    pub dependents: Vec<String>,
    pub key_sources: Vec<String>,
    #[serde(default)]
    pub graph_hotspots: Vec<String>,
    #[serde(default)]
    pub communities: Vec<String>,
    #[serde(default)]
    pub cycle_warnings: Vec<String>,
    #[serde(default)]
    pub capability_topics: Vec<TopicSeed>,
}
