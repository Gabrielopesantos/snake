#![allow(dead_code)]

use core::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, PartialEq)]
pub enum Token {
    String(String),
    Integer(i32),
    Float(f64),
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Bang,
    Lt,
    Gt,
    Eq,
    NotEq,
    Comma,
    Colon,
    Semicolon,
    Let,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Ident(String),
    Fn,
    If,
    Else,
    True,
    False,
    Return,
    EOF,
}

struct Lexer<'a> {
    input: String,
    input_chars: Peekable<CharIndices<'a>>,

    pos: u32,
    read_pos: u32,
    ch: char,

    tokens: Vec<Token>,
}

impl<'a> Lexer<'_> {
    fn new(input: String) -> Self {
        let input_chars = input.to_owned().char_indices().peekable();
        let l = Lexer {
            input: input.to_owned(),
            input_chars,

            pos: 0,
            ch: '0',
            read_pos: 0,

            tokens: Vec::new(),
        };
        l.read_char();

        return l;
    }

    // TODO: Set a proper error later;
    fn lex(&self) -> Result<(), &'static str> {
        return Ok(());
    }

    fn read_char(&self) {}
}

pub fn lex(input: &str) -> Result<Vec<Token>, &'static str> {
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();

    while let Some((_, c)) = chars.next() {
        match c {
            '0'..='9' => {
                let mut value = String::from(c);
                let mut is_float = false;
                while let Some(&(_, next)) = chars.peek() {
                    match next {
                        '0'..='9' | '.' => {
                            value.push(next);
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

                tokens.push(token);
            }
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Asterisk),
            '/' => tokens.push(Token::Slash),
            ',' => tokens.push(Token::Comma),
            ':' => tokens.push(Token::Colon),
            ';' => tokens.push(Token::Semicolon),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '[' => tokens.push(Token::LBracket),
            ']' => tokens.push(Token::RBracket),
            '{' => tokens.push(Token::LBrace),
            '}' => tokens.push(Token::RBrace),
            '<' => tokens.push(Token::Lt),
            '>' => tokens.push(Token::Gt),
            '=' => {
                if let Some(&(_, next)) = chars.peek() {
                    if next == '=' {
                        tokens.push(Token::Eq);
                        chars.next();
                    } else {
                        tokens.push(Token::Assign)
                    }
                } else {
                    return Err("Invalid character in input");
                }
            }
            '!' => {
                if let Some(&(_, next)) = chars.peek() {
                    if next == '=' {
                        tokens.push(Token::NotEq);
                        chars.next();
                    } else {
                        tokens.push(Token::Bang)
                    }
                } else {
                    return Err("Invalid character in input");
                }
            }
            '"' => {
                let mut string_literal = String::new();
                while let Some(&(_, next)) = chars.peek() {
                    chars.next();
                    if next == '"' {
                        tokens.push(Token::String(string_literal));
                        break;
                    } else {
                        string_literal.push(next)
                    }
                }
            }
            ' ' | '\t' | '\n' | '\r' => {
                // Update start position to the next character
                ()
            }
            // Identifiers or keywords
            _ => {
                if is_valid_identifier_char(c) {
                    // TODO: Same as parsing the string
                    let mut string_literal = String::from(c);
                    while let Some(&(_, next)) = chars.peek() {
                        if next == ' ' || !is_valid_identifier_char(next) {
                            match token_keyword_literal_to_enum(&string_literal) {
                                Some(token) => {
                                    tokens.push(token);
                                }
                                None => {
                                    tokens.push(Token::Ident(string_literal));
                                }
                            }
                            break;
                        } else {
                            string_literal.push(next)
                        }
                        chars.next();
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

fn is_valid_identifier_char(ch: char) -> bool {
    ch == '_' || ch.is_alphabetic()
}

#[cfg(test)]
mod tests {
    use super::{lex, Token};

    #[test]
    fn test_lexer() {
        let input = "let five = 5;
let ten = 10;

fn add(x, y) {
    return x + y;
}

let result = add(five, ten);
!-/*5;
5 < 10 > 5;


if (5 < 10) {
    return true;
} else {
    return false;
}


10 == 10.2;
10 != 9.6;
\"foobar\"
\"foo bar\"
[1, 2];
{\"foo\": \"bar\"}
";

        let expected_tokens: Vec<Token> = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Integer(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Integer(10),
            Token::Semicolon,
            Token::Fn,
            Token::Ident("add".to_string()),
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::LParen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Integer(5),
            Token::Semicolon,
            Token::Integer(5),
            Token::Lt,
            Token::Integer(10),
            Token::Gt,
            Token::Integer(5),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Integer(5),
            Token::Lt,
            Token::Integer(10),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Integer(10),
            Token::Eq,
            Token::Float(10.2),
            Token::Semicolon,
            Token::Integer(10),
            Token::NotEq,
            Token::Float(9.6),
            Token::Semicolon,
            Token::String("foobar".to_string()),
            Token::String("foo bar".to_string()),
            Token::LBracket,
            Token::Integer(1),
            Token::Comma,
            Token::Integer(2),
            Token::RBracket,
            Token::Semicolon,
            Token::LBrace,
            Token::String("foo".to_string()),
            Token::Colon,
            Token::String("bar".to_string()),
            Token::RBrace,
            Token::Semicolon,
        ];

        match lex(input) {
            Ok(tokens) => {
                for (i, token) in tokens.iter().enumerate() {
                    println!("Token: {:?}", token);
                    assert_eq!(expected_tokens[i], *token);
                }
            }
            Err(err) => {
                panic!("Lexer error: {}", err);
            }
        }
    }
}
