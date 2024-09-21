use clap::Parser;
use json_schema_generator::generate_json_schema;
use serde_json::Value;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Input file name
    input: Option<String>,

    /// Output file name
    #[clap(short, long)]
    output: Option<String>,

    /// Output to stdout
    #[clap(short, long)]
    stdout: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let json_value = read_input(&cli.input)?;
    let schema = generate_json_schema(&json_value);
    write_output(&cli, &schema)?;

    Ok(())
}

fn read_input(input: &Option<String>) -> io::Result<Value> {
    let json_str = match input {
        Some(filename) => fs::read_to_string(filename)?,
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        }
    };

    serde_json::from_str(&json_str).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn write_output(cli: &Cli, schema: &Value) -> io::Result<()> {
    let schema_str = serde_json::to_string_pretty(schema)?;

    if cli.stdout {
        println!("{}", schema_str);
    } else if let Some(output_file) = &cli.output {
        fs::write(output_file, &schema_str)?;
    } else if let Some(input_file) = &cli.input {
        let output_file = format!(
            "{}.jsonschema",
            Path::new(input_file).file_stem().unwrap().to_str().unwrap()
        );
        fs::write(output_file, &schema_str)?;
    } else {
        println!("{}", schema_str);
    }

    Ok(())
}
