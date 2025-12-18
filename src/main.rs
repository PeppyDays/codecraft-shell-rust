use std::io;
use std::io::Write;

use codecrafters_shell::command::Command;
use codecrafters_shell::tokenization::tokenize;

fn main() {
    loop {
        prompt();
        run(&read());
    }
}

fn prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

fn read() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

fn run(line: &str) {
    let parts = tokenize(line.trim());
    if let Some(cmd) = Command::parse(&parts) {
        cmd.execute();
    }
}
