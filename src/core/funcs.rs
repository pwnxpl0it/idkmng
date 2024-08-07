use crate::Fns;
use colored::*;
use core::fmt;
use promptly::prompt;
use regex::Regex;
use std::{collections::HashMap, env};
use indexmap::IndexMap;

impl std::fmt::Display for Fns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Fns::Read => write!(f, "read"),
            Fns::Env => write!(f, "env"),
        }
    }
}

impl Fns {
    /// This method removes function name from keywords,
    /// Example: {{$hi:read}}
    /// returns: {{$hi}}
    pub fn remove_fn_name(keyword: String, func_name: Fns) -> String {
        keyword.replace(&format!(":{}", func_name), "")
    }

    /// This method finds `functions` in a string based on a Regex pattern that matches keywords with functions
    pub fn find(
    txt: String,
    keywords: &HashMap<String, String>,
    re: &Regex,
) -> Option<IndexMap<String, (String, Fns)>> {
    let mut found = IndexMap::new();

    for key in re.captures_iter(&txt) {
        if let Some(key) = key.get(0) {
            let keyword = key.as_str().to_string();

            if !keywords.contains_key(&keyword) {
                let data = keyword.as_str().split(':').collect::<Vec<_>>();

                if data.len() == 2 {
                    let keyword_name = data[0].replace("{{$", "").replace("}}", "");
                    let func = data[1].replace("}}", "");

                    match func.as_str() {
                        "read" => {
                            found.insert(keyword_name, (keyword.clone(), Fns::Read));
                        }
                        "env" => {
                            found.insert(keyword_name, (keyword.clone(), Fns::Env));
                        }
                        _ => {
                            eprintln!(
                                "\n{}: '{}' is not a valid function",
                                "error".to_string().red(),
                                func.yellow()
                            );
                            return None;
                        }
                    }
                }
            }
        }
    }

    Some(found)
}

    /// This method executes allowed functions such as: read,env
    pub fn exec(func: Fns, keyword_name: String) -> Result<String, String> {
        match func {
            Fns::Read => Ok(prompt(keyword_name).unwrap()),
            Fns::Env => Ok(Self::env(keyword_name)),
        }
    }

    /// This function reads from environment variables and returns the value as a string
    fn env(name: String) -> String {
        env::var(name).unwrap()
    }
}
