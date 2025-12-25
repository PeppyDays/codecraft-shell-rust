use std::io;
use std::io::Write;

use codecrafters_shell::command::Command;
use codecrafters_shell::logger::get_stderr_logger;
use codecrafters_shell::logger::get_stdout_logger;
use codecrafters_shell::redirection::split_redirections;
use codecrafters_shell::tokenization::tokenize;

fn main() {
    loop {
        prompt();
        match read() {
            Some(line) => run(&line),
            None => break,
        }
    }
}

fn prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

fn read() -> Option<String> {
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(0) => None,
        Ok(_) => Some(line),
        Err(_) => None,
    }
}

fn run(line: &str) {
    let parts = tokenize(line.trim());
    let (command_parts, stdout_redirection_file_name, stderr_redirection_file_name) =
        split_redirections(&parts);
    let log_stdout = get_stdout_logger(stdout_redirection_file_name);
    let log_stderr = get_stderr_logger(stderr_redirection_file_name);
    if let Some(cmd) = Command::parse(command_parts) {
        cmd.execute(log_stdout, log_stderr);
    }
}
