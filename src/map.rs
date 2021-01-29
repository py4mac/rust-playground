pub fn run() {
    let mut numbers = vec![1,2,3,4,5];

    //numbers[2] = 20;

    let items: Vec<_>  = numbers
        .iter()
        .map(|row| row  + 1)
        .collect();

    println!("{:?}", items);

    println!("Done");

}