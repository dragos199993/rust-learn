pub fn run() {
    let mut numbers: [i32; 4] = [1, 2, 3, 4];

    println!("{}", numbers[2]);

    numbers[2] = 20;

    println!("{}", numbers[2]);

    println!("{}", std::mem::size_of_val(&numbers));
}
