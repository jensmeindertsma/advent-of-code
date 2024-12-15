use serde_json::Value;

pub fn part_1(input: &str) -> i64 {
    let document: Value = serde_json::from_str(input.trim()).unwrap();

    fn count(value: &Value) -> i64 {
        match value {
            Value::Array(array) => array.iter().map(count).sum(),
            Value::Bool(_) => 0,
            Value::Null => 0,
            Value::Number(number) => number.as_i64().unwrap(),
            Value::Object(object) => object.into_iter().map(|(_, value)| count(value)).sum(),
            Value::String(_) => 0,
        }
    }

    count(&document)
}
