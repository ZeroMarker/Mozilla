mod guess;
type T = (i32, f64);

/// # Test
/// This is test.
/// ```
/// let a = test();
/// ```
///

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

    let mut x = {
        let y = 1;
        y + 1
    };

    let y = "Hello";
    match x {
        1 => {

        }

        _ => {}
    }

    for i in (0..a.len()).rev() {
        println!("{}", a[i]);
    }
    let result = loop {
        if x > 10 {
            break x;
        }
        x += 1;
    };
    // guess::guess();

    for i in 0..10 {
        println!("fibonacci({}) = {}", i, fibonacci(i))
    }


    /// # Ownership
    /// Heap & Stack
    ///
    /// String literal value hard code => .out
    ///
    /// drop
    ///
    /// Resource acquisition is initialization
    let s = "Hello";
    let mut str = String::from("Hello");
    str.push_str(" world");

    let s1 = String::from("hello");
    let s2 = s1;
    let s3 = s2.clone();
    // s1 is invalid
    // println!("{s1}");

    // reference
    let length = calculate_length(&s3);
}

fn test() -> i32 {
    5
}
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn fibonacci(n: u64) -> u64{
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n-2)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}