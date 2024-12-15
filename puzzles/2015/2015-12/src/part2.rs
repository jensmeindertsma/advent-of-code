use serde_json::Value;

pub fn part_2(input: &str) -> i64 {
    let document: Value = serde_json::from_str(input.trim()).unwrap();

    fn count(value: &Value) -> i64 {
        match value {
            Value::Array(array) => array.iter().map(count).sum(),
            Value::Bool(_) => 0,
            Value::Null => 0,
            Value::Number(number) => number.as_i64().unwrap(),
            Value::Object(object) => {
                if object.keys().any(|key| key == "red")
                    || object
                        .values()
                        .any(|value| *value == Value::String("red".to_string()))
                {
                    0
                } else {
                    object.into_iter().map(|(_, value)| count(value)).sum()
                }
            }
            Value::String(_) => 0,
        }
    }

    count(&document)
}
