use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

const EXEC_PERMISSION_MASK: u32 = 0o111;

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

    pub fn execute(&self, log: impl FnMut(&str)) {
        match self {
            Command::Exit => Self::execute_exit(),
            Command::Echo(args) => Self::execute_echo(args, log),
            Command::Pwd => Self::execute_pwd(log),
            Command::Type(cmd) => Self::execute_type(cmd, log),
            Command::Cd(target) => Self::execute_cd(target),
            Command::External(cmd, args) => Self::execute_external(cmd, args, log),
        }
    }

    fn execute_exit() -> ! {
        std::process::exit(0)
    }

    fn execute_echo(args: &[&str], mut log: impl FnMut(&str)) {
        log(&args.join(" "));
    }

    fn execute_pwd(mut log: impl FnMut(&str)) {
        match std::env::current_dir() {
            Ok(path) => log(path.display().to_string().as_str()),
            Err(e) => eprintln!("pwd: error retrieving current directory: {}", e),
        }
    }

    fn execute_type(cmd: &str, mut log: impl FnMut(&str)) {
        if BUILTIN_COMMANDS.contains(&cmd) {
            log(&format!("{} is a shell builtin", cmd));
        } else {
            match Self::find_command_in_path(cmd) {
                Some(path) => log(&format!("{} is {}", cmd, path.display())),
                None => log(&format!("{}: not found", cmd)),
            }
        }
    }

    fn execute_cd(target: &Option<&str>) {
        let home = std::env::var("HOME").unwrap_or_default();
        let dir: &str = match target {
            Some(t) => t,
            None => &home,
        };
        if std::env::set_current_dir(dir).is_err() {
            eprintln!("cd: {}: No such file or directory", dir);
        }
    }

    fn execute_external(cmd: &str, args: &[&str], mut log: impl FnMut(&str)) {
        let output = std::process::Command::new(cmd).args(args).output();
        match output {
            Ok(output) => {
                if !output.stdout.is_empty()
                    && let Ok(stdout) = String::from_utf8(output.stdout)
                {
                    log(&stdout)
                }
                if !output.stderr.is_empty()
                    && let Ok(stderr) = String::from_utf8(output.stderr)
                {
                    eprintln!("{}", stderr.trim());
                }
            }
            Err(_) => {
                eprintln!("{}: command not found", cmd);
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

    fn is_executable(path: &Path) -> bool {
        std::fs::metadata(path)
            .map(|meta| meta.permissions().mode() & EXEC_PERMISSION_MASK != 0)
            .unwrap_or(false)
    }
}
