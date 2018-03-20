extern crate error_handling;

use std::fs::File;
use std::io;
use std::io::Read;
use std::net::IpAddr;
use error_handling::Guess;

fn main() {
//    panic!("Crash and burn!");

//    let v = vec![1, 2, 3];
//
//    v[99];

//    let f = File::open("hello.txt");
//    let f = match f {
//        Ok(file) => file,
//        Err(error) => {
//            panic!("opening file: {}", error);
//        },
//    };

//    let f = match File::open("hello.txt") {
//        Ok(file) => file,
//        Err(error) => panic!("opening file: {}", error),
//    };
//
//    println!("{:?}", f);

    let file_name = "hello.txt";

//    let f = File::open(&file_name);
//    let f = match f {
//        Ok(file) => file,
//        Err(ref error) if error.kind() == ErrorKind::NotFound => {
//            match File::create(&file_name) {
//                Ok(fc) => fc,
//                Err(e) => {
//                    panic!("tried creating: {}", e)
//                },
//            }
//        },
//        Err(error) => {
//            panic!("opening file: {}", error);
//        }
//    };

//    let f = File::open(&file_name).unwrap();
//    let f = File::open(&file_name).expect("opening file");

    custom_code();
}

//fn read_username_from_file() -> Result<String, io::Error> {
//    let f = File::open("hello.txt");
//
//    let mut f = match f {
//        Ok(file) => file,
//        Err(e) => return Err(e),
//    };
//
//    let mut s = String::new();
//
//    match f.read_to_string(&mut s) {
//        Ok(_) => Ok(s),
//        Err(e) => Err(e),
//    }
//}

// same as above, but shortcuts
//fn read_username_from_file() -> Result<String, io::Error> {
//    let mut f = File::open("hello.txt")?; // the `?` will return error if failure
//    let mut s = String::new();
//    f.read_to_string(&s)?;
//
//    Ok(s)
//}

// even shorter
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn its_ok_to_unwrap_in_limited_cases() {
    // Since we know that 127.0.0.1 is a valid IP address, it's OK to ignore the possible error
    let ip = "127.0.0.1".parse::<IpAddr>().unwrap();
}

fn custom_code() {
    let g = Guess::new(23);
    println!("{:?}", g.value());

    let g = Guess::new(0);
    println!("{:?}", g);
}