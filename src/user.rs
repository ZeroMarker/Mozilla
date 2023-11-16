use std::fmt::{Display, Formatter};

/// use string
///
/// not use &str, slice type
///
/// for get full ownership of struct member
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like struct
struct AlwaysEqual;

pub fn create_user() {
    let mut user = User {
        active: true,
        username: "Mark Chen".to_string(),
        email: "mark.chen.im@gmail.com".to_string(),
        sign_in_count: 9,
    };
    user.email = "mark@123.com".to_string();

    let mark = {
        build_user("mark".to_string(), "hello@11.com".to_string())
    };
    println!("{mark}");

    let alice = User {
        username: String::from("Alice"),
        ..mark
    };
    println!("{alice}");

    let black = (1, 3, 4);
    println!("{}", black.0);
}

fn build_user(username: String, email: String) -> User {
    let user = User {
        active: true,
        username,
        email,
        sign_in_count: 0,
    };
    user
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("username: {}", self.username).as_str())
    }
}