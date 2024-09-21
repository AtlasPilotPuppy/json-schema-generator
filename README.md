# JSON Schema Generator

This is a command-line tool that generates JSON Schema from JSON data. It's built in Rust and provides a simple way to create JSON Schema definitions from existing JSON structures.

## Features

- Generate JSON Schema from JSON files or stdin
- Support for all JSON data types (object, array, string, number, boolean, null)
- Automatic detection of required fields
- Handling of mixed-type arrays
- Output to file or stdout

## Installation

To install the JSON Schema Generator, you need to have Rust and Cargo installed on your system. Then, you can clone this repository and build the project:

```bash
git clone https://github.com/AtlasPilotPuppy/json-schema-generator.git
cd json-schema-generator
cargo build --release
```

The compiled binary will be available in `target/release/json_schema_generator`.

## Usage

```
json_schema_generator [OPTIONS] [INPUT_FILE]
```

### Options:

- `-o, --output <FILE>`: Specify the output file. If not provided, output will be written to `<INPUT_FILE>.jsonschema` or stdout if reading from stdin.
- `-s, --stdout`: Force output to stdout, even when an input file is provided.
- `-h, --help`: Print help information.

### Examples:

1. Generate schema from a file:

   ```
   json_schema_generator input.json
   ```

2. Generate schema from a file and save to a specific output file:

   ```
   json_schema_generator input.json -o output_schema.json
   ```

3. Generate schema from stdin:

   ```
   cat input.json | json_schema_generator
   ```

4. Generate schema from a file and output to stdout:

   ```
   json_schema_generator input.json --stdout
   ```

## JSON Schema Version

This tool generates JSON Schema compatible with draft-07.

## Limitations

- The tool generates a basic schema and may not capture all possible constraints or patterns in your data.
- It doesn't handle `$ref` references or complex validation rules.
- The schema for arrays assumes all items in the array follow the same schema.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

