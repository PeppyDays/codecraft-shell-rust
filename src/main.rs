use std::io;
use std::io::Write;

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();

    println!("{}: command not found", command.trim());
}
