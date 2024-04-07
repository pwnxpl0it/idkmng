use colored::*;
use core::fmt;
use regex::Regex;
use std::{collections::HashMap, env, io};

#[derive(Clone, Copy)]
pub enum Fns {
    Read,
    Env,
}

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
        keyword.replace(&format!("{{$}}{}", func_name), "")
    }

    /// This method finds `functions` in a string based on a Regex pattern that matches keywords
    pub fn find(
        txt: String,
        keywords: HashMap<String, String>,
        re: Regex,
    ) -> Option<(String, String, Fns)> {
        for key in re.captures_iter(&txt) {
            if let Some(key) = key.get(0) {
                let keyword = key.as_str().to_string();

                if !keywords.contains_key(&keyword) {
                    let data = keyword.as_str().split(':').collect::<Vec<_>>();

                    if data.len() == 2 {
                        let keyword_name = data[0].replace("{{$", "").replace("}}", "");
                        let func = data[1].replace("}}", "");

                        match func.as_str() {
                            "read" => return Some((keyword_name, keyword, Fns::Read)),
                            "env" => return Some((keyword_name, keyword, Fns::Env)),
                            _ => {
                                eprintln!(
                                    "\n{}: '{}' is not a valid function",
                                    "error".red(),
                                    func.yellow()
                                );
                                return None;
                            }
                        }
                    }
                }
            }
        }

        None
    }

    /// This method executes allowed functions such as: read,env
    pub fn exec(func: Fns, keyword_name: String) -> Result<String, String> {
        match func {
            Fns::Read => Ok(Self::read(keyword_name)),
            Fns::Env => Ok(Self::env(keyword_name)),
        }
    }

    /// This function asks for user input from user
    fn read(name: String) -> String {
        println!("{}:", name);
        let mut input = String::from("");
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    /// This function reads from environment variables and returns the value as a string
    fn env(name: String) -> String {
        env::var(name).unwrap()
    }
}
