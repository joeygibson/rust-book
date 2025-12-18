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
