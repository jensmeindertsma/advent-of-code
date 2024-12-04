use crate::common::Miner;

pub fn part_1(input: &str) -> usize {
    for (number, hash) in Miner::new(input.trim()) {
        if hash.starts_with("00000") {
            return number;
        }
    }

    panic!("failed to find a solution")
}
