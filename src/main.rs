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
    Comma,
    Colon,
    Semicolon,
    Let,
    RParen,
    LParen,
    RBracket,
    LBracket,
    RBrace,
    LBrace,
    Ident(String),
    Fn,
    If,
    Else,
    True,
    False,
    Return,
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
            ',' => tokens.push(TokenWithLoc {
                token: Token::Comma,
                start: current_start,
                end: current_end,
            }),
            ':' => tokens.push(TokenWithLoc {
                token: Token::Colon,
                start: current_start,
                end: current_end,
            }),
            ';' => tokens.push(TokenWithLoc {
                token: Token::Semicolon,
                start: current_start,
                end: current_end,
            }),
            '(' => {
                tokens.push(TokenWithLoc {
                    token: Token::RParen,
                    start: current_start,
                    end: current_end,
                });
            }
            ')' => {
                tokens.push(TokenWithLoc {
                    token: Token::LParen,
                    start: current_start,
                    end: current_end,
                });
            }
            '[' => {
                tokens.push(TokenWithLoc {
                    token: Token::RBracket,
                    start: current_start,
                    end: current_end,
                });
            }
            ']' => {
                tokens.push(TokenWithLoc {
                    token: Token::LBracket,
                    start: current_start,
                    end: current_end,
                });
            }
            '{' => {
                tokens.push(TokenWithLoc {
                    token: Token::RBrace,
                    start: current_start,
                    end: current_end,
                });
            }
            '}' => {
                tokens.push(TokenWithLoc {
                    token: Token::LBrace,
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
            ' ' | '\t' | '\n' | '\r' => {
                // Update start position to the next character
                current_start = idx + 1;
            }
            // Identifiers or keywords
            _ => {
                if c.is_alphabetic() || c == '_' {
                    // TODO: Same as parsing the string
                    let mut string_literal = String::from(c);
                    // ??????????????
                    current_start = idx; // Update start position to include the first character
                    while let Some(&(_, next)) = chars.peek() {
                        current_end += 1;
                        chars.next();
                        if next == ' ' {
                            // NOTE: Still not checking if there's in an invalid char
                            // in the keyword or identifier
                            match token_keyword_literal_to_enum(&string_literal) {
                                Some(token) => {
                                    tokens.push(TokenWithLoc {
                                        token,
                                        start: current_start,
                                        end: current_end,
                                    });
                                }
                                None => {
                                    tokens.push(TokenWithLoc {
                                        token: Token::Ident(string_literal),
                                        start: current_start,
                                        end: current_end,
                                    });
                                }
                            }
                            break;
                        } else {
                            string_literal.push(next)
                        }
                    }
                } else {
                    return Err("Invalid character in input");
                }
            }
        }
    }

    Ok(tokens)
}

fn token_keyword_literal_to_enum(token_literal: &str) -> Option<Token> {
    match token_literal {
        "let" => Some(Token::Let),
        "fn" => Some(Token::Fn),
        "if" => Some(Token::If),
        "else" => Some(Token::Else),
        "true" => Some(Token::True),
        "false" => Some(Token::False),
        "return" => Some(Token::Return),
        _ => None,
    }
}

fn main() {
    let input = "
let x = 3 * 2;
fn add_two(number: int) {
    return number + 2;
}
print(x);
";
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
