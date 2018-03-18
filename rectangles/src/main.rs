fn main() {
    let rect = (30, 50);

    println!(
        "Area of rectangle is {} square pixels",
        area(rect)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
