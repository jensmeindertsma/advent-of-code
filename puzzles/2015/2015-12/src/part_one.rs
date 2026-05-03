use serde_json::Value;

pub fn part_one(input: &str) -> i64 {
    let value: Value = serde_json::from_str(input).expect("input should be valid JSON");

    sum_value(&value)
}

fn sum_value(value: &Value) -> i64 {
    match value {
        Value::Number(number) => number.as_i64().expect("input contains only integers"),
        Value::Array(array) => array.iter().map(sum_value).sum(),
        Value::Object(object) => object.values().map(sum_value).sum(),
        _ => 0,
    }
}
