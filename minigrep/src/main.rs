use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_name = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_name);

    let mut f = File::open(file_name).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("reading file");

    println!("With text:\n{}", contents);
}
