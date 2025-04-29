use nom::{
    bytes::complete::{tag, take_while1},
    error::Error as NomParseError,
    sequence::terminated,
    Err as NomError, IResult, Parser,
};

#[derive(Debug)]
pub struct Ingredient {
    pub capacity: i8,
    pub durability: i8,
    pub flavor: i8,
    pub texture: i8,
    pub calories: u8,
}

type ParseError<'a> = NomError<NomParseError<&'a str>>;

pub fn parse_ingredient(input: &str) -> Result<Ingredient, ParseError> {
    fn string(input: &str) -> IResult<&str, &str> {
        take_while1(|c: char| c.is_ascii_alphabetic()).parse(input)
    }

    fn number(input: &str) -> IResult<&str, i8> {
        take_while1(|c: char| c.is_ascii_digit() || c == '-')
            .map_res(|string: &str| string.parse())
            .parse(input)
    }

    (
        terminated(string, tag(": capacity ")),
        terminated(number, tag(", durability ")),
        terminated(number, tag(", flavor ")),
        terminated(number, tag(", texture ")),
        terminated(number, tag(", calories ")),
        take_while1(|c: char| c.is_ascii_digit()).map_res(|string: &str| string.parse()),
    )
        .map(
            |(_name, capacity, durability, flavor, texture, calories)| Ingredient {
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
