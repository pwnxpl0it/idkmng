use crate::config::{
    Config,
    CONFIG_PATH,
    KEYWORDS_FORMAT
};
use std::{env,collections::HashMap};

pub struct Keywords {}

impl Keywords {
    pub fn new(name: String,function: String) -> String{
        if !function.is_empty(){
            KEYWORDS_FORMAT.to_string().replace("%s",&name).replace("f",&function)
        }else{
            KEYWORDS_FORMAT.to_string().replace("%s",&name).replace(":f","")
        }
    }

    pub fn init() -> HashMap<String, String> {
        let mut keywords = HashMap::new();
        keywords.insert(Self::new(String::from("HOME"),"".to_string()), env::var("HOME").unwrap());
        keywords.insert(Self::new(String::from("PROJECTNAME"),"".to_string()), "".to_string());
        keywords.insert(Self::new(String::from("CURRENTDIR"),"".to_string()),env::current_dir().unwrap()
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
