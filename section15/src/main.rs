extern crate section15;
use section15::Config;
use std::env;
use std::process;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem - {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    section15::run(config).unwrap_or_else(|err| {
        println!("Application Error - {}", err);
        process::exit(1);
    });
}

// cargo run Lorem lorem.txt
// Case_Sensitive=1 cargo run Lorem lorem.txt
