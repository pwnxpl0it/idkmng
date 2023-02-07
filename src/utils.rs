use serde::{Deserialize,Serialize};
use std::{
    collections::HashMap,
    path::Path,
    env, fs, io
};
use toml;

fn create_dirs(dir: &str) {
    match fs::create_dir_all(dir) {
        Ok(_) => println!("Target Directory: {}", dir),
        Err(e) => eprintln!("{}", e),
    }
}

fn write_content(path: &str, content: String) {
    match fs::write(path, content) {
        Ok(_) => println!("file written: {}", path),
        Err(e) => eprintln!("Error writing file: {} {}", path, e),
    }
}

fn init_keywords() -> HashMap<&'static str, String> {
    let mut keywords = HashMap::new(); //TODO: Add custom keywords
    keywords.insert("$HOME", env::var("HOME").unwrap());
    keywords.insert("$PROJECTNAME", "".to_string());
    keywords.insert("$CURRENTDIR", env::current_dir().unwrap()
                    .file_name().unwrap()
                    .to_str().unwrap()
                    .to_string());
    keywords
}

fn replace_keywords(keywords: HashMap<&str, String>, mut data: String) -> String {
    for (key, value) in keywords.iter() {
        data = data.replace(key, value);
    }
    data
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

#[derive(Debug, Deserialize,Serialize)]
pub struct Template {
    pub info: Option<Information>,
    pub files: Vec<File>,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct Information {
    pub name: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct File {
    pub path: String,
    pub content: String,
}

impl File {
    fn new(path_: String, content_: String) -> Self{
        File { path: path_, content: content_ }
    }
}

impl Template {

    fn new(info_: Information,files_:Vec<File>) -> Self{
        return Self{info: Some(info_),files: files_}
    }

    pub fn generate(){
        let dest = format!("{}.toml", init_keywords()["$CURRENTDIR"]);
        println!("Creating Template: {}",&dest);
        let mut files: Vec<File> = Vec::new();
        list_files(Path::new("./")).iter().for_each(|file|{
            if !file.contains(".git"){ //TODO: Add more to ignore list maybe adding a --ignore flag will be good 
                let file = File::new(
                    file.to_string().replace("./",""),{
                    match fs::read_to_string(file){
                        Ok(content) => content,
                        Err(e) => panic!("{}:{}",file,e),
                    }}
                ); 
                files.push(file);
            }
        });

       let template = Self::new(
           //TODO: Thinking to make constant information from a config file
           Information {
               name: Some(String::from("")),
               author: Some(String::from("")),
               description: Some(String::from("")),
           },files);

        let toml_string = toml::to_string_pretty(&template).expect("Failed to create toml string");
        fs::write(&dest,toml_string).unwrap();
    }

    pub fn extract(filename: String) {
        let mut keywords = init_keywords();
        let template = Self::validate_template(filename, keywords.to_owned());

        println!("Using Template: {}", &template);

        let sample = Self::parse_template(&template);
        Self::show_info(&sample);

        let files = sample.files;
        let mut project = String::new();
        files.into_iter().for_each(|file| {
            if file.path.contains("$PROJECTNAME") || file.content.contains("$PROJECTNAME") {
                if project.len() == 0 {
                    println!("Project name: ");
                    io::stdin().read_line(&mut project).unwrap();
                    project = project.trim().to_string();
                    keywords.insert("$PROJECTNAME", project.to_owned());
                }
            }

            let dir = file.path.split("/").collect::<Vec<_>>();
            let path = replace_keywords(keywords.to_owned(), file.path.to_owned());

            if dir.len() > 1 {
                create_dirs(&replace_keywords(
                    keywords.to_owned(),
                    file.path
                        .replace(&dir[dir.len() - 1], "")
                        .replace("~",&keywords["$HOME"]),
                ));
            }

            write_content(
                &path.replace("~",&keywords["$HOME"]),
                replace_keywords(keywords.to_owned(),file.content),
            )
        });
    }

    fn parse_template(template: &str) -> Self {
        let content = fs::read_to_string(template)
            .expect(format!("Failed to Parse {}", template).as_str());
        toml::from_str(&content).unwrap()
    }

    // I think this should be implemented in Cli::args idk it just works
    fn validate_template(mut template: String, keywords: HashMap<&str, String>) -> String {
        if template.contains(".toml") {
            //IGNORE
        } else {
            template += ".toml"
        }

        if template.contains("/") {
            //IGNORE
        } else {
            template = "~/.config/idkmng/templates/".replace(
                "~" , &keywords["$HOME"]
                ) + &template
        }
        template
    }

    fn show_info(template: &Self) {
        match &template.info {
            Some(information) => println!(
                "{}",
                format!(
                    "
Name: {} 
Description: {}
Author: {}
    ",
                    information.name.as_ref().unwrap(),
                    information.description.as_ref().unwrap(),
                    information.author.as_ref().unwrap()
                )
            ),
            None => {}
        }
    }
}
