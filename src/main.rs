extern crate goryl;
use goryl::interpreter;
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len().cmp(&2) {
        std::cmp::Ordering::Less | std::cmp::Ordering::Greater => println!("Usage: goryl <file>"),
        std::cmp::Ordering::Equal => {
            let content = fs::read_to_string(&args[1]);
            match content {
                Ok(code) => interpreter::modname::run_file(code),
                Err(e) => println!("Bug: Could not read file: {}", e),
            }
        }
        _ => println!("Usage: goryl <file>"),
    }
}
