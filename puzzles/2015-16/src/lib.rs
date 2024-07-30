mod sue;

use sue::Sue;

const TICKER_TAPE: &str = "
    children: 3
    cats: 7
    samoyeds: 2
    pomeranians: 3
    akitas: 0
    vizslas: 0
    goldfish: 5
    trees: 3
    cars: 2
    perfumes: 1
";

pub fn part_1(input: &str) -> usize {
    let mut iter = TICKER_TAPE.trim().lines().peekable();
    let mut sue_str = String::from("Sue 0: ");

    while let Some(line) = iter.next() {
        if iter.peek().is_some() {
            sue_str.push_str(&format!("{}, ", line.trim()));
        } else {
            sue_str.push_str(line.trim());
        }
    }

    println!("todo sue = {sue_str}");

    let correct_sue: Sue = sue_str.parse().unwrap();

    println!("Sue = {correct_sue:?}");

    input
        .trim()
        .lines()
        .map(|line| line.parse::<Sue>().unwrap())
        .enumerate()
        .find_map(|(n, sue)| {
            if sue.akitas.is_some() && sue.akitas != correct_sue.akitas {
                return None;
            }

            if sue.cars.is_some() && sue.cars != correct_sue.cars {
                return None;
            }

            if sue.cats.is_some() && sue.cats != correct_sue.cats {
                return None;
            }

            if sue.children.is_some() && sue.children != correct_sue.children {
                return None;
            }

            if sue.goldfish.is_some() && sue.goldfish != correct_sue.goldfish {
                return None;
            }

            if sue.perfumes.is_some() && sue.perfumes != correct_sue.perfumes {
                return None;
            }

            if sue.pomeranians.is_some() && sue.pomeranians != correct_sue.pomeranians {
                return None;
            }

            if sue.samoyeds.is_some() && sue.samoyeds != correct_sue.samoyeds {
                return None;
            }

            if sue.trees.is_some() && sue.trees != correct_sue.trees {
                return None;
            }

            if sue.vizslas.is_some() && sue.vizslas != correct_sue.vizslas {
                return None;
            }

            Some(n + 1)
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        // use crate::part_1;

        // assert_eq!(part_1(INPUT), 6);
    }
}
