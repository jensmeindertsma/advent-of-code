use nom::{combinator::map, sequence::tuple};

pub fn ingredient(input: &str) -> nom::IResult<&str, Ingredient> {
    map(
        tuple((name, capacity, durability, flavor, texture, calories)),
        |(name, capacity, durability, flavor, texture, calories)| Ingredient {
            name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
        },
    )(input)
}
