use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::os::unix::prelude::PermissionsExt;
use std::path::PathBuf;
use std::process;
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
                match find_command_in_path(args) {
                    Some(path) => println!("{} is {}", args, path.to_string_lossy()),
                    None => println!("{}: not found", args),
                }
            }
        }
        _ => {
            find_command_in_path(command).map_or_else(
                || println!("{}: command not found", command),
                |_path| {
                    process::Command::new(command)
                        .args(args.split_whitespace())
                        .status()
                        .unwrap_or_else(|e| {
                            eprintln!("Failed to execute {}: {}", command, e);
                            process::exit(1);
                        });
                },
            );
        }
    }
}

fn find_command_in_path(command: &str) -> Option<PathBuf> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).find_map(|path| {
            let full_path = path.join(command);
            (full_path.is_file() && is_executable(&full_path)).then_some(full_path)
        })
    })
}

fn is_executable(path: &PathBuf) -> bool {
    fs::metadata(path)
        .map(|meta| meta.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}
