use serde::{Deserialize, Serialize};
pub mod git;
pub mod opts;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Options {
    pub git: bool,
    pub json_data: Option<serde_json::Value>,
    pub project_root: String,
}
