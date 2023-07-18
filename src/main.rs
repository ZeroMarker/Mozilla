use std::io;
use rand::Rng;
use std::cmp::Ordering;

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

  

  let secret_number = rand::thread_rng().gen_range(1..101);

  if cfg!(debug_assertions) {
    println!("Secret number: {}", secret_number);
  }
  
  loop{
    println!("Please input your guess:");
    let mut guess = String::new();
  
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
  
    println!("You guessed {guess}");
  
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        eprintln!("Invaild number, retry.");
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
