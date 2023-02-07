mod args;
mod utils;
use utils::*;
use args::Cli;

fn main() {
    let args = Cli::parse();

    if let Some(_) = args.subcommand_matches("init"){
        Template::generate();
    }else if let Some(filename) = args.value_of("template"){
        Template::extract(filename.to_string());
    }else{
        println!("No args specified please use --help");
    }
}
