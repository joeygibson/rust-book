fn main() {
    let s = String::from("hello");
    let s1 = &s[0..2];
    let s2 = &s[..2];   // same as above

    let len = s.len();

    let s3 = &s[3..len];
    let s4 = &s[3..]; // same as above

    let slice = &s[0..len];
    let slice1 = &s[..]; // same as above

    let s5 = String::from("hello world");
    let first = first_word(&s5);
    println!("First word: {}", first);

    let first_again = first_word(&s5[..]);

    let lit = "hello world";
    let first1 = first_word(lit);

    println!("First word1: {}", first1);
}

// defining parameter as `&str` instead of `&String` allows it to accept both
// Strings and slices
//fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

//fn first_word(s: &String) -> usize {
//    let bytes = s.as_bytes();
//
//    for (i, &item) in bytes.iter().enumerate() {
//        if item == b' ' {
//            return i;
//        }
//    }
//
//    s.len()
//}