use std::env;
use std::process;
use image_convert::Config;

fn main() {
    println!("Hello, world!");
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
}
