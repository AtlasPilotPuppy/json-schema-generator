//! This module provides functionality to generate JSON schemas from JSON instances.
//! It supports various JSON types including objects, arrays, strings, numbers, booleans, and null values.

use serde_json::{json, Map, Value};

/// Generates a JSON schema for a given JSON instance.
///
/// # Arguments
///
/// * `instance` - A reference to a `serde_json::Value` representing the JSON instance.
///
/// # Returns
///
/// A `serde_json::Value` representing the JSON schema for the given instance.
///
/// # Examples
///
/// ```
/// use serde_json::json;
/// use json_schema_generator::generate_json_schema;
///
/// let instance = json!({"name": "John", "age": 30});
/// let schema = generate_json_schema(&instance);
///
/// assert_eq!(schema, json!({
/// "$schema": "http://json-schema.org/draft-07/schema#",
///     "type": "object",
///     "properties": {
///         "name": {"type": "string"},
///         "age": {"type": "integer"}
///     },
///     "required": ["age", "name"]
/// }));
/// ```
pub fn generate_json_schema(instance: &Value) -> Value {
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
        }
        Value::Bool(_) => json!({"type": "boolean"}),
        Value::Null => json!({"type": "null"}),
    }
}

fn generate_object_schema(instance: &Value) -> Value {
    let mut schema = json!({
        "type": "object",
        "properties": {},
        "required": []
    });

    if let Value::Object(obj) = instance {
        for (key, value) in obj {
            if key == "$ref" {
                schema["$ref"] = value.clone();
            } else {
                let mut sub_schema = generate_json_schema(value);
                if let Some(obj) = sub_schema.as_object_mut() {
                    obj.remove("$schema"); // Remove $schema from nested objects
                }
                schema["properties"][key] = sub_schema;
                schema["required"]
                    .as_array_mut()
                    .unwrap()
                    .push(Value::String(key.clone()));
            }
        }
    }

    // Sort the "required" array
    if let Some(required) = schema["required"].as_array_mut() {
        required.sort_by(|a, b| a.as_str().unwrap().cmp(b.as_str().unwrap()));
    }

    // Add $schema only to the top-level object
    schema["$schema"] = json!("http://json-schema.org/draft-07/schema#");

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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_generate_json_schema_string() {
        let input = json!("test");
        let expected = json!({"type": "string"});
        assert_eq!(generate_json_schema(&input), expected);
    }

    #[test]
    fn test_generate_json_schema_integer() {
        let input = json!(42);
        let expected = json!({"type": "integer"});
        assert_eq!(generate_json_schema(&input), expected);
    }

    #[test]
    fn test_generate_json_schema_number() {
        let input = json!(3.14);
        let expected = json!({"type": "number"});
        assert_eq!(generate_json_schema(&input), expected);
    }

    #[test]
    fn test_generate_json_schema_boolean() {
        let input = json!(true);
        let expected = json!({"type": "boolean"});
        assert_eq!(generate_json_schema(&input), expected);
    }

    #[test]
    fn test_generate_json_schema_null() {
        let input = json!(null);
        let expected = json!({"type": "null"});
        assert_eq!(generate_json_schema(&input), expected);
    }

    #[test]
    fn test_generate_object_schema() {
        let input = json!({
            "name": "John Doe",
            "age": 30,
            "is_student": false
        });
        let expected = json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"},
                "is_student": {"type": "boolean"}
            },
            "required": ["age", "is_student", "name"]
        });
        assert_eq!(generate_json_schema(&input), expected);
    }

    #[test]
    fn test_generate_array_schema() {
        let input = json!([1, 2, 3]);
        let expected = json!({
            "type": "array",
            "items": {"type": "integer"}
        });
        assert_eq!(generate_json_schema(&input), expected);
    }

    #[test]
    fn test_generate_array_schema_mixed_types() {
        let input = json!([1, "two", 3.0]);
        let expected = json!({
            "type": "array",
            "items": {
                "oneOf": [
                    {"oneOf": [
                        {"type": "integer"},
                        {"type": "string"}
                    ]},
                    {"type": "number"}
                ]
            }
        });
        assert_eq!(generate_json_schema(&input), expected);
    }

    #[test]
    fn test_generate_array_schema_empty() {
        let input = json!([]);
        let expected = json!({
            "type": "array",
            "items": {}
        });
        assert_eq!(generate_json_schema(&input), expected);
    }

    #[test]
    fn test_find_common_schema() {
        let schemas = vec![
            json!({"type": "integer"}),
            json!({"type": "string"}),
            json!({"type": "boolean"}),
        ];
        let expected = json!({
            "oneOf": [
                {"oneOf": [
                    {"type": "integer"},
                    {"type": "string"}
                ]},
                {"type": "boolean"}
            ]
        });
        assert_eq!(find_common_schema(&schemas), expected);
    }

    #[test]
    fn test_merge_schemas_same_type() {
        let schema1 = json!({"type": "object", "properties": {"a": {"type": "string"}}});
        let schema2 = json!({"type": "object", "properties": {"b": {"type": "integer"}}});
        let expected = json!({
            "type": "object",
            "properties": {
                "a": {"type": "string"},
                "b": {"type": "integer"}
            }
        });
        assert_eq!(merge_schemas(&schema1, &schema2), expected);
    }

    #[test]
    fn test_merge_schemas_different_types() {
        let schema1 = json!({"type": "string"});
        let schema2 = json!({"type": "integer"});
        let expected = json!({
            "oneOf": [
                {"type": "string"},
                {"type": "integer"}
            ]
        });
        assert_eq!(merge_schemas(&schema1, &schema2), expected);
    }

    #[test]
    fn test_generate_schema_with_ref() {
        let input = json!({
            "$ref": "#/definitions/address",
            "address": {
                "street": "123 Main St",
                "city": "New York"
            }
        });
        let expected = json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "type": "object",
            "properties": {
                "address": {
                    "type": "object",
                    "properties": {
                        "street": {"type": "string"},
                        "city": {"type": "string"}
                    },
                    "required": ["city", "street"]
                }
            },
            "required": ["address"],
            "$ref": "#/definitions/address"
        });
        assert_eq!(generate_json_schema(&input), expected);
    }
}
