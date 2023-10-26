mod guess;
type T = (i32, f64);

fn main() {
    let sum = add(3, 5);
    println!("Hello, world!");
    println!("The sum of a + b is {sum}");
    const  THREE_HOURS_SECOND: u32 = 60 * 60 * 3;

    let tup: (i32, f64, u64) = (11, 3.14, 33);
    let tuple: T = (33, 6.4);
    let pi = tup.2;

    //array
    let a = [3; 4];

    guess::guess();

}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
