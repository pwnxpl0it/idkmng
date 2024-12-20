use crate::utils::*;
use colored::Colorize;
use promptly::prompt;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};
pub mod file;
pub mod options;
use crate::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Options {
    pub git: bool,
    pub json_data: Option<serde_json::Value>,
    pub project_root: String,
}

pub const KEYWORDS_REGEX: &str = r"\{\{\$.*?\}\}";

impl Default for Template {
    fn default() -> Self {
        Self {
            options: Some(Options::default()),
            info: None,
            files: None,
        }
    }
}

impl Template {
    pub fn set_info(&mut self, info: Information) {
        self.info = Some(info);
    }

    pub fn set_files(&mut self, files: Vec<File>) {
        self.files = Some(files);
    }

    pub fn set_options(&mut self, options: Options) {
        self.options = Some(options);
    }

    pub fn dump_options(&mut self) -> Option<Options> {
        self.options.as_ref()?;
        Some(self.options.clone().unwrap())
    }

    pub fn generate(dest: &str) {
        let mut files: Vec<File> = Vec::new();

        list_files(Path::new("./")).iter().for_each(|file| {
            //TODO: Add more to ignore list maybe adding a --ignore flag will be good
            if !file.contains(".git") {
                let file = File::from(file.to_string().replace("./", ""), {
                    match fs::read_to_string(file) {
                        Ok(content) => content,
                        Err(e) => panic!("{}:{}", file.red().bold(), e),
                    }
                });
                files.push(file); // Push to Files Vector
            }
        });

        let template = Template {
            info: None,
            files: Some(files),
            options: None,
        };

        let toml_string = toml::to_string_pretty(&template).expect("Failed to create toml string");
        fs::write(dest, toml_string).unwrap();
    }

    //TODO: maybe move this to utils or just crate a submodule for it
    pub fn liquify(string: &str) -> String {
        let parser = liquid::ParserBuilder::with_stdlib().build().unwrap();
        let empty_globals = liquid::Object::new();

        parser
            .parse(string)
            .unwrap()
            .render(&empty_globals)
            .unwrap()
    }

    pub fn extract(mut self, keywords: &mut HashMap<String, String>) {
        let mut project = String::from("");
        let mut output = String::from("");
        let re = Regex::new(KEYWORDS_REGEX).unwrap();
        let files = self.files.clone().expect("No files table");
        let mut options = self.dump_options().unwrap_or_default();

        files.into_iter().for_each(|file| {
            *keywords = Fns::find_and_exec(
                file.content.clone(),
                keywords.clone(),
                re.clone(),
                options.json_data.clone().unwrap_or(serde_json::Value::Null),
            );

            *keywords = Fns::find_and_exec(
                file.path.clone(),
                keywords.clone(),
                re.clone(),
                options.json_data.clone().unwrap_or(serde_json::Value::Null),
            );

            if (file.path.contains("{{$PROJECTNAME}}")
                || file.content.contains("{{$PROJECTNAME}}")
                || options.project_root.contains("{{$PROJECTNAME}}"))
                && project.is_empty()
            {
                project = prompt("Project name").unwrap();
                options.set_project_root(&project);
                keywords.insert("{{$PROJECTNAME}}".to_string(), project.to_owned());
            }

            let dir = file.path.split('/').collect::<Vec<_>>();
            let path = Keywords::replace_keywords(keywords.to_owned(), file.path.to_owned());

            if dir.len() > 1 {
                create_dirs(&shellexpand::tilde(&Keywords::replace_keywords(
                    keywords.to_owned(),
                    file.path.to_owned().replace(dir[dir.len() - 1], ""),
                )))
            }

            output = Keywords::replace_keywords(keywords.to_owned(), file.content);

            let liquified = Self::liquify(&output);

            write_content(&shellexpand::tilde(&path), liquified)
        });

        options.handle();
    }

    pub fn show_info(template: &Self) {
        match &template.info {
            Some(information) => println!(
                "{}: {}\n{}: {}\n{}: {}\n",
                "Name".yellow(),
                information.name.as_ref().unwrap().bold().green(),
                "Description".yellow(),
                information.description.as_ref().unwrap().bold().green(),
                "Author".yellow(),
                information.author.as_ref().unwrap().bold().green()
            ),
            None => {}
        }
    }
}
