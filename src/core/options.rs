use crate::Options;
use colored::*;
use std::process::Command;

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

    pub fn check_git() -> Result<(), String> {
        if Command::new("git").arg("--version").spawn().is_err() {
            return Err("Git is not installed. Please install git and try again.".to_string());
        }
        Ok(())
    }

    pub fn git_init(self) {
        if let Err(e) = Self::check_git() {
            println!("{}: {}", "error".red().bold(), e.red().bold());
            return;
        }

        if self.project_root.is_empty() {
            println!(
                "{}: {}",
                "error".red().bold(),
                "Please specify a project root.".red().bold()
            );
            return;
        }

        if let Err(e) = std::env::set_current_dir(&self.project_root) {
            println!(
                "{}: {}",
                "error".red().bold(),
                format!("Failed to change directory: {}", e).red().bold()
            );
            return;
        }

        let cmd = Command::new("git")
            .arg("init") //TODO: maybe add git arguments? that can be a bit risky..
            .stderr(std::process::Stdio::null()) // hide hints and errors
            .status();

        if let Ok(status) = cmd {
            if status.success() {
                println!("{}", "\n✅ Git initialized successfully.".green().bold());
            } else {
                println!(
                    "{}: {}",
                    "error".red().bold(),
                    "Git initialization failed.".red().bold()
                );
            }
        } else {
            println!(
                "{}: {}",
                "error".red().bold(),
                "Failed to run git command.".red().bold()
            );
        }
    }

    pub fn handle_options(self) {
        if self.git {
            println!(
                "\nInitializing git repository for {}\n",
                self.project_root.blue()
            );
            self.git_init();
        }
    }
}
