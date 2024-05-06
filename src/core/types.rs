use serde::{Deserialize, Serialize};

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
}
