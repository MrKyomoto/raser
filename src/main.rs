mod json_error;
mod json_lexer;
mod json_parser;
mod json_value;

use std::{error::Error, fs::read_dir};

use json_error::*;
use json_parser::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_test()?;
    number_test()?;

    let tests_dir = "test_json";
    json_file_test(tests_dir)?;

    Ok(())
}

fn json_file_test(tests_dir: &'static str) -> Result<(), Box<dyn Error>> {
    Ok(for entry in read_dir(tests_dir)? {
        let entry = entry?;
        let json_path = entry.path();

        println!("Test case {:#}:", entry.file_name().into_string().unwrap());
        let json = JsonParser::from_json_file(json_path.as_os_str().to_str().unwrap())?;
        println!("{}\n", json);
    })
}

fn simple_test() -> Result<(), Box<dyn Error>> {
    let test_cases = vec![
        "null",
        "true",
        "false",
        "0",
        "123",
        "3.1415",
        "-456",
        "-0.01",
        "[]",
        "{}",
        "\"hello\"",
        "\"a\"",
        "\"test\\n\\t\\\"escaped\"",
        "\"test\\u12abc\"",
        "\"hello\\u1world\"",
        // "\"demo\\u123xyz\"",
        // "\"bad\\u12g3code\"",
        // "\"wrong\\uGGGGhere\"",
        //
    ];

    Ok(for (i, &input) in test_cases.iter().enumerate() {
        println!("Test case {:#}:", i);
        let json = JsonParser::from_str(input)?;
        println!("{}\n", json);
    })
}

fn number_test() -> Result<(), Box<dyn Error>> {
    let valid_numbers = vec![
        "0",
        "1",
        "123",
        "9999999",
        "-0",
        "-1",
        "-123",
        "0.0",
        "0.5",
        "123.456",
        "1.0",
        "-0.5",
        "-123.456",
        "1e0",
        "1e5",
        "1E5",
        "123e-10",
        "123E+10",
        "0.1e3",
        "-1.23e4",
        "-1.23E-4",
        "1234567890123456789",
        "-1234567890123456789",
        "0.123456789",
        "-0.123456789",
    ];
    Ok(for (i, &input) in valid_numbers.iter().enumerate() {
        println!("Test case {:#}:", i);
        let json = JsonParser::from_str(input)?;
        println!("{}\n", json);
    })
}
