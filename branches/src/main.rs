fn main() {
    let condition = true;

    let number = if condition {
        5
    } else {
        6
    };

    println!("Number: {}", number);

    let a = [10, 20, 30, 40, 50];

    for el in a.iter() {
        println!("El: {}", el);
    }

    for el in (1..4).rev() {
        println!("{}!", el);
    }
}
