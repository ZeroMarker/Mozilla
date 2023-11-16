pub fn owner() {
    /// # Ownership
    /// Heap & Stack
    ///
    /// String literal value hard code => .out
    ///
    /// drop
    ///
    /// Resource acquisition is initialization
    let s = "Hello";
    let mut str = String::from(s);
    str.push_str(" world");

    let s1 = String::from("hello");
    let s2 = s1;
    let s3 = s2.clone();
    // s1 is invalid
    // println!("{s1}");
    // reference
    let length = calculate_length(&s3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}