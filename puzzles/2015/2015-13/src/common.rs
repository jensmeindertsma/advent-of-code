use nom::{
    Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
};

pub struct Relation {
    pub person: String,
    pub neighbor: String,
    pub delta: i16,
}

pub fn parse_relation(input: &str) -> nom::IResult<&str, Relation> {
    map(
        (
            alpha1,
            tag(" would "),
            map(
                (
                    alt((
                        (tag("gain "), map_res(digit1, |s: &str| s.parse::<i16>())),
                        (
                            tag("lose "),
                            map_res(digit1, |s: &str| s.parse::<i16>().map(|i| -i)),
                        ),
                    )),
                    tag(" happiness units"),
                ),
                |((_, delta), _)| delta,
            ),
            tag(" by sitting next to "),
            alpha1,
            tag("."),
        ),
        |(person, _, delta, _, neighbor, _)| Relation {
            person: person.to_string(),
            neighbor: neighbor.to_string(),
            delta,
        },
    )
    .parse(input)
}
