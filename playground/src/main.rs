struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);
    println!("The length of '{}' is {}.", s1, calculate_length(&s1));

    let s2 = String::from("this is a test");

    let word = first_word(&s2);
    println!("the first word is: {}", word);

    let foo = "vini vidi vici";
    let word = first_word(foo);
    println!("the first word is: {}", word);

    let user = build_user(String::from("foo@foo.com"), String::from("foo"));

    println!("User email: {}", user.email);

    let user2 = User {
        email: String::from("bar@bar.com"),
        ..user
    };

    println!("User2 email: {}", user2.email);

    let red = Color(255, 0, 0);
    println!("Red: {}", red);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
