use std::io;

pub fn read(name: String) -> String {
    println!("{}:",name.replace(
            ":read",""
            ));
    let mut input = String::from("");
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
