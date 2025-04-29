use nom::{
    bytes::complete::{tag, take_while1},
    sequence::terminated,
    IResult, Parser,
};

pub fn part_1(input: &str) -> usize {
    let ingredients: Vec<Ingredient> = input
        .trim()
        .lines()
        .map(|line| parse_ingredient(line.trim()).unwrap())
        .collect();

    let mut mixer = Mixer::new(100, ingredients.len());
    let mut max_score = 0;

    let mut cookies_baked = 0;

    // for _ in 0..10 {
    //    let mix = mixer.mix().unwrap();

    while let Some(mix) = mixer.mix() {
        cookies_baked += 1;

        let mut capacity: isize = 0;
        let mut durability: isize = 0;
        let mut flavor: isize = 0;
        let mut texture: isize = 0;
        let mut calories: isize = 0;

        // Iterate through the ingredients, multiplying by their "amount"
        // as part of the current composition, and then add it to this cookie's
        // total property value.
        for (index, amount) in mix.iter().enumerate() {
            let ingredient = &ingredients[index];
            // println!("* using {amount} teaspoons of {}", ingredient.name);

            // This might cause issues if amount is too large
            let amount = *amount as isize;

            capacity += ingredient.capacity as isize * amount;
            durability += ingredient.durability as isize * amount;
            flavor += ingredient.flavor as isize * amount;
            texture += ingredient.texture as isize * amount;
            calories += ingredient.calories as isize * amount;
        }

        // println!("-> the cookie has a capacity of {capacity}");
        // println!("-> the cookie has a durability of {durability}");
        // println!("-> the cookie has a flavor of {flavor}");
        // println!("-> the cookie has a texture of {texture}");
        // println!("-> the cookie has {calories} calories");

        // `.max(0)` guarantees value is non-negative so casting to usize is safe.
        let score = capacity.max(0) as usize
            * durability.max(0) as usize
            * flavor.max(0) as usize
            * texture.max(0) as usize;

        // println!("=== the cookie has a score of {score}");

        // println!("--------");

        if score > max_score {
            max_score = score
        }
    }

    println!("Baked {cookies_baked} cookies");

    max_score
}

pub struct Mixer {
    total: usize,
    elements: usize,
    mix: Vec<usize>,
    state: State,
}

enum State {
    Ready,
    Running,
    Done,
}

impl Mixer {
    pub fn new(total: usize, elements: usize) -> Self {
        // Allocates vector with `elements` length and 0usize values.
        let mix = vec![0; elements];

        Self {
            total,
            elements,
            mix,
            state: State::Ready,
        }
    }

    pub fn mix(&mut self) -> Option<&Vec<usize>> {
        match self.state {
            State::Done => None,
            State::Ready => {
                self.state = State::Running;

                // Set mix to something like `[0, 0, 0, 100]`
                if self.elements > 0 {
                    self.mix[self.elements - 1] = self.total;
                }

                Some(&self.mix)
            }
            State::Running => {
                // Walks right to left finding the first non-zero element
                let mut index = self.elements - 1;
                while index > 0 && self.mix[index] == 0 {
                    index -= 1;
                }

                if index == 0 {
                    self.state = State::Done;
                    return Some(&self.mix);
                }

                // The first non-zero element is decremented by one and the
                // element to the left is incremented by one
                self.mix[index] -= 1;
                self.mix[index - 1] += 1;

                // Gather all to the right of index into a total sum
                let mut sum = 0;
                for j in index..self.elements {
                    sum += self.mix[j];
                    self.mix[j] = 0;
                }

                self.mix[self.elements - 1] = sum;

                Some(&self.mix)
            }
        }
    }
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
