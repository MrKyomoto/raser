use std::{collections::HashMap, fmt};

#[derive(Debug)]
pub enum JsonValue {
    Null,
    Number(f64),
    String(String),
    Bool(bool),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonValue::Null => write!(f, "null"),
            JsonValue::Number(num) => write!(f, "{}", num),
            JsonValue::String(s) => write!(f, "\"{}\"", s),
            JsonValue::Bool(booler) => write!(f, "{}", booler),
            JsonValue::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            JsonValue::Object(obj) => {
                write!(f, "{{")?;
                for (i, (k, v)) in obj.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "\"{}\":{}", k, v)?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl JsonValue {
    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }
    pub fn is_number(&self) -> bool {
        matches!(self, JsonValue::Number(_))
    }
    pub fn is_string(&self) -> bool {
        matches!(self, JsonValue::String(_))
    }
    pub fn is_bool(&self) -> bool {
        matches!(self, JsonValue::Bool(_))
    }
    pub fn is_array(&self) -> bool {
        matches!(self, JsonValue::Array(_))
    }
    pub fn is_object(&self) -> bool {
        matches!(self, JsonValue::Object(_))
    }

    pub fn as_num(&self) -> Option<f64> {
        if let JsonValue::Number(num) = self {
            Some(*num)
        } else {
            None
        }
    }
    pub fn as_str(&self) -> Option<&String> {
        if let JsonValue::String(s) = self {
            Some(s)
        } else {
            None
        }
    }
    pub fn as_array(&self) -> Option<&Vec<JsonValue>> {
        if let JsonValue::Array(arr) = self {
            Some(arr)
        } else {
            None
        }
    }
    pub fn as_object(&self) -> Option<&HashMap<String, JsonValue>> {
        if let JsonValue::Object(obj) = self {
            Some(obj)
        } else {
            None
        }
    }
}
