use crate::keywords::Keywords;
use crate::types::Template;
use colored::Colorize;
use std::collections::HashMap;
use std::fs;
use crate::utils::gethome;
use toml::Value;

pub struct Config {
    pub path: String,
}

pub const CONFIG_PATH: &str = "{{$HOME}}/.config/idkmng/config.toml";
pub const TEMPLATES_PATH: &str = "{{$HOME}}/.config/idkmng/templates/";
pub const KEYWORDS_FORMAT: &str = "{{$%s:f}}";
pub const KEYWORDS_REGEX: &str = r"\{\{\$[^\s}]+(:[^\s}]+)?\}\}";

impl Config {
    pub fn init(self) {
        eprintln!(
                "\n{}: Looks like first time running idkmng,\n- [{}] creating your config at {}\n- [{}] creating templates folder at {}\n",
                "error".bold().red(),
                "+".red(),
                self.path,
                "+".red(),
                TEMPLATES_PATH
            );

        // this sample is just a template that create config.toml and the new.toml template for the
        // first time, Now something maybe confusing is the "initPJNAME" wtf is it ? 
        // That's just a way to workaround auto replacing PROJECTNAME in templates
        let sample = r#"
[[files]]
path = 'TEMPLATES_PATH/new.toml'
content = '''
[info]
name = "idkmng Template"
description = "A Template for making a template"
author = "Mohamed Tarek @pwnxpl0it"

[[files]]
path="~/.config/idkmng/templates/initPJNAME.toml"
content="""
[info]
name = "initPJNAME"
description = ""
author = ""

[[files]]
path=""
content=\"\"\"

\"\"\"
"""
'''

[[files]]
path = 'CONFIGPATH'
content = '''
[Keywords]
'''
            "#
        .replace("CONFIGPATH", &self.path)
        .replace("TEMPLATES_PATH", &TEMPLATES_PATH.replace("{{$HOME}}", &gethome()));

        Template::extract(sample.to_string(), false);
    }

    pub fn get_keywords(self) -> HashMap<String, String> {
        let mut keywords = HashMap::new();
        if let Ok(toml_str) = fs::read_to_string(&self.path) {
            let toml_val: Value = toml::from_str(&toml_str).unwrap();

            let keywords_table = toml_val.get("Keywords").unwrap().as_table().unwrap();

            for (key, value) in keywords_table.iter() {
                let value_str = match value {
                    Value::String(s) => s.clone(),
                    _ => value.to_string(),
                };
                keywords.insert(Keywords::new(key.to_string(), "".to_string()), value_str);
            }
        } else {
            Self::init(self);
        }

        keywords
    }
}
