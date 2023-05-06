use std::{io,env};

pub fn read(name: String) -> String {
    println!("{}:",name.replace(
            ":read",""
            ));
    let mut input = String::from("");
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn env(name: String) -> String {

    let from_env = env::var(
            name
                .replace(":from_env","")
                .replace("{{$","")
        ).unwrap();

    from_env
}
