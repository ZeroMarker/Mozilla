use std::io;
// cargo new
// cargo build
// cargo run
// cargo check
// cargo clean
fn main() {
    let sum = add(3, 5);
    println!("Hello, world!");
    println!("The sum of a + b is {sum}");
    guess();
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn guess() {
    println!("Guess a number!");

    println!("Please input your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed {guess}");
}
