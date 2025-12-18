#[derive(PartialEq)]
enum State {
    Normal,
    SingleQuoted,
    DoubleQuoted,
}

pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut state = State::Normal;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match state {
            State::Normal => match c {
                '\'' => state = State::SingleQuoted,
                '"' => state = State::DoubleQuoted,
                '\\' => {
                    if let Some(&next) = chars.peek() {
                        current.push(next);
                        chars.next();
                    }
                }
                c if c.is_whitespace() => {
                    if !current.is_empty() {
                        tokens.push(std::mem::take(&mut current));
                    }
                }
                c => current.push(c),
            },
            State::SingleQuoted => match c {
                '\'' => state = State::Normal,
                c => current.push(c),
            },
            State::DoubleQuoted => match c {
                '"' => state = State::Normal,
                '\\' => {
                    if let Some(&next) = chars.peek() {
                        if matches!(next, '"' | '\\' | '$' | '`' | '\n') {
                            current.push(next);
                            chars.next();
                        } else {
                            current.push('\\');
                        }
                    }
                }
                c => current.push(c),
            },
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}
