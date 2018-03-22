use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("processing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_name);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

struct Config {
    query: String,
    file_name: String,
}

impl Config {
    // `&'static str` is the type and lifetime of string literals
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_name = args[2].clone();

        Ok(Config {
            query,
            file_name,
        })
    }
}

fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.file_name)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}