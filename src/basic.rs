type T = (i32, f64);

/// # Test
/// This is test.
/// ```
/// let a = test();
/// ```
///
fn test() -> i32 {
    5
}
pub fn basic() -> i32{
    let sum = add(3, test());
    println!("Hello, world!");
    println!("The sum of a + b is {sum}");

    const  THREE_HOURS_SECOND: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_SECOND);

    let tup: (i32, f64, u64) = (11, 3.14, 33);
    let tuple: T = (33, 6.4);
    println!("{}",tuple.1);

    let mut pi = tup.2;
    pi = pi + 0;
    println!("{}", pi);
    
    //array
    let a = [3; 4];

    let mut x = {
        let y = 1;
        y + 1
    };

    let y = "Hello, World!";
    println!("{y}");
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

    for i in 0..10 {
        println!("fibonacci({}) = {}", i, fibonacci(i))
    }

    result
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