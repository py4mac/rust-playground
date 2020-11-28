pub fn run() {
    let mut hello = String::from("hello ");

    println!("Len {}", hello.len());

    hello.push('W');

    hello.push_str("orld!");

    //Capacity
    println!("Capacity {}", hello.capacity());

    println!("Is Empty {}", hello.is_empty());

    println!("Contains 'World' {}", hello.contains("World"));

    println!("Replace {}", hello.replace("World", "There"));

    for word in hello.split_ascii_whitespace() {
        println!("{}", word);
    }

    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('b');

    println!("{}", s);

    assert_eq!(s, "acb");


    println!("{}", hello);

}