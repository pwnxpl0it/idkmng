use crate::config::KEYWORDS_REGEX;
use crate::config::TEMPLATES_PATH;
use crate::funcs::*;
use crate::keywords::Keywords;
use colored::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io, path::Path};

fn create_dirs(dir: &str) {
    match fs::create_dir_all(dir) {
        Ok(_) => println!("{}: {}", "creating directory".blue(), dir.bold().green()),
        Err(e) => eprintln!("{}: {}", "error".red(), e),
    }
}

fn write_content(path: &str, content: String) {
    match fs::write(path, content) {
        Ok(_) => println!("{}: {}", "file written".blue(), path.bold().green()),
        Err(e) => eprintln!("{}: {} {}", "error".red(), path, e),
    }
}

fn list_files(dir: &Path) -> Vec<String> {
    let mut files = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                files.extend(list_files(&path));
            } else {
                files.push(path.to_string_lossy().into_owned());
            }
        }
    }
    files
}

fn find_and_exec_fns(
    txt: String,
    mut keywords: HashMap<String, String>,
    re: Regex,
) -> HashMap<String, String> {
    if let Some((keyword_name, keyword, function)) = Fns::find(txt, keywords.clone(), re) {
        let value = Fns::exec(function, keyword_name.clone()).unwrap();
        keywords.insert(keyword.clone(), value.clone());
        keywords.insert(
            Fns::remove_fn_name(keyword.clone(), function.clone()),
            value.clone(),
        );
    } else {
        //Ignore
    }

    keywords
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Template {
    pub info: Option<Information>,
    pub files: Vec<File>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Information {
    pub name: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub path: String,
    pub content: String,
}

impl File {
    fn new(path_: String, content_: String) -> Self {
        File {
            path: path_,
            content: content_,
        }
    }
}

impl Template {
    fn new(info_: Information, files_: Vec<File>) -> Self {
        Self {
            info: Some(info_),
            files: files_,
        }
    }

    pub fn generate() {
        let dest = format!("{}.toml", Keywords::init()["{{$CURRENTDIR}}"]);
        println!("{}: {}", "Creating Template".bold().green(), &dest.yellow());
        let mut files: Vec<File> = Vec::new();
        list_files(Path::new("./")).iter().for_each(|file| {
            if !file.contains(".git") {
                //TODO: Add more to ignore list maybe adding a --ignore flag will be good
                let file = File::new(file.to_string().replace("./", ""), {
                    match fs::read_to_string(file) {
                        Ok(content) => content,
                        Err(e) => panic!("{}:{}", file.red(), e),
                    }
                });
                files.push(file);
            }
        });

        let template = Self::new(
            //TODO: Thinking to make constant information from a config file
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

    pub fn extract(filename: String) {
        let mut keywords = Keywords::init();
        let template = Self::validate_template(filename, keywords.to_owned());
        let re = Regex::new(KEYWORDS_REGEX).unwrap();

        println!("{}: {}", "Using Template".blue(), &template.magenta());

        let sample = Self::parse_template(&template);
        Self::show_info(&sample);

        let files = sample.files;
        let mut project = String::from("");
        files.into_iter().for_each(|file| {
            //TODO: Instead of calling this function twice, I should extend the function to handle content and path
            keywords = find_and_exec_fns(file.content.clone(), keywords.clone(), re.clone());
            keywords = find_and_exec_fns(file.path.clone(), keywords.clone(), re.clone());

            if file.path.contains("{{$PROJECTNAME}}") || file.content.contains("{{$PROJECTNAME}}") {
                if project.is_empty() {
                    println!("Project name: ");
                    io::stdin().read_line(&mut project).unwrap();
                    project = project.trim().to_string();
                    keywords.insert("{{$PROJECTNAME}}".to_string(), project.to_owned());
                }
            }

            let dir = file.path.split('/').collect::<Vec<_>>();
            let path = Keywords::replace_keywords(keywords.to_owned(), file.path.to_owned());

            if dir.len() > 1 {
                create_dirs(
                    &Keywords::replace_keywords(
                        keywords.to_owned(),
                        file.path.to_owned().replace(dir[dir.len() - 1], ""),
                    )
                    .replace('~', &keywords["{{$HOME}}"]),
                )
            }

            write_content(
                &path.replace('~', &keywords["{{$HOME}}"]),
                Keywords::replace_keywords(keywords.to_owned(), file.content),
            )
        });
    }

    fn parse_template(template: &str) -> Self {
        let content =
            fs::read_to_string(template).unwrap_or_else(|_| panic!("Failed to Parse {}", template));
        toml::from_str(&content).unwrap()
    }

    // I think this should be implemented in Cli::args idk it just works
    fn validate_template(mut template: String, keywords: HashMap<String, String>) -> String {
        if template.contains(".toml") {
            //IGNORE
        } else {
            template += ".toml"
        }

        if fs::read_to_string(&template).is_ok() {
            //IGNORE
        } else {
            template = TEMPLATES_PATH.replace("{{$HOME}}", &keywords["{{$HOME}}"]) + &template
        }

        template
    }

    fn show_info(template: &Self) {
        match &template.info {
            Some(information) => println!(
                "
{}: {} 
{}: {}
{}: {}
    ",
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
