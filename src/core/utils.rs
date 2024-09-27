use crate::Fns;
use colored::*;
use jq_rs;
use regex::Regex;
use std::{collections::HashMap, fs, path::Path};

pub fn create_dirs(dir: &str) {
    match fs::create_dir_all(dir) {
        Ok(_) => println!("{}: {}", "creating directory".blue(), dir.bold().green()),
        Err(e) => eprintln!("{}: {}", "error".red(), e),
    }
}

pub fn write_content(path: &str, content: String) {
    match fs::write(path, content.replace("initPJNAME", "{{$PROJECTNAME}}")) {
        Ok(_) => println!("{}: {}", "file written".blue(), path.bold().green()),
        Err(e) => eprintln!("{}: {} {}", "error".red(), path, e),
    }
}

pub fn list_files(dir: &Path) -> Vec<String> {
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

pub fn find_and_exec(
    txt: String,
    mut keywords: HashMap<String, String>,
    re: Regex,
    json_data: serde_json::Value,
) -> HashMap<String, String> {
    if let Some(found) = Fns::find(txt, &keywords, &re) {
        for (keyword_name, (keyword, function)) in found {
            //HACK: Just a bit of optimization, if the json_data is null then it doesn't make sense to run jq
            // because doing so is every expensive and here we are dealing with dynamic queries
            if !json_data.is_null() {
                if keyword_name.contains(".") {
                    //TODO: This is not very performant but it works for now UwU
                    let output = jq_rs::run(&keyword_name, &json_data.to_string());

                    if let Ok(value) = &output {
                        //NOTE: This will also replace any quotes in the value
                        keywords.insert(keyword, value.replace("\"", ""));
                    }
                    continue;
                }
            }

            if let Ok(value) = Fns::exec(function, keyword_name) {
                match function {
                    Fns::None => {
                        println!(
                            "\n[{}] {}: {}\n",
                            "WRN".yellow(),
                            "Value not found".yellow(),
                            keyword.green()
                        );
                        keywords.insert(keyword, "".to_string());
                    }
                    _ => {
                        keywords.insert(keyword.clone(), value.clone());
                        keywords.insert(Fns::remove_fn_name(keyword, function), value);
                    }
                }
            }
        }
    } else {
        // Ignore
    }
    keywords
}
