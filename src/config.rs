use std::fs;
use std::collections::HashMap;
use toml::Value;
use crate::keywords::Keywords;

pub struct Config {
    pub path: String,
} 

pub const CONFIG_PATH: &str = "{{$HOME}}/.config/idkmng/config.toml";
pub const KEYWORDS_FORMAT: &str = "{{$%s:f}}";
pub const KEYWORDS_REGEX: &str = r"\{\{\$[^\s}]+(:[^\s}]+)?\}\}";

impl Config {
    
    pub fn get_keywords(self) -> HashMap<String, String> {
        let toml_str = fs::read_to_string(self.path).unwrap();
        let toml_val: Value = toml::from_str(&toml_str).unwrap();

        let keywords_table = toml_val
            .get("Keywords")
            .unwrap()
            .as_table()
            .unwrap();

        let mut keywords = HashMap::new();

        for (key, value) in keywords_table.iter() {
            let value_str = match value {
                Value::String(s) => s.clone(),
                _ => value.to_string(),
            };
            keywords.insert(
                Keywords::new(key.to_string(),"".to_string())
                ,value_str);
        }

        keywords
    }
}
