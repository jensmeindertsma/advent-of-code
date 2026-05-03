use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
};

pub fn relation(input: &str) -> Option<(&str, &str, i64)> {
    map(
        (
            alpha1,
            tag(" would "),
            friendship,
            tag(" happiness units by sitting next to "),
            alpha1,
            tag("."),
        ),
        |(person_a, _, friendship, _, person_b, _)| (person_a, person_b, friendship),
    )
    .parse(input)
    .map(|(_, relation)| relation)
    .ok()
}

fn friendship(input: &str) -> IResult<&str, i64> {
    (alt((
        map(
            (tag("gain "), map_res(digit1, |s: &str| s.parse())),
            |(_, units)| units,
        ),
        map(
            (tag("lose "), map_res(digit1, |s: &str| s.parse::<i64>())),
            |(_, units)| -units,
        ),
    )))
    .parse(input)
}
