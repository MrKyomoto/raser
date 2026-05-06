use crate::{json_error::LexError, JsonError};

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
}

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn next(&mut self) -> char {
        let c = self.chars[self.pos];
        self.pos += 1;
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
                match self.peek() {
                    Some('"') => s.push('"'),
                    Some('n') => s.push('\n'),
                    Some('t') => s.push('\t'),
                    Some('r') => s.push('\r'),
                    Some('\\') => s.push('\\'),
                    _ => return Err(JsonError::Lexer(LexError::InvalidEscape)),
                }
                self.next();
                continue;
            }
            s.push(self.next());
        }

        Err(JsonError::Lexer(LexError::UnexpectedEof))
    }

    fn read_number(&mut self) -> Result<f64, JsonError> {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            // TODO: 这里的判断数字逻辑实际上简化了,严谨处理还需要修改
            if c.is_ascii_digit() || c == '.' || c == '-' {
                s.push(self.next());
            } else {
                break;
            }
        }
        s.parse()
            .map_err(|_| JsonError::Lexer(LexError::InvalidNumber))
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
            _ => Err(JsonError::Lexer(LexError::InvalidChar(
                word.chars().next().unwrap(),
            ))),
        }
    }

    fn next_token(&mut self) -> Result<Token, JsonError> {
        self.skip_whitespace();
        match self.peek() {
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
                Ok(Token::RBracket)
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
            Some('n' | 't' | 'r') => self.read_keyword(),
            None => Err(JsonError::Lexer(LexError::UnexpectedEof)),
            Some(c) => Err(JsonError::Lexer(LexError::InvalidChar(c))),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;

    #[test]
    fn lexer_test() {
        let input = r#"{"name":"test","age":18,null:true}"#;
        let mut lex = Lexer::new(input);

        while let Ok(token) = lex.next_token() {
            println!("{:?}", token);
        }
    }
}
