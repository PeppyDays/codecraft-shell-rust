use std::io;
use std::io::Write;
use std::process::exit;

fn main() {
    loop {
        prompt();
        let line = read();
        execute(&line);
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

static BUILTIN_COMMANDS: &[&str] = &["exit", "echo", "type"];

fn execute(line: &str) {
    let line = line.trim();
    let mut parts = line.splitn(2, ' ');
    let command = parts.next().unwrap_or("");
    let args = parts.next().unwrap_or("");

    match command {
        "exit" => exit(0),
        "echo" => println!("{}", args),
        "type" => {
            if BUILTIN_COMMANDS.contains(&args) {
                println!("{} is a shell builtin", args);
            } else {
                println!("{}: not found", args);
            }
        }
        _ => println!("{}: command not found", line),
    }
}
