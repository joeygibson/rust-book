use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() {
    // let file = match File::open("input.txt") {
    //     Ok(file) => file,
    //     Err(e) => {
    //         panic!("Error opening file: {}", e);
    //     }
    // };

    // let file = File::open("input.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {error:?}");
    //         })
    //     } else {
    //         panic!("Problem opening the file: {error:?}");
    //     }
    // });

    assert_eq!(last_char_of_first_line("Hello, world\nHow are you today?"),
               Some('d'));
}

// fn read_username_from_file() -> Result<String, std::io::Error> {
//     let username_file_result = File::open("username.txt");
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut username = String::new();
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// fn read_username_from_file() -> Result<String, std::io::Error> {
//     let mut username_file = File::open("username.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// fn read_username_from_file() -> Result<String, std::io::Error> {
//     let mut username = String::new();
//     File::open("username.txt")?.read_to_string(&mut username)?;
//     Ok(username)
// }

fn read_username_from_file() -> Result<String, std::io::Error> {
    std::fs::read_to_string("username.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
