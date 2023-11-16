pub fn slice() {
    let y = "Hello, World!";
    // slice
    let hello = &y[..=4];
    println!("{}",hello);
}

fn dangle() -> String {
    let s = String::from("Hello");
    /// &s1
    s
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
// second_word(s)
// second_word(s[0..3])
fn second_word(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}