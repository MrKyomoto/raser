use std::collections::HashMap;

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
    pub fn new(mut lex: Lexer) -> Result<Self, JsonError> {
        let cur = lex.next_token()?;
        Ok(Self { lex: lex, cur: cur })
    }

    fn next(&mut self) -> Result<(), JsonError> {
        self.cur = self.lex.next_token()?;
        Ok(())
    }

    pub fn parse_value(&mut self) -> Result<JsonValue, JsonError> {
        match &self.cur {
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
        while !matches!(self.cur, Token::RBracket) {
            arr.push(self.parse_value()?);
            if matches!(self.cur, Token::Comma) {
                self.next()?;
                if matches!(self.cur, Token::RBracket) {
                    return Err(JsonError::Parser(ParserError::TrailingComma));
                }
            }
        }
        self.next()?;
        Ok(JsonValue::Array(arr))
    }

    fn parse_object(&mut self) -> Result<JsonValue, JsonError> {
        self.next()?;
        let mut obj: HashMap<String, JsonValue> = HashMap::new();
        while !matches!(self.cur, Token::RBrace) {
            // NOTE: step 1 parse keyword
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

            if matches!(self.cur, Token::Comma) {
                self.next()?;
                if matches!(self.cur, Token::RBrace) {
                    return Err(JsonError::Parser(ParserError::TrailingComma));
                }
            }
        }
        self.next()?;
        Ok(JsonValue::Object(obj))
    }

    pub fn parse_json(input: &str) -> Result<JsonValue, JsonError> {
        let lex = Lexer::new(input);
        let mut parser = JsonParser::new(lex)?;
        let value = parser.parse_value()?;

        Ok(value)
    }
}
