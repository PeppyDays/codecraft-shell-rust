use std::fs::File;
use std::io::Write;

pub fn get_logger(standard_output_redirection_file_name: Option<&str>) -> impl FnMut(&str) {
    let mut file = standard_output_redirection_file_name
        .map(|path| File::create(path).expect("Unable to create log file"));

    move |text: &str| {
        log(text.trim(), file.as_mut());
    }
}

fn log(text: &str, standard_output_redirection_file: Option<&mut File>) {
    match standard_output_redirection_file {
        Some(f) => {
            writeln!(f, "{}", text).expect("Unable to write to log file");
        }
        None => {
            println!("{}", text);
        }
    }
}
