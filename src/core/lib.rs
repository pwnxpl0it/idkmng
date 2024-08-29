pub mod config;
mod file;
pub mod funcs;
pub mod keywords;
mod templates;
mod utils;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Config {
    pub path: String,
    pub templates_path: String,
}

pub struct Keywords {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Information {
    pub name: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct File {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Template {
    pub info: Option<Information>,
    pub files: Vec<File>,
}

#[derive(Clone, Copy)]
pub enum Fns {
    Read,
    Env,
    None,
}
