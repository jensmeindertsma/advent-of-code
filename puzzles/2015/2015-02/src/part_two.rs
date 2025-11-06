use crate::present::Present;

pub fn part_two(input: &str) -> usize {
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
            present.sides().map(|s| s.perimeter()).iter().min().unwrap() + present.volume()
        })
        .sum()
}
