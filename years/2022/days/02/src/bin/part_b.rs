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

            (match sides.next().unwrap() {
                "X" => {
                    // Need to lose
                    let outcome_score = 0;

                    let shape_score = match opponent {
                        Move::Rock => Move::Scissors,
                        Move::Paper => Move::Rock,
                        Move::Scissors => Move::Paper,
                    } as u8;

                    shape_score + outcome_score
                }
                "Y" => {
                    // Need to draw
                    let outcome_score = 3;

                    // Match opponent move
                    let shape_score = opponent as u8;

                    shape_score + outcome_score
                }
                "Z" => {
                    // Need to win
                    let outcome_score = 6;

                    let shape_score = match opponent {
                        Move::Rock => Move::Paper,
                        Move::Paper => Move::Scissors,
                        Move::Scissors => Move::Rock,
                    } as u8;

                    shape_score + outcome_score
                }
                _ => panic!("Unexpected input token"),
            }) as u32
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
        assert_eq!(calculate_score(TEST_INPUT), 12)
    }
}
