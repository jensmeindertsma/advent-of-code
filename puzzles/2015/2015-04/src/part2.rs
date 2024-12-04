use crate::common::Miner;

pub fn part_2(input: &str) -> usize {
    for (number, hash) in Miner::new(input.trim()) {
        if hash.starts_with("000000") {
            return number;
        }
    }

    panic!("failed to find a solution")
}
