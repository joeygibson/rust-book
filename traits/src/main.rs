extern crate traits;

use traits::*;
use std::fmt::Display;
use std::fmt::Debug;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("Summary: {}", tweet.summary());
    notify(tweet);
}

fn notify<T: Summarizable>(item: T) {
    println!("Breaking news: {}", item.summary());
}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) {}

// same as above, but might be clearer
fn othr_function<T, U>(t: T, u: U)
    where T: Display + Clone,
          U: Clone + Debug {}