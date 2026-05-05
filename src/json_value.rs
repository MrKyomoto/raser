use std::collections::HashMap;

#[derive(Debug)]
pub enum JsonValue {
    Monostate,
    Double(f64),
    String(String),
    Bool(bool),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}
