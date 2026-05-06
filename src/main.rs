mod json_error;
mod json_parser;
mod json_value;
mod lexer;

use json_error::*;
use json_parser::*;
#[warn(unused_imports)]
use json_value::*;
use lexer::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let input =
    //     r#"{"name":"raser","version":1.0,"ok":true,"list":[1,2,3],"data":{"data1":1,"data2":2}"#;
    // let json = JsonParser::parse_json(input)?;
    // println!("{:#?}", json);
    // let input = r#"{}"#;
    // let json = JsonParser::parse_json(input)?;
    // println!("{:#?}", json);
    // let input = r#"[]"#;
    // let json = JsonParser::parse_json(input)?;
    // println!("{:#?}", json);
    let input = r#"[1,2,3,"hello",true,null]"#;
    let json = JsonParser::parse_json(input)?;
    println!("{:#?}", json);
    let input = r#"{"user":{"name":"test","info":{"age":20,"job":"worker"}}}"#;
    let json = JsonParser::parse_json(input)?;
    println!("{:#?}", json);
    Ok(())
}
