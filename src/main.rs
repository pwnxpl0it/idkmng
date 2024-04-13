use idkmng::types::Template;
mod args;
use args::Cli;
use colored::*;

fn main() {
    let args = Cli::parse();

    if args.subcommand_matches("init").is_some() {
        Template::generate();
    } else if let Some(filename) = args.value_of("template") {
        Template::extract(filename.to_string(), true);
    } else {
        println!(
            "{} {}",
            "No args specified please use".yellow(),
            "--help".bold().green()
        );
    }
}
