pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

fn main() {
    println!("Hello")
}

pub mod housing {
    pub mod money {
        pub enum Days {
            Monday,
            Tuesday,
        }
        pub fn hello() {
            crate::housing::price::line();
            super::price::line();
            self::make();
        }
        fn make() {

        }
    }
    mod price {
        pub fn line() -> i32{
            2
        }
    }
}