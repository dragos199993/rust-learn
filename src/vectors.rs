pub fn run() {
    let mut vector_numbers: Vec<i32> = vec![1, 2, 3, 4];

    println!("{:?}", vector_numbers);
    vector_numbers.push(20);
    println!("{:?}", vector_numbers);
    println!("{:?}", vector_numbers.len());
    vector_numbers.pop();
    println!("{:?}", vector_numbers.len());
}
