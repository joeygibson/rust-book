#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated functions are like static methods
    fn square(size: u32) -> Rectangle {
        Rectangle{width: size, height: size}
    }
}

fn main() {
    let rect = Rectangle{width: 30, height: 50};

    println!(
        "Area of rectangle is {} square pixels",
        rect.area()
    );

    // :? for basic Debug output
    // :#? for formatted Debug output
    println!("R: {:#?}", rect);

    let rect1 = Rectangle{width: 10, height: 10};

    println!("r1 > r? {}", rect1.can_hold(&rect));
    println!("r > r1? {}", rect.can_hold(&rect1));

    println!();

    let square = Rectangle::square(23);
    println!(
        "Area of square is {} square pixels",
        square.area()
    );
    
    println!("Square: {:#?}", square);
}
