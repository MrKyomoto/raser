use std::fmt;

#[derive(Debug)]
pub enum JsonError {
    Lexer(LexError),
    Parser(ParserError),
}

#[derive(Debug)]
pub enum LexError {
    InvalidChar(char),
    InvalidNumber,
    InvalidEscape,  // NOTE: 字符转义错误(非法转义 i.e. \z \123)
    InvalidUnicode, // NOTE: Unicode转义格式错误(i.e. \u12 不够四位)
    UnexpectedEof,  // NOTE: 未闭合
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken,
    MismatchBrace,
    MismatchBracket,
    KeyNotString,
    TrailingComma,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexError::InvalidChar(c) => write!(f, "invalid char: {}", c),
            LexError::InvalidNumber => write!(f, "invalid number"),
            LexError::InvalidEscape => write!(f, "invalid escape"),
            LexError::InvalidUnicode => write!(f, "invalid unicode"),
            LexError::UnexpectedEof => write!(f, "unexpected eof"),
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::UnexpectedToken => write!(f, "unexpected token"),
            ParserError::MismatchBrace => write!(f, "mismatch brace"),
            ParserError::MismatchBracket => write!(f, "mismatch bracket"),
            ParserError::KeyNotString => write!(f, "key is not a string"),
            ParserError::TrailingComma => write!(f, "trailing comma"),
        }
    }
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonError::Lexer(lex_error) => write!(f, "lex error: {lex_error}"),
            JsonError::Parser(parser_error) => write!(f, "parser error: {parser_error}"),
        }
    }
}

impl std::error::Error for JsonError {}
