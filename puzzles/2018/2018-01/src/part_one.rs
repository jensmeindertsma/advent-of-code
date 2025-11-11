use crate::{parsing::parse, shared::Change};

pub fn part_one(input: &str) -> isize {
    input.trim().lines().map(|line| parsing::parse(line)).fold(
        0,
        |frequency, change| match change {
            Change::Increase(amount) => frequency + amount,
            Change::Decrease(amount) => frequency - amount,
        },
    )
}

// ((alt(tag("+"), tag("-")), digit1))
