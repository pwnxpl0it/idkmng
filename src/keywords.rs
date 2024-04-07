use crate::config::{Config, CONFIG_PATH, KEYWORDS_FORMAT};
use chrono::Datelike;
use std::{collections::HashMap, env};

pub struct Keywords {}

impl Keywords {
    pub fn new(name: String, function: String) -> String {
        if !function.is_empty() {
            KEYWORDS_FORMAT
                .to_string()
                .replace("%s", &name)
                .replace('f', &function)
        } else {
            KEYWORDS_FORMAT
                .to_string()
                .replace("%s", &name)
                .replace(":f", "")
        }
    }

    pub fn init() -> HashMap<String, String> {
        let mut keywords = HashMap::new();
        keywords.insert(
            Self::new(String::from("HOME"), "".to_string()),
            env::var("HOME").unwrap(),
        );
        keywords.insert(
            Self::new(String::from("PROJECTNAME"), "".to_string()),
            "".to_string(),
        );
        keywords.insert(
            Self::new(String::from("CURRENTDIR"), "".to_string()),
            env::current_dir()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        );

        keywords.insert(
            Self::new(String::from("NOW_UTC"), "".to_string()),
            chrono::Utc::now().to_string(),
        );

        keywords.insert(
            Self::new(String::from("NOW"), "".to_string()),
            chrono::Local::now().to_string(),
        );

        keywords.insert(
            Self::new(String::from("YYYY"), "".to_string()),
            chrono::Local::now().year().to_string(),
        );

        keywords.insert(
            Self::new(String::from("YY"), "".to_string()),
            chrono::Local::now().format("%y").to_string(),
        );

        keywords.insert(
            Self::new(String::from("MM"), "".to_string()),
            chrono::Local::now().month().to_string(),
        );

        keywords.insert(
            Self::new(String::from("DD"), "".to_string()),
            chrono::Local::now().day().to_string(),
        );

        let other_keywords = Config {
            path: CONFIG_PATH.replace("{{$HOME}}", &keywords["{{$HOME}}"]),
        }
        .get_keywords(); // Special keywords
        keywords.extend(other_keywords);
        keywords
    }

    pub fn replace_keywords(keywords: HashMap<String, String>, mut data: String) -> String {
        for (key, value) in keywords.iter() {
            data = data.replace(key, value);
        }
        data
    }
}
