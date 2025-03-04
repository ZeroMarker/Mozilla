use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn guess() {
    println!("Guess a number!");

    let secret_number = rand::thread_rng().gen_range(1..101); // 1..=100

    /// ## Release Environment
    if cfg!(debug_assertions) {
        println!("Secret number: {}", secret_number);
    } else {
        println!("Secret number: {}", "***");
    }

    loop {
        println!("Please input your guess:");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        println!("You guessed {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid number, retry.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big."),
            Ordering::Less => println!("Too small."),
            Ordering::Equal => {
                println!("You win.");
                break;
            }
        }
    }
}
