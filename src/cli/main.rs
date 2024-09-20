use idkmng::Config;
use idkmng::Keywords;
use idkmng::Template;
use std::fs;
mod args;
use args::Cli;
use colored::*;

fn main() {
    let args = Cli::parse();
    let config = Config::new(args.value_of("config").unwrap());
    let mut keywords = Keywords::init(config.clone());
    let mut json_data: serde_json::Value = Default::default();

    if args.is_present("json") {
        let json_file = fs::read_to_string(args.value_of("json").unwrap());
        json_data = serde_json::from_str(&json_file.unwrap()).unwrap();
    }

    if args.subcommand_matches("init").is_some() {
        let dest = format!(
            "{}.toml",
            std::env::current_dir()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
        );
        println!("{}: {}", "Creating Template".bold().green(), &dest.yellow());
        Template::generate(&dest);
    } else if let Some(filename) = args.value_of("template") {
        let template = Template::validate(filename.to_string(), config.templates_path.clone());
        println!("\n{}: {}", "Using Template".blue(), &template.magenta());
        if !args.is_present("quiet") {
            Template::show_info(&Template::parse(&template, true));
        }

        Template::extract(template, true, &mut keywords, json_data);
    } else {
        println!(
            "{} {}",
            "No args specified please use".yellow(),
            "--help".bold().green()
        );
    }
}
