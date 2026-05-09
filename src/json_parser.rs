use std::{collections::HashMap, error::Error, fs::File, io::Read};

use crate::{
    json_error::{JsonError, ParserError},
    json_value::JsonValue,
    lexer::{Lexer, Token},
};

pub struct JsonParser {
    lex: Lexer,
    cur: Token,
}

impl JsonParser {
    fn new(mut lex: Lexer) -> Result<Self, JsonError> {
        let cur = lex.next_token()?;
        Ok(Self { lex: lex, cur: cur })
    }

    fn next(&mut self) -> Result<(), JsonError> {
        self.cur = self.lex.next_token()?;
        Ok(())
    }

    pub fn parse_value(&mut self) -> Result<JsonValue, JsonError> {
        match &self.cur {
            Token::EoF => Ok(JsonValue::Eof),
            Token::LBrace => self.parse_object(),
            Token::LBracket => self.parse_array(),
            Token::Null => {
                self.next()?;
                Ok(JsonValue::Null)
            }
            Token::True => {
                self.next()?;
                Ok(JsonValue::Bool(true))
            }
            Token::False => {
                self.next()?;
                Ok(JsonValue::Bool(false))
            }
            Token::Number(num) => {
                let v = *num;
                self.next()?;
                Ok(JsonValue::Number(v))
            }
            Token::String(s) => {
                let v = s.clone();
                self.next()?;
                Ok(JsonValue::String(v))
            }
            _ => Err(JsonError::Parser(ParserError::UnexpectedToken)),
        }
    }

    fn parse_array(&mut self) -> Result<JsonValue, JsonError> {
        self.next()?;
        let mut arr = Vec::new();
        if !matches!(self.cur, Token::RBracket) {
            loop {
                arr.push(self.parse_value()?);
                match self.cur {
                    Token::Comma => {
                        self.next()?;
                        if matches!(self.cur, Token::RBracket) {
                            return Err(JsonError::Parser(ParserError::TrailingComma));
                        }
                    }
                    Token::RBracket => break,
                    Token::RBrace => return Err(JsonError::Parser(ParserError::MismatchBracket)),
                    _ => return Err(JsonError::Parser(ParserError::UnexpectedToken)),
                }
            }
        }

        self.next()?;
        Ok(JsonValue::Array(arr))
    }

    fn parse_object(&mut self) -> Result<JsonValue, JsonError> {
        self.next()?;
        let mut obj: HashMap<String, JsonValue> = HashMap::new();

        if !matches!(self.cur, Token::RBrace) {
            loop {
                let key = match &self.cur {
                    Token::String(s) => {
                        let k = s.clone();
                        self.next()?;
                        k
                    }
                    _ => return Err(JsonError::Parser(ParserError::KeyNotString)),
                };
                // NOTE: step 2 skip colon
                if !matches!(self.cur, Token::Colon) {
                    return Err(JsonError::Parser(ParserError::UnexpectedToken));
                }
                // NOTE: step 3 parse value
                self.next()?;
                let value = self.parse_value()?;

                obj.insert(key, value);

                match self.cur {
                    Token::Comma => {
                        self.next()?;
                        if matches!(self.cur, Token::RBrace) {
                            return Err(JsonError::Parser(ParserError::TrailingComma));
                        }
                    }
                    Token::RBrace => break,
                    Token::RBracket => return Err(JsonError::Parser(ParserError::MismatchBrace)),
                    _ => return Err(JsonError::Parser(ParserError::UnexpectedToken)),
                }
            }
        }

        self.next()?;
        Ok(JsonValue::Object(obj))
    }

    pub fn from_str(input: &str) -> Result<JsonValue, JsonError> {
        const SENTIAL: &str = "\0";
        let input = &(input.to_string() + SENTIAL);
        let lex = Lexer::new(input);
        let mut parser = JsonParser::try_from(lex)?;
        let value = JsonValue::try_from(&mut parser)?;

        Ok(value)
    }

    pub fn from_json_file(file_path: &str) -> Result<JsonValue, Box<dyn Error>> {
        let mut file = File::open(file_path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        let json_value = JsonParser::from_str(&buffer)?;

        Ok(json_value)
    }
}

impl TryFrom<Lexer> for JsonParser {
    type Error = JsonError;

    fn try_from(lex: Lexer) -> Result<Self, Self::Error> {
        JsonParser::new(lex)
    }
}
