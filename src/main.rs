use std::env;
use std::fs;
use std::path::Path;
use serde_json::{Value, json, Map};

fn main() {
    // Get the filename from the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    // Read and parse the JSON file
    let json_data = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    let json_value: Value = match serde_json::from_str(&json_data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Invalid JSON: {}", e);
            std::process::exit(1);
        }
    };

    // Generate JSON schema
    let schema = generate_json_schema(&json_value);

    // Write schema to new file
    let schema_filename = format!("{}.jsonschema", Path::new(filename).file_stem().unwrap().to_str().unwrap());
    match fs::write(&schema_filename, serde_json::to_string_pretty(&schema).unwrap()) {
        Ok(_) => println!("Schema written to {}", schema_filename),
        Err(e) => eprintln!("Error writing schema file: {}", e),
    }
}

fn generate_json_schema(instance: &Value) -> Value {
    match instance {
        Value::Object(_) => generate_object_schema(instance),
        Value::Array(arr) => generate_array_schema(arr),
        Value::String(_) => json!({"type": "string"}),
        Value::Number(n) => {
            if n.is_i64() {
                json!({"type": "integer"})
            } else {
                json!({"type": "number"})
            }
        },
        Value::Bool(_) => json!({"type": "boolean"}),
        Value::Null => json!({"type": "null"}),
    }
}

fn generate_object_schema(instance: &Value) -> Value {
    let mut schema = json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {},
        "required": []
    });

    if let Value::Object(obj) = instance {
        for (key, value) in obj {
            schema["properties"][key] = generate_json_schema(value);
            schema["required"].as_array_mut().unwrap().push(Value::String(key.clone()));
        }
    }

    schema
}

fn generate_array_schema(arr: &Vec<Value>) -> Value {
    if arr.is_empty() {
        return json!({
            "type": "array",
            "items": {}
        });
    }

    let item_schemas: Vec<Value> = arr.iter().map(generate_json_schema).collect();
    let common_schema = find_common_schema(&item_schemas);

    json!({
        "type": "array",
        "items": common_schema
    })
}

fn find_common_schema(schemas: &[Value]) -> Value {
    if schemas.is_empty() {
        return json!({});
    }

    let mut common = schemas[0].clone();
    for schema in schemas.iter().skip(1) {
        common = merge_schemas(&common, schema);
    }

    common
}

fn merge_schemas(schema1: &Value, schema2: &Value) -> Value {
    if schema1 == schema2 {
        return schema1.clone();
    }

    let mut merged = json!({
        "oneOf": [schema1, schema2]
    });

    if let (Value::Object(obj1), Value::Object(obj2)) = (schema1, schema2) {
        if obj1.get("type") == obj2.get("type") {
            merged = json!({
                "type": obj1["type"].clone()
            });

            if obj1.contains_key("properties") && obj2.contains_key("properties") {
                let mut properties = Map::new();
                let props1 = obj1["properties"].as_object().unwrap();
                let props2 = obj2["properties"].as_object().unwrap();

                for (key, value) in props1.iter().chain(props2.iter()) {
                    properties.insert(key.clone(), value.clone());
                }

                merged["properties"] = Value::Object(properties);
            }
        }
    }

    merged
}

