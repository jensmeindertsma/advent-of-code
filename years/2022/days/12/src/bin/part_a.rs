#![feature(iter_next_chunk)]

use day_12::{Map, Pathfinder};
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let map = Map::from(input.as_str());
    let pathfinder = Pathfinder::from(&map);

    let steps = pathfinder.run();

    println!("Minimum amount of steps required: {steps}");
}

#[cfg(test)]
mod tests {
    use day_12::{Map, Pathfinder, TEST_INPUT};

    #[test]
    fn test_input() {
        let map = Map::from(TEST_INPUT);
        let pathfinder = Pathfinder::from(&map);

        assert_eq!(pathfinder.run(), 31)
    }
}
