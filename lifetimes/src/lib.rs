use std::fmt::Display;

pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    pub fn level(&self) -> i32 {
        3
    }

    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);

        self.part
    }
}

pub fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display {
    println!("Announcement: {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}