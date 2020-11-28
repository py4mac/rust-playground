pub fn run() {
    greeting("Hello", "James");
    
    let get_sum = add(5,5);
    println!("Sum: {}", get_sum);

    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("Sum: {}", add_nums(2,2));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meetup you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}