use crate::options::git;
use crate::Options;
use colored::*;

impl Default for Options {
    fn default() -> Self {
        Self {
            json_data: Some(serde_json::Value::Null),
            git: false,
            project_root: String::new(),
        }
    }
}

impl Options {
    pub fn set_git(&mut self, git: bool) {
        self.git = git;
    }

    pub fn set_json(&mut self, json_data: serde_json::Value) {
        self.json_data = Some(json_data);
    }

    pub fn set_project_root(&mut self, project_root: &str) {
        self.project_root = project_root.to_string();
    }

    pub fn handle_options(self) {
        if self.git {
            if self.project_root.is_empty() {
                println!(
                    "{}: {}",
                    "error".red().bold(),
                    "Please specify a project root.".red().bold()
                );
                return;
            }

            println!(
                "\nInitializing git repository for {}\n",
                self.project_root.blue()
            );

            git::git_init(&self.project_root);
        }
    }
}
