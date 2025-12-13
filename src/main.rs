use std::io;
use std::io::Write;
use std::process::exit;

fn main() {
    loop {
        prompt();
        let command = read();
        if command.trim() == "exit" {
            break;
        }
        execute(command);
    }
}

fn prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

fn read() -> String {
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    command
}

fn execute(command: String) {
    let command = command.trim();

    match command {
        "exit" => {
            exit(0);
        }
        _ => {
            println!("{}: command not found", command);
        }
    }
}
