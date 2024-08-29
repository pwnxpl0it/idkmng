use crate::Fns;
use colored::*;
use regex::Regex;
use serde_json::Value;
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

//NOTE: in future I may create a module to better handle json data
pub fn json_get_value<'a>(json: &'a Value, indexes: &[&str]) -> Option<&'a Value> {
    let mut current_value = json;
    for index in indexes {
        current_value = current_value.get(index)?;
    }
    Some(current_value)
}

pub fn find_and_exec_fns(
    txt: String,
    mut keywords: HashMap<String, String>,
    re: Regex,
    json_data: serde_json::Value,
) -> HashMap<String, String> {
    if let Some(found) = Fns::find(txt, &keywords, &re) {
        for (keyword_name, (keyword, function)) in found {
            if keyword_name.contains(".") || !keyword.contains(":") {
                let keys: Vec<&str> = keyword_name.split('.').collect();
                if let Some(data) = json_get_value(&json_data, &keys) {
                    keywords.insert(keyword.clone(), data.as_str().unwrap().to_string());
                }

                continue;
            } 

            if let Ok(value) = Fns::exec(function, keyword_name.clone()) {
                keywords.insert(keyword.clone(), value.clone());
                keywords.insert(
                    Fns::remove_fn_name(keyword.clone(), function),
                    value.clone(),
                );
            }
        }
    } else {
        // Ignore
    }
    keywords
}
