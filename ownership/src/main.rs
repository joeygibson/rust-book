fn main() {
    // string literal; stored on the stack
//    let s = "hello";

    // String value, created from literal, stored on heap
    let mut s = String::from("hello");

    // is mutable
    s.push_str(", world!");

    println!("s: {}", s);

    let s1 = String::from("foo");
    let s2 = s1;            // this is a `move`, and `s1` is no longer valid
    let s3 = s2.clone();    // this is a deep copy

//    println!("s1: {}", s1);     // this won't compile, since `s1` is no longer valid
    println!("s2: {}", s2);
    println!("s3: {}", s3);

    ownership_example_1();
    ownership_example_2();

    reference_example_1();

    dangling_example();
}

fn ownership_example_1() {
    let s = String::from("hello");
    takes_ownership(s);
//    println!("S: {}", s); // fails, since `s` is no longer valid

    let x = 5;
    makes_copy(x);
    println!("x: {}", x);   // still works, since `i32` is `Copy`
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn ownership_example_2() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string // returned, and moved out to the caller
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("A: {}", a_string);

    a_string    // returns ownership back to caller
}

fn reference_example_1() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("Len of `{}` is {}", s1, len);

    // This won't work; compile barfs
//    let s2 = String::from("hello");
//    change(&s2);

    let mut s2 = String::from("hello");
    change(&mut s2);
}

fn dangling_example() {
    let reference_to_nothing = dangle();
}

// This doesn't work, because the String `s` is pointing to is deallocated
// when `s` goes out of scope
//fn dangle() -> &String {
//    let s = String::from("hello");
//
//    &s
//}

// This one _does_ work; it returns the string directly, transferring ownership
fn dangle() -> String {
    let s = String::from("hello");

    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // `s` goes out of scope, but since it doesn't own the string it points to,
    // no deallocation happens
}

// this does not work, since you can't change an immutable reference
//fn change(s: &String) {
//    s.push_str("foo");
//}

// this does, as long as arg is passed as `&mut`
fn change(s: &mut String) {
    s.push_str(", world");
}