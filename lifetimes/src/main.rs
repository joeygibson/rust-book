extern crate lifetimes;

use lifetimes::*;

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("Longest: {}", result);

    println!("XXX: {}", xxx("foo"));

    struct_with_lifetime();

    let lng = longest_with_an_announcement(&string1, string2, "Bar");
    println!("LNG: {}", lng);
}

// When returning a reference from a function, the lifetime parameter for the return
// type needs to match the lifetime parameter of one of the arguments.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn xxx<'a>(x: &'a str) -> String {
    let foo = String::from("testing");

    // Need to return owned types when functions creat them
    foo
}

fn struct_with_lifetime() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a `.`.");

    let i = ImportantExcerpt { part: first_sentence };
    i.announce_and_return_part("FOO!");
}