pub fn rectangle() {
    let mut rect =  Rectangle{
        width: 49,
        height: 49,
    };

    println!(
        "The area of rectangle is {} square pixels",
        rect.area()
    );

    /// {:?}
    /// ```rust
    /// println!("rect is {:#?}", rect);
    /// ```
    ///
    println!("rect is {rect:#?}");
    dbg!(&rect);

    let rec = Rectangle {
        width: 39,
        height:53,
    };
    dbg!(rect == rec);
    rect.double_width();
    // dbg!(rect);
    dbg!(rect.can_hold(&rec));
    dbg!(Rectangle::associated_fn(1));
}
#[derive(Debug)]
#[derive(PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    // self: &Self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn double_width(&mut self) {
        self.width *= 2
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width >= other.width && self.height >= other.height
    }
    // static function
    fn associated_fn(x: i32) -> i32 {
        x + 1
    }
}
