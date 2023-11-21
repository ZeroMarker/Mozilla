pub fn message() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let z = match y {
        None => x,
        Some(y) => x + y
    };

    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!",state)
    }
    else {
        count = count + 1
    }
}

#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => {
                println!("Quit!")
            }
            Message::Move { x, y } => {
                println!("Move:({x}, {y})")
            }
            Message::Write(value) => {
                println!("Write a value: {value}")
            }
            Message::ChangeColor(r, g, b) => {
                println!("R: {r}, G: {g}, B: {b}")
            }
        }
    }
}