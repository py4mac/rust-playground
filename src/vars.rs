pub fn run() {
    let name = "Brad";

    let mut age = 10;

    println!("Name {}, Age {}", name, age);

    age = 8;

    println!("Name {}, Age {}", name, age);

    // Define constants
    const ID: i32 = 001;
    println!("ID {}", ID);

    println!("Max i32: {}",  std::i32::MAX);

    let is_activate: bool = true;
    let is_greater: bool = age < 6;

    let a1 = '\u{1F600}';

    println!("{:?}",  (is_activate, is_greater, a1));
}