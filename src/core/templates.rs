use crate::utils::*;
use crate::*;
use colored::Colorize;
use promptly::prompt;
use regex::Regex;
use std::{collections::HashMap, fs, path::Path};

pub const KEYWORDS_FORMAT: &str = "{{$%s:f}}";
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
        if self.options.is_none() {
            return None;
        }
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
        let mut options = self.dump_options().expect("No options");

        files.into_iter().for_each(|file| {
            *keywords = find_and_exec(
                file.content.clone(),
                keywords.clone(),
                re.clone(),
                options.json_data.clone().unwrap_or(serde_json::Value::Null),
            );

            *keywords = find_and_exec(
                file.path.clone(),
                keywords.clone(),
                re.clone(),
                options.json_data.clone().unwrap_or(serde_json::Value::Null),
            );

            if file.path.contains("{{$PROJECTNAME}}")
                || file.content.contains("{{$PROJECTNAME}}")
                || options.project_root.contains("{{$PROJECTNAME}}")
            {
                if project.is_empty() {
                    project = prompt("Project name").unwrap();
                    keywords.insert("{{$PROJECTNAME}}".to_string(), project.to_owned());
                    if options.project_root == "{{$PROJECTNAME}}" {
                        options.set_project_root(&project);
                    }
                }
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

        Options::handle_options(options);
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
