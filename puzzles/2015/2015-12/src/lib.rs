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

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!(part_1("[1,2,3]"), 6);
        assert_eq!(part_1("{\"a\":2,\"b\":4}"), 6);
        assert_eq!(part_1("[[[3]]]"), 3);
        assert_eq!(part_1("{\"a\":{\"b\":4},\"c\":-1}"), 3);
        assert_eq!(part_1("{\"a\":[-1,1]}"), 0);
        assert_eq!(part_1("[-1,{\"a\":1}]"), 0);
        assert_eq!(part_1("[]"), 0);
        assert_eq!(part_1("{}"), 0);

        assert_eq!(part_1(INPUT), 156366);
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        assert_eq!(part_2("[1,2,3]"), 6);
        assert_eq!(part_2("[1,{\"c\":\"red\",\"b\":2},3]"), 4);
        assert_eq!(part_2("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"), 0);
        assert_eq!(part_2("[1,\"red\",5]"), 6);

        assert_eq!(part_2(INPUT), 96852);
    }
}
