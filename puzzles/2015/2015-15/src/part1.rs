use crate::{
    ingredient::{parse_ingredient, Ingredient},
    mixer::Mixer,
};

pub fn part_1(input: &str) -> usize {
    let ingredients: Vec<Ingredient> = input
        .trim()
        .lines()
        .map(|line| parse_ingredient(line.trim()).unwrap())
        .collect();

    let mut mixer = Mixer::new(100, ingredients.len());
    let mut max_score = 0;

    while let Some(mix) = mixer.mix() {
        let mut capacity: isize = 0;
        let mut durability: isize = 0;
        let mut flavor: isize = 0;
        let mut texture: isize = 0;

        // Iterate through the ingredients, multiplying by their "amount"
        // as part of the current composition, and then add it to this cookie's
        // total property value.
        for (index, amount) in mix.iter().enumerate() {
            let ingredient = &ingredients[index];

            if *amount > isize::MAX as usize {
                panic!("Mixer overloaded!")
            }

            let amount = *amount as isize;

            capacity += ingredient.capacity as isize * amount;
            durability += ingredient.durability as isize * amount;
            flavor += ingredient.flavor as isize * amount;
            texture += ingredient.texture as isize * amount;
        }

        // `.max(0)` guarantees value is non-negative so casting to usize is safe.
        let score = capacity.max(0) as usize
            * durability.max(0) as usize
            * flavor.max(0) as usize
            * texture.max(0) as usize;

        if score > max_score {
            max_score = score
        }
    }

    max_score
}
