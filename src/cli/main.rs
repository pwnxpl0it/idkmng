use idkmng::types::Template;
mod args;
use args::Cli;
use colored::*;

fn main() {
    let args = Cli::parse();

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
        let template = Template::validate(filename.to_string());
        println!("\n{}: {}", "Using Template".blue(), &template.magenta());
        if !args.is_present("quiet") {
            Template::show_info(&Template::parse(&template, true));
        }

        Template::extract(template, true);
    } else {
        println!(
            "{} {}",
            "No args specified please use".yellow(),
            "--help".bold().green()
        );
    }
}
