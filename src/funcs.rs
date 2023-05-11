use std::{io,env};

pub fn read(name: String) -> String {
    println!("{}:",name);
    let mut input = String::from("");
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn env(name: String) -> String {
    let from_env = env::var(
            name
        ).unwrap();

    from_env
}
