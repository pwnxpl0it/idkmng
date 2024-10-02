use crate::Fns;
use crate::Keywords;
use colored::*;
use core::fmt;
use indexmap::IndexMap;
use promptly::prompt;
use regex::Regex;
use std::{collections::HashMap, env};

impl std::fmt::Display for Fns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Read => write!(f, "read"),
            Self::Env => write!(f, "env"),
            Self::None => write!(f, ""),
        }
    }
}

impl Fns {
    pub fn remove_fn_name(keyword: String, func_name: Self) -> String {
        keyword.replace(&format!(":{}", func_name), "")
    }

    /// This method finds `functions` in a string based on a Regex pattern that matches keywords
    pub fn find(
        txt: String,
        keywords: &HashMap<String, String>,
        re: &Regex,
    ) -> Option<IndexMap<String, (String, Self)>> {
        let mut found = IndexMap::new();

        for key in re.captures_iter(&txt) {
            if let Some(key) = key.get(0) {
                let keyword = key.as_str().to_string();

                if !keywords.contains_key(&keyword) {
                    let new_keyword = Keywords::strip(keyword.clone()).trim().to_string();
                    let data = new_keyword.split(':').collect::<Vec<_>>();

                    if data.len() == 2 {
                        let keyword_name = Keywords::strip(data[0].to_string());
                        let func = data[1].trim();

                        //TODO: handle the whitespaces better than this
                        match func {
                            "read" => {
                                found.insert(keyword_name, (keyword, Self::Read));
                            }
                            "env" => {
                                found.insert(keyword_name, (keyword, Self::Env));
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
                    } else {
                        let keyword_name = Keywords::strip(keyword.clone());
                        found.insert(keyword_name, (keyword, Self::None));
                        continue;
                    }
                }
            }
        }

        Some(found)
    }

    /// This method executes allowed functions such as: read,env
    pub fn exec(func: Self, keyword_name: String) -> Result<String, String> {
        match func {
            Self::Read => Ok(prompt(keyword_name).unwrap()),
            Self::Env => Ok(Self::env(keyword_name)),
            Self::None => Ok(Keywords::new(keyword_name, None)),
        }
    }

    /// This function reads from environment variables and returns the value as a string
    fn env(name: String) -> String {
        env::var(name).unwrap()
    }
}
