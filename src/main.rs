mod guess;

fn main() {
    let sum = add(3, 5);
    println!("Hello, world!");
    println!("The sum of a + b is {sum}");
    guess::guess();
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
