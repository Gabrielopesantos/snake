use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug)]
pub enum LexerError {
    TokenParseError(TokenParseError),
}

impl From<TokenParseError> for LexerError {
    fn from(error: TokenParseError) -> Self {
        LexerError::TokenParseError(error)
    }
}

#[derive(Debug)]
pub enum TokenParseError {
    InvalidInput, // NOTE: Not used for anything for now
    ParseError(ParseError),
}

impl From<ParseFloatError> for TokenParseError {
    fn from(error: ParseFloatError) -> Self {
        TokenParseError::ParseError(ParseError::from(error))
    }
}

impl From<ParseIntError> for TokenParseError {
    fn from(error: ParseIntError) -> Self {
        TokenParseError::ParseError(ParseError::from(error))
    }
}

#[derive(Debug)]
pub enum ParseError {
    IntError(ParseIntError),
    FloatError(ParseFloatError),
}

impl From<ParseIntError> for ParseError {
    fn from(error: ParseIntError) -> Self {
        ParseError::IntError(error)
    }
}

impl From<ParseFloatError> for ParseError {
    fn from(error: ParseFloatError) -> Self {
        ParseError::FloatError(error)
    }
}
