use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use json_schema_generator::generate_json_schema;
use serde_json::Value;

fn main() {
    let (input, output) = parse_args().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    let json_value = read_input(&input).unwrap_or_else(|e| {
        eprintln!("Error reading input: {}", e);
        std::process::exit(1);
    });

    let schema = generate_json_schema(&json_value);

    write_output(&output, &schema).unwrap_or_else(|e| {
        eprintln!("Error writing output: {}", e);
        std::process::exit(1);
    });
}

fn parse_args() -> Result<(Option<String>, Option<String>), String> {
    let args: Vec<String> = env::args().collect();
    let mut input = None;
    let mut output = None;
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    output = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    return Err("Output file not specified after -o/--output".to_string());
                }
            }
            arg if !arg.starts_with('-') => {
                if input.is_none() {
                    input = Some(arg.to_string());
                    i += 1;
                } else {
                    return Err("Multiple input files specified".to_string());
                }
            }
            _ => return Err(format!("Unknown argument: {}", args[i])),
        }
    }

    Ok((input, output))
}

fn read_input(input: &Option<String>) -> Result<Value, Box<dyn std::error::Error>> {
    let json_str = if let Some(filename) = input {
        fs::read_to_string(filename)?
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    Ok(serde_json::from_str(&json_str)?)
}

fn write_output(output: &Option<String>, schema: &Value) -> Result<(), Box<dyn std::error::Error>> {
    let schema_str = serde_json::to_string_pretty(schema)?;

    match output {
        Some(filename) => fs::write(filename, schema_str)?,
        None => io::stdout().write_all(schema_str.as_bytes())?,
    }

    Ok(())
}
