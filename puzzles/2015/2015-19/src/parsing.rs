use nom::{Parser, bytes::complete::tag, character::complete::alpha1, combinator::map};

pub fn replacement(input: &str) -> Option<(&str, &str)> {
    map((molecule, tag(" => "), molecule), |(from, _, to)| {
        (from, to)
    })
    .parse(input)
    .map(|(_, value)| value)
    .ok()
}

fn molecule(input: &str) -> nom::IResult<&str, &str> {
    alpha1.parse(input)
}
