use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

static BUILTIN_COMMANDS: &[&str] = &["exit", "echo", "type", "pwd", "cd"];

pub enum Command<'a> {
    Exit,
    Echo(Vec<&'a str>),
    Pwd,
    Type(&'a str),
    Cd(Option<&'a str>),
    External(&'a str, Vec<&'a str>),
}

impl<'a> Command<'a> {
    pub fn parse(parts: &'a [String]) -> Option<Command<'a>> {
        let (command, args) = parts.split_first()?;
        let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

        Some(match command.as_str() {
            "exit" => Command::Exit,
            "echo" => Command::Echo(args),
            "pwd" => Command::Pwd,
            "type" => Command::Type(args.first().copied()?),
            "cd" => {
                let target = args.first().copied();
                if target == Some("~") {
                    Command::Cd(None)
                } else {
                    Command::Cd(target)
                }
            }
            cmd => Command::External(cmd, args),
        })
    }

    pub fn execute(&self) {
        match self {
            Command::Exit => {
                std::process::exit(0);
            }
            Command::Echo(args) => {
                println!("{}", args.join(" "));
            }
            Command::Pwd => match std::env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => eprintln!("pwd: error retrieving current directory: {}", e),
            },
            Command::Type(cmd) => {
                if BUILTIN_COMMANDS.contains(cmd) {
                    println!("{} is a shell builtin", cmd);
                } else {
                    match Self::find_command_in_path(cmd) {
                        Some(path) => println!("{} is {}", cmd, path.display()),
                        None => println!("{}: not found", cmd),
                    }
                }
            }
            Command::Cd(target) => {
                let home = std::env::var("HOME").unwrap_or_default();
                let dir: &str = match target {
                    Some(t) => t,
                    None => &home,
                };
                if std::env::set_current_dir(dir).is_err() {
                    eprintln!("cd: {}: No such file or directory", dir);
                }
            }
            Command::External(cmd, args) => {
                let status = std::process::Command::new(cmd).args(args).status();
                match status {
                    Ok(status) => {
                        if !status.success() {
                            eprintln!("{}: command exited with status {}", cmd, status);
                        }
                    }
                    Err(_) => {
                        eprintln!("{}: command not found", cmd);
                    }
                }
            }
        }
    }

    fn find_command_in_path(command: &str) -> Option<PathBuf> {
        std::env::var_os("PATH").and_then(|paths| {
            std::env::split_paths(&paths).find_map(|path| {
                let full_path = path.join(command);
                (full_path.is_file() && Self::is_executable(&full_path)).then_some(full_path)
            })
        })
    }

    fn is_executable(path: &PathBuf) -> bool {
        std::fs::metadata(path)
            .map(|meta| meta.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }
}
