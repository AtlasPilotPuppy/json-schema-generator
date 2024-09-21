use serde_json::{Value, json, Map};

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
