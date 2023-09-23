use std::convert::{From, TryFrom};
use std::fmt::Display;
use crate::errors::TokenParseError;

#[derive(Debug, PartialEq)]
pub enum Token {
    // Special
    Illegal,
    EOF,

    // Identifiers and literals
    Ident(String),
    String(String),
    Float(f64),
    Integer(i64),

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
        match self {
            Token::Assign => write!(f, "="),
            Token::Bang => write!(f, "!"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Lt => write!(f, "<"),
            Token::Gt => write!(f, ">"),
            Token::Eq => write!(f, "=="),
            Token::NotEq => write!(f, "!="),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Colon => write!(f, ":"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::Fn => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Return => write!(f, "return"),
            Token::EOF => write!(f, "\0"),
            Token::Illegal => write!(f, "ILLEGAL"),
            Token::Float(value) => write!(f, "{}", value),
            // NOTE: ?
            Token::Integer(value) => write!(f, "{}", value),
            Token::String(value) | Token::Ident(value) => write!(f, "{}", value),
        }
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


impl TryFrom<String> for Token {
    type Error = TokenParseError;

    fn try_from(mut value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "==" => Ok(Token::Eq),
            "!=" => Ok(Token::NotEq),
            "fn" => Ok(Token::Fn),
            "let" => Ok(Token::Let),
            "true" => Ok(Token::True),
            "false" => Ok(Token::False),
            "if" => Ok(Token::If),
            "else" => Ok(Token::Else),
            "return" => Ok(Token::Return),
            // Here, we have identifiers, strings, and numbers ints and floats.
            // If the incoming `value` starts with a number, it cannot be an identifier.
            // If value, starts with `"`, it has to be a string.
            // Otherwise, its a number. If the number literal contains a `.` its a float, otherwise an int
            _ => {
                if value.starts_with('"') {
                    value.remove(0);
                    Ok(Token::String(value))
                } else if !value.starts_with(char::is_numeric) {
                    Ok(Token::Ident(value))
                } else {
                    if value.contains('.') {
                        let parsed_value = value.parse::<f64>()?;
                        Ok(Token::Float(parsed_value))
                    } else {
                        let parsed_value = value.parse::<i64>()?;
                        Ok(Token::Integer(parsed_value))
                    }
                }
            }
        }
    }
}

