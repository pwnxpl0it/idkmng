use crate::config::*;
use crate::utils::*;
use crate::*;
use colored::Colorize;
use promptly::prompt;
use regex::Regex;
use std::{collections::HashMap, fs, path::Path};

impl Template {
    /// This method returns a new Template instance, it takes `Information` and a vector of `File`.
    fn new(info_: Information, files_: Vec<File>) -> Self {
        Self {
            info: Some(info_),
            files: files_,
        }
    }

    /// This method basically generates a new Template and saves it
    /// It utilizes the Self::new() method, gives it an Empty `Information` Instance and the
    /// Vector it created by listing all files in the current working directory
    pub fn generate(dest: &str) {
        // get the current directory name by utilizing Keywords::init() -> a table of default
        // Keywords/Values for idkmng, that includes current directory

        let mut files: Vec<File> = Vec::new(); // Create a new Vector of File

        list_files(Path::new("./")).iter().for_each(|file| {
            //TODO: Add more to ignore list maybe adding a --ignore flag will be good
            if !file.contains(".git") {
                let file = File::new(file.to_string().replace("./", ""), {
                    match fs::read_to_string(file) {
                        Ok(content) => content,
                        Err(e) => panic!("{}:{}", file.red().bold(), e),
                    }
                });
                files.push(file); // Push to Files Vector
            }
        });

        let template = Self::new(
            Information {
                name: Some(String::from("")),
                author: Some(String::from("")),
                description: Some(String::from("")),
            },
            files,
        );

        let toml_string = toml::to_string_pretty(&template).expect("Failed to create toml string");
        fs::write(&dest, toml_string).unwrap();
    }

    /// This method "extracts" a template, means it takes a template and starts initializing files based on that template
    pub fn extract(template: String, is_file: bool, config: Config) {
        let mut keywords: HashMap<String, String>;
        let re = Regex::new(KEYWORDS_REGEX).unwrap();

        keywords = Keywords::init(config);

        let sample = Self::parse(&template, is_file);

        let files = sample.files;
        let mut project = String::from("");
        files.into_iter().for_each(|file| {
            keywords = find_and_exec_fns(file.content.clone(), keywords.clone(), re.clone());
            keywords = find_and_exec_fns(file.path.clone(), keywords.clone(), re.clone());

            if file.path.contains("{{$PROJECTNAME}}") || file.content.contains("{{$PROJECTNAME}}") {
                if project.is_empty() {
                    project = prompt("Project name").unwrap();
                    keywords.insert("{{$PROJECTNAME}}".to_string(), project.to_owned());
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

            write_content(
                &shellexpand::tilde(&path),
                Keywords::replace_keywords(keywords.to_owned(), file.content),
            )
        });
    }

    /// Parse a Template
    pub fn parse(template: &str, is_file: bool) -> Self {
        #[allow(unused_assignments)]
        let mut content = String::from("");
        match is_file {
            true => {
                content = fs::read_to_string(template)
                    .unwrap_or_else(|_| panic!("Failed to Parse {}", template));
            }
            false => content = template.to_string(),
        }

        toml::from_str(&content).unwrap()
    }

    /// This method validates template path, in other words it just checks if the template is in
    /// the current working directory,if not it uses the default templates directory, also automatically adds .toml
    pub fn validate(mut template: String, template_path: String) -> String {
        if !template.contains(".toml") {
            template += ".toml"
        }

        if !fs::read_to_string(&template).is_ok() {
            template = template_path + &template
        }

        template
    }

    /// This method shows information about current template, basically Reads them from Information
    /// section in the template TOML file
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
