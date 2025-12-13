use std::io;
use std::io::Write;

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();
}
