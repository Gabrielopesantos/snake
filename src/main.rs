#[derive(Debug, PartialEq)]
enum Token {
    String(String),
    Integer(i32),
    Float(f64),
    Assign,
    Plus,
    Minus,
    Multiply,
    Divide,
    Bang,
    Eq,
    NotEq,
    Semi,
    Let,
}

#[derive(Debug, PartialEq)]
struct TokenWithLoc {
    token: Token,
    start: usize,
    end: usize,
}

fn lex(input: &str) -> Result<Vec<TokenWithLoc>, &'static str> {
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();
    let mut current_start = 0;

    while let Some((idx, c)) = chars.next() {
        let mut current_end = idx + 1;

        match c {
            '0'..='9' => {
                let mut value = String::from(c);
                let mut is_float = false;
                while let Some(&(_, next)) = chars.peek() {
                    match next {
                        '0'..='9' | '.' => {
                            value.push(next);
                            current_end += 1;
                            chars.next();
                            if next == '.' {
                                // Check if `is_float` has been set before, if so,
                                // mark as an error
                                if is_float {
                                    return Err("Invalid character in input");
                                }

                                is_float = true
                            }
                        }
                        _ => break,
                    }
                }

                let token: Token;
                if is_float {
                    let parsed_value = value.parse::<f64>().unwrap();
                    token = Token::Float(parsed_value);
                } else {
                    let parsed_value = value.parse::<i32>().unwrap();
                    token = Token::Integer(parsed_value);
                }

                tokens.push(TokenWithLoc {
                    token,
                    start: current_start,
                    end: current_end,
                });
            }
            '+' => {
                tokens.push(TokenWithLoc {
                    token: Token::Plus,
                    start: current_start,
                    end: current_end,
                });
            }
            '-' => {
                tokens.push(TokenWithLoc {
                    token: Token::Minus,
                    start: current_start,
                    end: current_end,
                });
            }
            '*' => {
                tokens.push(TokenWithLoc {
                    token: Token::Multiply,
                    start: current_start,
                    end: current_end,
                });
            }
            '/' => {
                tokens.push(TokenWithLoc {
                    token: Token::Divide,
                    start: current_start,
                    end: current_end,
                });
            }
            '=' => {
                if let Some(&(_, next)) = chars.peek() {
                    if next == '=' {
                        current_end += 1;
                        tokens.push(TokenWithLoc {
                            token: Token::Eq,
                            start: current_start,
                            end: current_end,
                        });
                        chars.next();
                    } else {
                        tokens.push(TokenWithLoc {
                            token: Token::Assign,
                            start: current_start,
                            end: current_end,
                        })
                    }
                } else {
                    return Err("Invalid character in input");
                }
            }
            '!' => {
                if let Some(&(_, next)) = chars.peek() {
                    if next == '=' {
                        current_end += 1;
                        tokens.push(TokenWithLoc {
                            token: Token::NotEq,
                            start: current_start,
                            end: current_end,
                        });
                        chars.next();
                    } else {
                        tokens.push(TokenWithLoc {
                            token: Token::Bang,
                            start: current_start,
                            end: current_end,
                        })
                    }
                } else {
                    return Err("Invalid character in input");
                }
            }
            '"' => {
                let mut string_literal = String::new();
                current_start = idx; // Update start position to include the opening double quote
                while let Some(&(_, next)) = chars.peek() {
                    // Update end position to include the closing double quote
                    current_end += 1;
                    // Consume whatever char comes next, string char or closing double quote
                    chars.next();
                    if next == '"' {
                        tokens.push(TokenWithLoc {
                            token: Token::String(string_literal),
                            start: current_start,
                            end: current_end,
                        });
                        break;
                    } else {
                        string_literal.push(next)
                    }
                }
            }
            ';' => tokens.push(TokenWithLoc {
                token: Token::Semi,
                start: current_start,
                end: current_end,
            }),
            ' ' => {
                // Update start position to the next character
                current_start = idx + 1;
            }
            _ => {
                return Err("Invalid character in input");
            }
        }
    }

    Ok(tokens)
}

fn main() {
    let input = "9 + 4 * 2 - 8 / 2 \"Hello, Rust!\" 3.14 == 15 3 = 5 ! != 312 31231";
    match lex(input) {
        Ok(tokens) => {
            for token in &tokens {
                println!(
                    "Token: {:?}, Start: {}, End: {}",
                    token.token, token.start, token.end
                );
            }
        }
        Err(err) => {
            println!("Lexer error: {}", err);
        }
    }
}
