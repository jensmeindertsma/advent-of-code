pub fn part_1(input: &str) -> usize {
    find_hash_with_prefix(input.trim(), "00000")
}

pub fn part_2(input: &str) -> usize {
    find_hash_with_prefix(input.trim(), "000000")
}

fn find_hash_with_prefix(input: &str, prefix: &str) -> usize {
    for number in 1.. {
        let input = format!("{}{}", input, number);
        let digest = md5::compute(input.as_bytes());
        let hash = format!("{:x}", digest);

        if hash.starts_with(prefix) {
            return number;
        };
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!(part_1("abcdef"), 609043);
        assert_eq!(part_1("pqrstuv"), 1048970);

        assert_eq!(part_1(INPUT), 254575);
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        assert_eq!(part_2(INPUT), 1038736);
    }
}
