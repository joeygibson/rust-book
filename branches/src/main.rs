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

    // while let
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top)
    }

    let v = vec![1, 2, 3];

    for (index, value) in v.iter().enumerate() {
        println!("Val at {} is {}", index, value);
    }
}
