use std::env;
use std::fs;
use std::io::{self, Read};
use json_schema_generator::generate_json_schema;
use serde_json::Value;

fn main() {
    let json_value = read_input().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    let schema = generate_json_schema(&json_value);

    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}

fn read_input() -> Result<Value, Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    let json_str = if args.len() > 1 {
        // Read from file
        fs::read_to_string(&args[1])?
    } else {
        // Read from stdin
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    Ok(serde_json::from_str(&json_str)?)
}
