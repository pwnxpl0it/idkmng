use clap::{App, Arg,Command};

pub struct Cli {}

impl Cli{
    pub fn parse() -> clap::ArgMatches{
        App::new("idkmng")
            .about("TOML based project initializer")
            .version("0.1")
            .author("Mohamed Tarek @0xr00t3d")
            .arg(Arg::with_name("template")
                 .help("Template used to generate files")
                 .takes_value(true)
                 .index(1)
            )
            .subcommand(Command::new("init")
                        .about("Creates a template for the current directory")
                        )
            .get_matches()
    }
}