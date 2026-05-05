mod json_parser;
mod json_value;

use json_parser::*;
#[warn(unused_imports)]
use json_value::*;

fn main() {
    let json_parser = JsonParser::build("test".to_string());
    println!("Hello, world!");
}
