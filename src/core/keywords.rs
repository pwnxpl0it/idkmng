use crate::config::KEYWORDS_FORMAT;
use crate::Config;
use crate::Keywords;
use chrono::Datelike;
use std::{collections::HashMap, env};

impl Keywords {
    pub fn new(name: String, function: Option<String>) -> String {
        if function.is_some() {
            KEYWORDS_FORMAT
                .to_string()
                .replace("%s", &name)
                .replace('f', &function.unwrap())
        } else {
            KEYWORDS_FORMAT
                .to_string()
                .replace("%s", &name)
                .replace(":f", "")
        }
    }

    pub fn strip(keyword: String) -> String {
        keyword.replace("{{$", "").replace("}}", "")
    }

    pub fn init(config: Config) -> HashMap<String, String> {
        let mut keywords = HashMap::new();
        keywords.insert(
            Self::new(String::from("HOME"), None),
            env::var("HOME").unwrap(),
        );
        keywords.insert(
            Self::new(String::from("PROJECTNAME"), None),
            "".to_string(),
        );
        keywords.insert(
            Self::new(String::from("CURRENTDIR"), None),
            env::current_dir()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        );

        keywords.insert(
            Self::new(String::from("NOW_UTC"), None),
            chrono::Utc::now().to_string(),
        );

        keywords.insert(
            Self::new(String::from("NOW"), None),
            chrono::Local::now().to_string(),
        );

        keywords.insert(
            Self::new(String::from("YYYY"), None),
            chrono::Local::now().year().to_string(),
        );

        keywords.insert(
            Self::new(String::from("YY"), None),
            chrono::Local::now().format("%y").to_string(),
        );

        keywords.insert(
            Self::new(String::from("MM"), None),
            chrono::Local::now().month().to_string(),
        );

        keywords.insert(
            Self::new(String::from("DD"), None),
            chrono::Local::now().day().to_string(),
        );

        let other_keywords = config.expand();

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
