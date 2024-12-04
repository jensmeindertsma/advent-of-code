use crate::common::Miner;

pub fn part_1(input: &str) -> usize {
    for (n, hash) in Miner::new(input.trim()) {
        if hash.starts_with("00000") {
            return n;
        }
    }

    panic!("failed to find solution")
}
