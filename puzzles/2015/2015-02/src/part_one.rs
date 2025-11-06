use crate::present::Present;

pub fn part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut sizes = line.split('x').map(|d| d.parse().unwrap());
            Present {
                length: sizes.next().unwrap(),
                width: sizes.next().unwrap(),
                height: sizes.next().unwrap(),
            }
        })
        .map(|present| {
            let sides = present.sides().map(|s| s.area());
            sides.iter().sum::<usize>() + sides.iter().min().unwrap()
        })
        .sum()
}
