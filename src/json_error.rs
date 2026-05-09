use std::fmt;

#[derive(Debug)]
pub enum JsonError {
    Lexer(LexError),
    Parser(ParserError),
}

#[derive(Debug)]
pub enum LexError {
    InvalidChar(char),
    InvalidNumber(InvalidNumberType),
    InvalidEscape,  // NOTE: 字符转义错误(非法转义 i.e. \z \123)
    InvalidUnicode, // NOTE: Unicode转义格式错误(i.e. \u12 不够四位)
    UnexpectedEof,  // NOTE: 未闭合
}

#[derive(Debug)]
pub enum InvalidNumberType {
    LeadingZero,
    NoDigitsAfterDot,
    NoDigitsAfterExponent,
    // MultipleDots,
    // EmptyNumber,
    InvalidChar,
    ParseFailed,
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
            LexError::InvalidNumber(t) => write!(f, "invalid number: {}", t),
            LexError::InvalidEscape => write!(f, "invalid escape"),
            LexError::InvalidUnicode => write!(f, "invalid unicode"),
            LexError::UnexpectedEof => write!(f, "unexpected eof"),
        }
    }
}
impl fmt::Display for InvalidNumberType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidNumberType::LeadingZero => write!(f, "leading zero"),
            InvalidNumberType::NoDigitsAfterDot => write!(f, "no digits after dot"),
            InvalidNumberType::NoDigitsAfterExponent => write!(f, "no digits after exponent"),
            // InvalidNumberType::MultipleDots => write!(f, "multiple dots"),
            // InvalidNumberType::EmptyNumber => write!(f, "empty number"),
            InvalidNumberType::InvalidChar => write!(f, "invalid char"),
            InvalidNumberType::ParseFailed => write!(f, "parse failed"),
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
