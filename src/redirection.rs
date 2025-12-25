const STDOUT_REDIRECTION_OPERATORS: [&str; 2] = [">", "1>"];
const STDERR_REDIRECTION_OPERATORS: [&str; 1] = ["2>"];

pub fn split_redirections(parts: &[String]) -> (&[String], Option<&str>, Option<&str>) {
    let redirect_idx = parts
        .iter()
        .position(|p| is_redirection_operator(p))
        .unwrap_or(parts.len());

    let (command_parts, redirect_parts) = parts.split_at(redirect_idx);

    let mut stdout_redirection_file_name = None;
    let mut stderr_redirection_file_name = None;

    let mut i = 0;
    while i < redirect_parts.len() {
        let part = redirect_parts[i].as_str();

        if STDOUT_REDIRECTION_OPERATORS.contains(&part) {
            if i + 1 < redirect_parts.len() {
                stdout_redirection_file_name = Some(redirect_parts[i + 1].as_str());
            }
            i += 2;
        } else if STDERR_REDIRECTION_OPERATORS.contains(&part) {
            if i + 1 < redirect_parts.len() {
                stderr_redirection_file_name = Some(redirect_parts[i + 1].as_str());
            }
            i += 2;
        } else {
            i += 1;
        }
    }

    (
        command_parts,
        stdout_redirection_file_name,
        stderr_redirection_file_name,
    )
}

fn is_redirection_operator(s: &str) -> bool {
    STDOUT_REDIRECTION_OPERATORS.contains(&s) || STDERR_REDIRECTION_OPERATORS.contains(&s)
}
