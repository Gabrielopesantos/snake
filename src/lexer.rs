use crate::token::Token;
use std::{iter::Peekable, str::CharIndices};

struct Lexer<'a> {
    input_chars: Peekable<CharIndices<'a>>,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input_chars: input.char_indices().peekable(),
            ch: '\0',
        };
        lexer.read_char();
        return lexer;
    }
}

impl Lexer<'_> {
    // FIXME: Have a better error
    fn lex(&mut self) -> Result<Vec<Token>, &'static str> {
        let mut tokens = Vec::new();
        // FIXME: Irrefutable `while let` pattern
        while let token = self.next_token() {
            if token == Token::EOF {
                break;
            }
            tokens.push(token);
        }
        return Ok(tokens);
    }

    // FIXME: Should return a Result
    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            '0'..='9' => Token::from(self.read_number()),
            'a'..='z' | 'A'..='Z' | '_' => Token::from(self.read_identifier()),
            '!' | '=' => {
                if self.peek_char() != '=' {
                    Token::from(self.ch)
                } else {
                    let ch = self.ch;
                    // FIXME: What if this char is '\0'?
                    self.read_char();
                    Token::from(format!("{ch}{}", self.ch))
                }
            }
            '"' => Token::from(self.read_string()),
            _ => Token::from(self.ch),
        };

        self.read_char();
        return token;
    }

    fn read_char(&mut self) {
        self.ch = self.peek_char();
        self.input_chars.next();
    }

    fn peek_char(&mut self) -> char {
        match self.input_chars.peek() {
            Some((_, ch)) => *ch,
            None => '\0',
        }
    }

    // FIXME: This function doesn't take into account additional '"' chars in strings neither and
    // invalid end of input.
    fn read_string(&mut self) -> String {
        let mut string = String::from(self.ch);
        self.read_char();

        while self.ch != '"' {
            string.push(self.ch);
            self.read_char();
        }
        return string;
    }

    // FIXME: Rethink this function
    // Given where this function is called, expect the recursive call, there's no way for the first
    // char to be a `.`.
    fn read_number(&mut self) -> String {
        let mut number = String::from(self.ch);
        while let '0'..='9' | '.' = self.peek_char() {
            self.read_char();
            number.push(self.ch);
        }
        return number;
    }

    // FIXME: Rethink this function
    fn read_identifier(&mut self) -> String {
        let mut identifier = String::from(self.ch);
        while let 'a'..='z' | 'A'..='Z' | '_' = self.peek_char() {
            self.read_char();
            identifier.push(self.ch);
        }
        return identifier;
    }

    fn skip_whitespace(&mut self) {
        while let ' ' | '\t' | '\n' | '\r' = self.ch {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Token};

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
{\"foo\": \"bar\"};
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

        let mut lexer = Lexer::new(input);

        for (i, expected_token) in expected_tokens.into_iter().enumerate() {
            let actual_token = lexer.next_token();
            assert_eq!(
                expected_token, actual_token,
                "Test {}: tokens did not match, expected: {}. Got: {}",
                i, expected_token, actual_token
            )
        }
    }
}

