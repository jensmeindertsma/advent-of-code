use std::{cmp::Ordering, fs, str::FromStr};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let score = calculate_score(&input);

    println!("Total score: {}", score);
}

fn calculate_score(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut sides = line.split(' ');
            let opponent = sides.next().unwrap().parse::<Move>().unwrap();
            let suggested = sides.next().unwrap().parse::<Move>().unwrap();

            let outcome_score = match suggested.partial_cmp(&opponent) {
                Some(Ordering::Less) => 0,
                Some(Ordering::Equal) => 3,
                Some(Ordering::Greater) => 6,
                None => unreachable!(),
            };

            let shape_score = suggested as u8;

            (shape_score + outcome_score) as u32
        })
        .sum()
}

#[derive(PartialEq, Debug)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Move {
    type Err = &'static str;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Unknown move"),
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self {
            Move::Rock => match other {
                Move::Rock => Ordering::Equal,
                Move::Paper => Ordering::Less,
                Move::Scissors => Ordering::Greater,
            },
            Move::Paper => match other {
                Move::Rock => Ordering::Greater,
                Move::Paper => Ordering::Equal,
                Move::Scissors => Ordering::Less,
            },
            Move::Scissors => match other {
                Move::Rock => Ordering::Less,
                Move::Paper => Ordering::Greater,
                Move::Scissors => Ordering::Equal,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use day_02::TEST_INPUT;

    use super::calculate_score;

    #[test]
    fn example_list() {
        assert_eq!(calculate_score(TEST_INPUT), 15)
    }
}
