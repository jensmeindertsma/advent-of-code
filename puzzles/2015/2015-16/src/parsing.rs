use std::collections::HashMap;

use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
    multi::separated_list1,
};

#[derive(Clone, Debug)]
pub struct Person {
    pub number: u16,
    pub properties: HashMap<String, u8>,
}

pub fn person(input: &str) -> Option<Person> {
    map(
        (
            tag("Sue "),
            map_res(digit1, |s: &str| s.parse()),
            tag(": "),
            separated_list1(tag(", "), property),
        ),
        |(_, number, _, properties)| Person {
            number,
            properties: properties
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
        },
    )
    .parse(input)
    .map(|(_, person)| person)
    .ok()
}

fn property(input: &str) -> nom::IResult<&str, (&str, u8)> {
    map(
        (alpha1, tag(": "), map_res(digit1, |s: &str| s.parse())),
        |(name, _, value)| (name, value),
    )
    .parse(input)
}
