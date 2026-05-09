use crate::{
    JsonError,
    json_error::{ErrorWithPosition, InvalidNumberType, LexError},
};

#[derive(Debug)]
pub enum Token {
    LBrace,   // {
    RBrace,   // }
    LBracket, // [
    RBracket, // ]
    Colon,    // :
    Comma,    // ,
    Null,
    True,
    False,
    Number(f64),
    String(String),
    EoF,
}

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,

    // NOTE: err check
    pub column: usize,
    pub line: usize,
}

impl Lexer {
    fn make_err(&self, err: LexError) -> JsonError {
        JsonError::Lexer(ErrorWithPosition {
            line: self.line,
            column: self.column,
            err_type: err,
        })
    }
}
impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
            line: 0,
            column: 0,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn next(&mut self) -> char {
        let c = self.chars[self.pos];
        self.pos += 1;
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        c
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_ascii_whitespace() {
                self.next();
            } else {
                break;
            }
        }
    }

    fn read_string(&mut self) -> Result<String, JsonError> {
        self.next(); // NOTE: skip \"
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c == '"' {
                self.next();
                return Ok(s);
            }

            if c == '\\' {
                self.next();
                self.translate_escape(&mut s)?;
                self.next();
                continue;
            }

            if c.is_ascii_control() {
                // return Err(JsonError::Lexer(LexError::InvalidChar(c)));
                return Err(self.make_err(LexError::InvalidChar(c)));
            }

            s.push(self.next());
        }

        // Err(JsonError::Lexer(LexError::UnexpectedEof))
        Err(self.make_err(LexError::UnexpectedEof))
    }

    fn translate_escape(&mut self, s: &mut String) -> Result<(), JsonError> {
        Ok(match self.peek() {
            Some('"') => s.push('"'),
            Some('n') => s.push('\n'),
            Some('t') => s.push('\t'),
            Some('r') => s.push('\r'),
            Some('b') => s.push('\x08'), // NOTE: backspace
            Some('f') => s.push('\x0C'), // NOTE: form feed
            Some('\\') => s.push('\\'),
            Some('/') => s.push('/'),
            Some('u') => {
                // NOTE: \u0000
                self.next();
                let ch = self.translate_escape_unicode()?;
                s.push(ch);
            }
            _ => return Err(self.make_err(LexError::InvalidEscape)),
        })
    }

    fn translate_escape_unicode(&mut self) -> Result<char, JsonError> {
        let mut hex_str = String::new();
        for i in 0..4 {
            match self.peek() {
                Some(h) if h.is_ascii_hexdigit() => {
                    hex_str.push(h);
                    if i != 3 {
                        // NOTE: 上层会调用next,所以最后一位不能再调用
                        self.next();
                    }
                }
                _ => return Err(self.make_err(LexError::InvalidUnicode)),
            }
        }
        let code = u32::from_str_radix(&hex_str, 16)
            .map_err(|_| self.make_err(LexError::InvalidUnicode))?;
        let ch = char::from_u32(code).ok_or(self.make_err(LexError::InvalidUnicode))?;
        Ok(ch)
    }

    fn read_number(&mut self) -> Result<f64, JsonError> {
        let start = self.pos;

        if let Some('-') = self.peek() {
            self.next();
        }

        match self.peek() {
            Some('0') => {
                self.next();
                if let Some(c) = self.peek() {
                    if c.is_ascii_digit() {
                        return Err(
                            self.make_err(LexError::InvalidNumber(InvalidNumberType::LeadingZero))
                        );
                    }
                }
            }
            Some(c) if c.is_ascii_digit() => {
                while let Some(c) = self.peek() {
                    if c.is_ascii_digit() {
                        self.next();
                    } else {
                        break;
                    }
                }
            }
            _ => {
                return Err(self.make_err(LexError::InvalidNumber(InvalidNumberType::InvalidChar)));
            }
        }

        if let Some('.') = self.peek() {
            self.next();
            if !self.peek().map_or(false, |c| c.is_ascii_digit()) {
                return Err(
                    self.make_err(LexError::InvalidNumber(InvalidNumberType::NoDigitsAfterDot))
                );
            }
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    self.next();
                } else {
                    break;
                }
            }
        }

        if let Some('e' | 'E') = self.peek() {
            self.next();
            if let Some('+' | '-') = self.peek() {
                self.next();
            }
            if !self.peek().map_or(false, |c| c.is_ascii_digit()) {
                return Err(self.make_err(LexError::InvalidNumber(
                    InvalidNumberType::NoDigitsAfterExponent,
                )));
            }
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    self.next();
                } else {
                    break;
                }
            }
        }

        let s: String = self.chars[start..self.pos].iter().collect();
        let num = s
            .parse()
            .map_err(|_| self.make_err(LexError::InvalidNumber(InvalidNumberType::ParseFailed)))?;

        Ok(num)
    }

    fn read_keyword(&mut self) -> Result<Token, JsonError> {
        let start = self.pos;
        while let Some(c) = self.peek() {
            if c.is_ascii_alphabetic() {
                self.next();
            } else {
                break;
            }
        }
        let word: String = self.chars[start..self.pos].iter().collect();
        match word.as_str() {
            "null" => Ok(Token::Null),
            "true" => Ok(Token::True),
            "false" => Ok(Token::False),
            _ => Err(self.make_err(LexError::InvalidChar(word.chars().next().unwrap()))),
        }
    }

    pub fn next_token(&mut self) -> Result<Token, JsonError> {
        // println!("read next token, cur char is : {:?}", self.peek());
        self.skip_whitespace();
        // println!("skip whitespace, now the char is : {:?}", self.peek());
        match self.peek() {
            Some('\0') => Ok(Token::EoF),
            Some('{') => {
                self.next();
                Ok(Token::LBrace)
            }
            Some('}') => {
                self.next();
                Ok(Token::RBrace)
            }
            Some('[') => {
                self.next();
                Ok(Token::LBracket)
            }
            Some(']') => {
                self.next();
                Ok(Token::RBracket)
            }
            Some(':') => {
                self.next();
                Ok(Token::Colon)
            }
            Some(',') => {
                self.next();
                Ok(Token::Comma)
            }
            Some('"') => Ok(Token::String(self.read_string()?)),
            Some('-' | '0'..='9') => Ok(Token::Number(self.read_number()?)),
            Some('n' | 't' | 'f') => self.read_keyword(),
            Some(c) => Err(self.make_err(LexError::InvalidChar(c))),
            None => Err(self.make_err(LexError::UnexpectedEof)),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::json_lexer::Lexer;

    #[test]
    fn lexer_test() {
        let input = r#"{"name":"test","age":18,null:true}"#;
        let mut lex = Lexer::new(input);

        while let Ok(token) = lex.next_token() {
            println!("{:?}", token);
        }
    }
}
