use std::io;
use std::io::Write;

use codecrafters_shell::command::Command;
use codecrafters_shell::logger::get_logger;
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
    let mut parts = tokenize(line.trim());
    let standard_output_redirection_file_name = parse_redirection(&mut parts);
    let log = get_logger(standard_output_redirection_file_name.as_deref());
    if let Some(cmd) = Command::parse(&parts) {
        cmd.execute(log);
    }
}

fn parse_redirection(parts: &mut Vec<String>) -> Option<String> {
    let pos = parts.iter().position(|s| s == ">" || s == "1>")?;
    let mut rest = parts.split_off(pos);
    if rest.len() > 1 {
        Some(rest.remove(1))
    } else {
        None
    }
}
