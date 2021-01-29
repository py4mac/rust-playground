use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    let command = args[1].clone();

    // println!("Args {:?}", command);
    match command.as_ref() {
        "hello" => println!("Hi how are you?"),
        _ => println!("Else")
    }
}