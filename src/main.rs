extern crate goryl;
use goryl::interpreter;
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: goryl <file>");
    } else if args.len() == 2 {
        let content = fs::read_to_string(&args[1]);
        match content {
            Ok(code) => interpreter::run_file(code),
            Err(e) => println!("Error: {}", e),
        }
    } else if args.len() == 1 {
        interpreter::run_prompt();
    }
}
