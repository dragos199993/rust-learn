pub fn run() {
    let mut abc = String::from("Hello");

    println!("Max 32 {}, {}", std::i32::MAX, abc.len());

    abc.push('W');
    println!("New {}, {}", abc, abc.len());
    abc.push_str(" Test");
    println!("New {}, {}", abc, abc.len());
}
