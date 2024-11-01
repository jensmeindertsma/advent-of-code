use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let line = line.trim();

            let numbers = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());

            let min = numbers.clone().min().unwrap();
            let max = numbers.max().unwrap();

            max - min
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let line = line.trim();

            let numbers = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());

            numbers
                .tuple_combinations()
                .find_map(|(a, b)| {
                    if a % b == 0 {
                        Some(a / b)
                    } else if b % a == 0 {
                        Some(b / a)
                    } else {
                        None
                    }
                })
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!(
            part_1(
                "
                5 1 9 5
                7 5 3
                2 4 6 8
                "
            ),
            18
        );

        assert_eq!(part_1(INPUT), 39126);
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        assert_eq!(
            part_2(
                "
                5 9 2 8
                9 4 7 3
                3 8 6 5
                "
            ),
            9
        );

        assert_eq!(part_2(INPUT), 258);
    }
}
