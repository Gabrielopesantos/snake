use std::fmt::Display;

pub enum Token {
    // Special
    Illegal,
    EOF,

    // Identifiers and literals
    Ident(String),
    String(String),
    Float(f64),
    Int(i64),

    // Operators
    Assign,
    Bang,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    NotEq,

    // Delimiters
    Comma,
    Semicolon,
    Colon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    // Keywords
    Fn,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<char> for Token {
    fn from(ch: char) -> Self {
        match ch {
            '=' => Token::Assign,
            '!' => Token::Bang,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '<' => Token::Lt,
            '>' => Token::Gt,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            ':' => Token::Colon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            '\0' => Token::EOF,
            _ => Token::Illegal,
        }
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        match value.as_str() {
            "==" => Token::Eq,
            "!=" => Token::NotEq,
            "fn" => Token::Fn,
            "let" => Token::Let,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            // Here, we have identifiers, strings, and numbers ints and floats.
            // If the incoming `value` starts with a number, it cannot be an identifier.
            // If value, starts with `"`, it has to be a string.
            //
            // FIXME
            _ => {
                if value.starts_with('"') {
                    Token::String(value)
                } else if !value.starts_with(char::is_numeric) {
                    Token::Ident(value)
                } else {
                    if value.contains('.') {
                        // FIXME
                        Token::Float(3.14)
                    } else {
                        // FIXME
                        Token::Int(3)
                    }
                }
            },
        }
    }
}
