use dotenv::dotenv;
use std::env;

fn main() {
    // load from .env
    let result = dotenv();

    match result {
        Ok(_) => println!("Loaded .env file successfully"),
        Err(e) => println!("Error loading .env file: {}", e),
    }

    let greeting = env::var("GREETING").expect("GREETING not found");
    println!("{}", greeting);
}
