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

struct Sue {
    akitas: Option<u8>,
    cars: Option<u8>,
    cats: Option<u8>,
    children: Option<u8>,
    goldfish: Option<u8>,
    perfumes: Option<u8>,
    pomeranians: Option<u8>,
    samoyeds: Option<u8>,
    trees: Option<u8>,
    vizslas: Option<u8>,
}

pub fn part_1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<Sue>().unwrap())
        .enumerate
        .find_map(|(n, sue)| {
            let akitas_match = 
        })
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!(part_1("abc"), 3);
        assert_eq!(part_1("9876543210"), 10);

        assert_eq!(part_1(INPUT), 6);
    }
}
