use crate::common::Miner;

pub fn part_2(input: &str) -> usize {
    for (n, hash) in Miner::new(input.trim()) {
        if hash.starts_with("000000") {
            return n;
        }
    }

    panic!("failed to find solution")
}
