use crate::parsing;

pub fn part_one(input: &str) -> u16 {
    let wanted = vec![
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ];

    input
        .trim()
        .lines()
        .map(parsing::person)
        .map(|result| result.expect("parsing should never fail"))
        .filter(|person| {
            wanted
                .iter()
                .all(|(k, w)| person.properties.get(*k).is_none_or(|v| v == w))
        })
        .map(|person| person.number)
        .next()
        .unwrap()
}
