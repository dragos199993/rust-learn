mod print;

fn main() {
    print::run();
    println!("{} Number: {}", 1, "nice");
    println!("{0} just liked {1} at {1}", "Matthew", "post");
    println!(
        "{name} likes {activity}",
        name = "Christian",
        activity = "baseball"
    );
}
