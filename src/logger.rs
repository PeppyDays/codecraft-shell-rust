use std::fs::File;
use std::io::Write;

pub fn get_stdout_logger(stdout_redirection_file_name: Option<&str>) -> impl FnMut(&str) {
    let mut stdout_file = stdout_redirection_file_name
        .map(|path| File::create(path).expect("Unable to create log file"));

    move |text: &str| {
        log_stdout(text.trim(), stdout_file.as_mut());
    }
}

pub fn get_stderr_logger(stderr_redirection_file_name: Option<&str>) -> impl FnMut(&str) {
    let mut stderr_file = stderr_redirection_file_name
        .map(|path| File::create(path).expect("Unable to create log file"));

    move |text: &str| {
        log_stderr(text.trim(), stderr_file.as_mut());
    }
}

fn log_stdout(text: &str, stdout_redirection_file: Option<&mut File>) {
    match stdout_redirection_file {
        Some(f) => {
            writeln!(f, "{}", text).expect("Unable to write to log file");
        }
        None => {
            println!("{}", text);
        }
    }
}

fn log_stderr(text: &str, stderr_redirection_file: Option<&mut File>) {
    match stderr_redirection_file {
        Some(f) => {
            writeln!(f, "{}", text).expect("Unable to write to log file");
        }
        None => {
            println!("{}", text);
        }
    }
}
