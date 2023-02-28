use crate::config::{Config,CONFIG_PATH,KEYWORDS_FORMAT};
use std::{env,collections::HashMap};

pub struct Keywords {}

impl Keywords {
    pub fn new(name: String) -> String{
        KEYWORDS_FORMAT.to_string().replace("%s",&name)
    }

    pub fn init() -> HashMap<String, String> {
        let mut keywords = HashMap::new();
        keywords.insert(Self::new(String::from("HOME")) , env::var("HOME").unwrap());
        keywords.insert(Self::new(String::from("PROJECTNAME")), "".to_string());
        keywords.insert(Self::new(String::from("CURRENTDIR")),env::current_dir().unwrap()
                        .file_name().unwrap()
                        .to_str().unwrap()
                        .to_string());
        let other_keywords = Config{
            path: CONFIG_PATH.replace("{{$HOME}}",&keywords["{{$HOME}}"]),
        }.get_keywords(); // Special keywords
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
