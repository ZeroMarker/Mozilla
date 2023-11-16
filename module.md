// In file program1.rs
pub mod my_module {
    pub fn my_function() {
        // Your code here
    }
}

// In file program2.rs
use program1::my_module; // Import the module from program1.rs

fn main() {
    my_module::my_function(); // Call the function from the imported module
}

[dependencies]
program1 = "0.1.0"

---
// max.rs
pub fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

// main.rs
mod max;

fn main() {
    let a = 10;
    let b = 20;
    let result = max::max(a, b);
    println!("The maximum of {} and {} is: {}", a, b, result);
}
