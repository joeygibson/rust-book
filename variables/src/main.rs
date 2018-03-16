fn main() {
    let mut x = 5;
    println!("The value of x is {}.", x);

    x = 6;
    println!("The value of x is {}.", x);

    let y = 23;
    let y = 99;

    println!("The value of y is {}.", y);

    let tup = (1, 23_000, 'a', "foo");
    let z = tup.3;

    println!("The value of z is {}.", z);

    let (x, y, z, a) = tup;
    println!("z = {}", z);

    let arr = [5, 4, 3, 2, 1];
}
