use crate::counting::count;
use serde_json::{Map, Value};

pub fn part_1(input: &str) -> i64 {
    let document: Value = serde_json::from_str(input.trim()).unwrap();

    fn handle_object(object: &Map<String, Value>) -> i64 {
        object
            .into_iter()
            .map(|(_, value)| count(value, handle_object))
            .sum()
    }

    count(&document, handle_object)
}
