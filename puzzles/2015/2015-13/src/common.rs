use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
    Parser,
};

pub struct Relation {
    pub person: String,
    pub neighbor: String,
    pub delta: isize,
}

pub fn relation(input: &str) -> nom::IResult<&str, Relation> {
    map(
        (
            alpha1,
            tag(" would "),
            map(
                (
                    alt((
                        (tag("gain "), map_res(digit1, |s: &str| s.parse::<isize>())),
                        (
                            tag("lose "),
                            map_res(digit1, |s: &str| s.parse::<isize>().map(|i| -i)),
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
