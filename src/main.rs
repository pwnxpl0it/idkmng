mod args;
mod utils;
mod config;
mod keywords;
mod funcs;
use utils::*;
use colored::*;
use args::Cli;

fn main() {
    let args = Cli::parse();

    if args.subcommand_matches("init").is_some(){
        Template::generate();
    }else if let Some(filename) = args.value_of("template"){
        Template::extract(filename.to_string());
    }else{
        println!("{} {}","No args specified please use".yellow(),"--help".bold().green());
    }
}
