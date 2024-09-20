use itertools::Itertools;

pub fn part_1(input: &str, liters: usize) -> usize {
    let containers: Vec<usize> = input
        .trim()
        .lines()
        .map(|l| l.trim().parse().unwrap())
        .collect();

    let combinations = (1..=containers.len())
        .flat_map(|length| containers.iter().combinations(length))
        .filter(|c| c.iter().map(|x| **x).sum::<usize>() == liters);

    // combinations
    //     .clone()
    //     .for_each(|c| println!("{c:?} = {}", c.iter().map(|x| **x).sum::<usize>()));

    combinations.count()
}

pub fn part_2(input: &str, liters: usize) -> usize {
    let containers: Vec<usize> = input
        .trim()
        .lines()
        .map(|l| l.trim().parse().unwrap())
        .collect();

    let combinations =
        (1..=containers.len()).flat_map(|length| containers.iter().combinations(length));

    // println!("Generated {} combinations", combinations.clone().count());

    // combinations
    //     .clone()
    //     .for_each(|c| println!("{c:?} = {}", c.iter().map(|x| **x).sum::<usize>()));

    let combinations = combinations.filter(|c| c.iter().map(|x| **x).sum::<usize>() == liters);

    // println!(
    //     "Found {} combinations that add up to {liters} liters",
    //     combinations.clone().count()
    // );

    let min_container_amount = combinations.clone().map(|c| c.len()).min().unwrap();

    combinations
        .filter(|c| c.len() == min_container_amount)
        .count()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    const EXAMPLE_INPUT: &str = "
        20
        15
        5
        10
        5
    ";

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!(part_1(EXAMPLE_INPUT, 25), 4);

        assert_eq!(part_1(INPUT, 150), 1304);
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        assert_eq!(part_2(EXAMPLE_INPUT, 25), 3);

        assert_eq!(part_2(INPUT, 150), 18);
    }
}
