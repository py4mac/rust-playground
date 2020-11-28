use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    numbers[2] = 20;

    println!("{:?}", numbers);

    println!("Len {}", numbers.len());

    println!("Array occupied {} bytes", mem::size_of_val(&numbers));


    let slice: &[i32] = &numbers[0..2];

    println!("{:?}", slice);

}