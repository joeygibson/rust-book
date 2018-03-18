#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect = Rectangle{width: 30, height: 50};

    println!(
        "Area of rectangle is {} square pixels",
        area(&rect)
    );

    // :? for basic Debug output
    // :#? for formatted Debug output
    println!("R: {:#?}", rect);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
