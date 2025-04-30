use serde_json::{Map, Value};

pub fn count<F>(value: &Value, object_handler: F) -> i64
where
    F: Fn(&Map<String, Value>) -> i64 + Clone,
{
    match value {
        Value::Array(array) => array
            .iter()
            .map(|element| count(element, object_handler.clone()))
            .sum(),
        Value::Bool(_) => 0,
        Value::Null => 0,
        Value::Number(number) => number.as_i64().unwrap(),
        Value::Object(object) => object_handler(object),
        Value::String(_) => 0,
    }
}
