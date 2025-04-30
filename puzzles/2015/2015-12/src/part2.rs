use crate::counting::count;
use serde_json::{Map, Value};

pub fn part_2(input: &str) -> i64 {
    let document: Value = serde_json::from_str(input.trim()).unwrap();

    fn handle_object(object: &Map<String, Value>) -> i64 {
        if object.keys().any(|key| key == "red")
            || object
                .values()
                .any(|value| *value == Value::String("red".to_string()))
        {
            0
        } else {
            object
                .into_iter()
                .map(|(_, value)| count(value, handle_object))
                .sum()
        }
    }

    count(&document, handle_object)
}
