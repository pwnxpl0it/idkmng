use clap::{App, Arg, Command};

pub struct Cli {}

impl Cli {
    pub fn parse() -> clap::ArgMatches {
        App::new("idkmng")
            .about("TOML based project initializer")
            .version("2.0")
            .author("Mohamed Tarek @pwnxpl0it")
            .arg(
                Arg::with_name("template")
                    .help("Template used to generate files")
                    .takes_value(true)
                    .index(1),
            )
            .arg(
                Arg::with_name("quiet")
                    .help("Hide information section of the template")
                    .short('q')
                    .requires("template"),
            )
            .subcommand(Command::new("init").about("Creates a template for the current directory"))
            .get_matches()
    }
}
