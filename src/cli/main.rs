use idkmng::Config;
use idkmng::Template;
mod args;
use args::Cli;
use colored::*;

fn main() {
    let args = Cli::parse();

    let config = Config::new(args.value_of("config").unwrap());

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

        Template::extract(template, true, config);
    } else {
        println!(
            "{} {}",
            "No args specified please use".yellow(),
            "--help".bold().green()
        );
    }
}
