#![feature(iter_next_chunk)]

use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let monkey_business = measure_monkey_business(&input);

    println!("Level of monkey business after 20 rounds: {monkey_business}");
}

fn measure_monkey_business(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::from_description).collect();

    for round in 1..=20 {
        println!("--- STARTING ROUND {round} ---");

        for monkey_position in 0..monkeys.len() {
            println!("Monkey {monkey_position}");

            // for item_position in 0..monkey.items.len() {
            //     let item = monkey.items[item_position];

            //     println!(
            //         "Monkey inspects an item with a worry level of {}.",
            //         item.worry_level
            //     );

            //     match &monkey.operation {
            //         Operation::Addition(value) => match value {
            //             Value::Old => {
            //                 println!(
            //                     "Worry level is increased by itself to {}.",
            //                     item.worry_level * 2
            //                 );
            //                 item.worry_level += item.worry_level;
            //             }
            //             Value::Number(amount) => {
            //                 println!(
            //                     "Worry level is increased by {amount} to {}.",
            //                     item.worry_level + amount
            //                 );
            //                 item.worry_level += amount;
            //             }
            //         },
            //         Operation::Multiplication(value) => match value {
            //             Value::Old => {
            //                 println!(
            //                     "Worry level is multiplied by itself to {}.",
            //                     item.worry_level * item.worry_level
            //                 );
            //                 item.worry_level *= item.worry_level;
            //             }
            //             Value::Number(amount) => {
            //                 println!(
            //                     "Worry level is multiplied by {amount} to {}.",
            //                     item.worry_level * amount
            //                 );
            //                 item.worry_level *= amount;
            //             }
            //         },
            //     };

            //     item.worry_level = (item.worry_level as f64 / 3.0).floor() as usize;
            //     println!(
            //         "Monkey gets bored with item. Worry level is divided by 3 to {}.",
            //         item.worry_level
            //     );

            //     let target = match monkey.test.check.check(item.worry_level) {
            //         true => monkey.test.truthy_target,

            //         false => monkey.test.falsy_target,
            //     };

            //     let item = monkey.items.remove(item_position);
            //     monkeys[target].items.push(item)
            // }
        }
    }

    monkeys.sort_by(|a, b| a.activity.partial_cmp(&b.activity).unwrap());
    monkeys
        .iter()
        .take(2)
        .map(|monkey| monkey.activity)
        .reduce(|acc, item| acc * item)
        .unwrap()
}

#[derive(Debug)]
struct Monkey {
    activity: usize,
    items: Vec<Item>,
    operation: Operation,
    test: Test,
}

#[derive(Debug)]
struct Item {
    worry_level: usize,
}

impl Monkey {
    fn from_description(description: &str) -> Self {
        let mut lines = description.lines().skip(1);

        let items = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Starting items:")
            .unwrap()
            .split(',')
            .map(|slice| Item {
                worry_level: slice.trim().parse().unwrap(),
            })
            .collect();

        let operation = {
            let mut pieces = lines
                .next()
                .unwrap()
                .trim()
                .strip_prefix("Operation: new = old")
                .unwrap()
                .split_whitespace();

            let symbol = pieces.next().unwrap();

            let rhs = match pieces.next().unwrap() {
                "old" => Value::Old,
                other => Value::Number(other.parse().unwrap()),
            };

            match symbol {
                "+" => Operation::Addition(rhs),
                "*" => Operation::Multiplication(rhs),
                other => panic!("Unexpected operation: `{other}`"),
            }
        };

        let test = {
            let test_description = lines.next().unwrap().trim().strip_prefix("Test:").unwrap();

            let first_digit_position = test_description.find(|c: char| c.is_ascii_digit()).unwrap();
            let check = test_description[..first_digit_position].trim();

            let check = match check {
                "divisible by" => Check::DivisibleBy(
                    test_description[first_digit_position..]
                        .trim()
                        .parse()
                        .unwrap(),
                ),
                other => panic!("Unexpected check: {other}"),
            };

            Test {
                check,
                truthy_target: lines
                    .next()
                    .unwrap()
                    .trim()
                    .strip_prefix("If true: throw to monkey")
                    .unwrap()
                    .trim()
                    .parse()
                    .unwrap(),
                falsy_target: lines
                    .next()
                    .unwrap()
                    .trim()
                    .strip_prefix("If false: throw to monkey")
                    .unwrap()
                    .trim()
                    .parse()
                    .unwrap(),
            }
        };

        Self {
            activity: 0,
            items,
            operation,
            test,
        }
    }
}

#[derive(Debug)]
enum Operation {
    Addition(Value),
    Multiplication(Value),
}

#[derive(Debug)]
enum Value {
    Number(usize),
    Old,
}

#[derive(Debug)]
struct Test {
    check: Check,
    truthy_target: usize,
    falsy_target: usize,
}

#[derive(Debug)]
enum Check {
    DivisibleBy(usize),
}

impl Check {
    fn check(&self, value: usize) -> bool {
        match self {
            Self::DivisibleBy(amount) => value % amount == 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use day_11::TEST_INPUT;

    use super::measure_monkey_business;

    #[test]
    fn test_input() {
        assert_eq!(measure_monkey_business(TEST_INPUT), 11605)
    }
}
