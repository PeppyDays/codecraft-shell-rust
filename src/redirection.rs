const REDIRECTION_OPERATORS: [&str; 2] = [">", "1>"];

pub fn split_redirections(mut parts: Vec<String>) -> (Vec<String>, Option<String>) {
    let pos = parts
        .iter()
        .position(|s| REDIRECTION_OPERATORS.contains(&s.as_str()));

    match pos {
        Some(pos) => {
            let mut rest = parts.split_off(pos);
            if rest.len() > 1 {
                (parts, Some(rest.remove(1)))
            } else {
                (parts, None)
            }
        }
        None => (parts, None),
    }
}
