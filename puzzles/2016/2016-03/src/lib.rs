use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|string| {
            string
                .split_whitespace()
                .map(|side| side.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|sides: &Vec<usize>| {
            let a = sides[0];
            let b = sides[1];
            let c = sides[2];

            (a + b) > c && (a + c) > b && (b + c) > a
        })
        .count()
}

pub fn part_2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|string| {
            string
                .split_whitespace()
                .map(|side| side.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .chunks(3)
        .into_iter()
        .flat_map(|mut chunk| {
            let row1 = chunk.next().unwrap();
            let row2 = chunk.next().unwrap();
            let row3 = chunk.next().unwrap();

            [
                vec![row1[0], row2[0], row3[0]],
                vec![row1[1], row2[1], row3[1]],
                vec![row1[2], row2[2], row3[2]],
            ]
            .into_iter()
        })
        .filter(|sides: &Vec<usize>| {
            let a = sides[0];
            let b = sides[1];
            let c = sides[2];

            (a + b) > c && (a + c) > b && (b + c) > a
        })
        .count()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!(part_1(INPUT), 982);
    }
}
