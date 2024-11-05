use colored::*;
use std::{fs, path::Path};

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
