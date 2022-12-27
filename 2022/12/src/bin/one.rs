#![feature(iter_next_chunk)]

use day_12::{map::Map, pathfinder::Pathfinder};
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let map = Map::from(input.as_str());
    let pathfinder = Pathfinder::from(&map);

    let path = pathfinder.run();

    println!("Minimum amount of steps required: {}", path.len());
}

#[cfg(test)]
mod tests {
    use day_12::{map::Map, pathfinder::Pathfinder, TEST_INPUT};

    #[test]
    fn test_input() {
        let map = Map::from(TEST_INPUT);
        let pathfinder = Pathfinder::from(&map);

        assert_eq!(pathfinder.run().len(), 31)
    }
}
