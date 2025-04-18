use nom::{
    bytes::complete::{tag, take_while1},
    sequence::terminated,
    IResult, Parser,
};

pub fn part_1(input: &str) -> u32 {
    let ingredients: Vec<Ingredient> = input
        .trim()
        .lines()
        .map(|line| parse_ingredient(line.trim()).unwrap())
        .collect();

    dbg!(ingredients);

    todo!();
}

#[derive(Debug)]
struct Ingredient<'a> {
    name: &'a str,
    capacity: i8,
    durability: i8,
    flavor: i8,
    texture: i8,
    calories: i8,
}

type ParseError<'a> = nom::Err<nom::error::Error<&'a str>>;

fn parse_ingredient(input: &str) -> Result<Ingredient, ParseError> {
    fn string(input: &str) -> IResult<&str, &str> {
        take_while1(|c: char| c.is_ascii_alphabetic()).parse(input)
    }

    fn number(input: &str) -> IResult<&str, i8> {
        take_while1(|c: char| c.is_ascii_digit() || c == '-')
            .map_res(|speed: &str| speed.parse())
            .parse(input)
    }

    (
        terminated(string, tag(": capacity ")),
        terminated(number, tag(", durability ")),
        terminated(number, tag(", flavor ")),
        terminated(number, tag(", texture ")),
        terminated(number, tag(", calories ")),
        number,
    )
        .map(
            |(name, capacity, durability, flavor, texture, calories)| Ingredient {
                name,
                capacity,
                durability,
                flavor,
                texture,
                calories,
            },
        )
        .parse(input)
        .map(|(_, ingredient)| ingredient)
}
