use std::{
    iter::Peekable,
    str::CharIndices
};
use std::ops::Add;
use crate::token::Token;

struct Lexer<'a> {
    input_chars: Peekable<CharIndices<'a>>,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: & 'a str) -> Self {
        let mut lexer = Lexer{
            input_chars: input.char_indices().peekable(),
            ch: '\0',
        };
        lexer.read_char();
        return lexer;
    }
}


impl Lexer<'_> {
    // FIXME: Have a better error
    fn lex(&mut self) -> Result<Vec<Token>, &'static str>{
        let mut tokens = Vec::new();
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
            _ => Token::from(self.ch)
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
            None => '\0'
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

    fn read_number(&mut self) -> String {
        let mut number = String::new();
        while let '0'..='9' | '.' = self.ch {
            number.push(self.ch);
            self.read_char();
            // NOTE: This can lead to issue if the number has more than one `.`.
            if self.ch == '.' {
                number.push('.');
                number.push_str(&self.read_number());
            }
        }
        return number;
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let 'a'..='z' | 'A'..='Z' | '_'  = self.ch {
            identifier.push(self.ch);
            self.read_char();
        }
        return identifier;
    }

    fn skip_whitespace(&mut self) {
        while let  ' ' | '\t' | '\n' | '\r' = self.ch {
            self.read_char();
        }
    }
}
