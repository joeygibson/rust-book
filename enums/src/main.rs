// first verison
//enum IpAddrKind {
//    V4,
//    V6
//}
//
//struct IpAddr {
//    kind: IpAddrKind,
//    address: String
//}

// more concise
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
    Georgia,
}

fn main() {
//    let four = IpAddrKind::V4;
//    let six = IpAddrKind::V6;

//    route(four);
//    route(six);

    let home = IpAddr::V4(10, 10, 20, 30);

    let loopback = IpAddr::V6(String::from("::1"));

    let some_number = Some(23);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // placeholder to make match exhaustive
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => ()
    }

    // This is a more concise match, when we only care about one thing
    let some_u8_value = Some(3);
    if let Some(3) = some_u8_value {
        println!("THREE");
    } else {
        println!("Something else");
    }
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter: {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(num) => Some(num + 1)
    }
}


//fn route(ip_type: IpAddrKind) {
//
//}