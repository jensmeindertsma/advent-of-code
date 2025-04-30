use crate::{
    evaluator::{Evaluator, Target},
    sue::parse_sue,
};

pub fn part_2(input: &str) -> u16 {
    let target = Target {
        akitas: 0,
        cars: 2,
        cats: 7,
        children: 3,
        goldfish: 5,
        perfumes: 1,
        pomeranians: 3,
        samoyeds: 2,
        trees: 3,
        vizslas: 0,
    };

    let evaluator = Evaluator {
        akitas: |value, target| value == target,
        cars: |value, target| value == target,
        cats: |value, target| value > target,
        children: |value, target| value == target,
        goldfish: |value, target| value < target,
        perfumes: |value, target| value == target,
        pomeranians: |value, target| value < target,
        samoyeds: |value, target| value == target,
        trees: |value, target| value > target,
        vizslas: |value, target| value == target,
    };

    for sue in input
        .trim()
        .lines()
        .map(|line| parse_sue(line.trim()).unwrap())
    {
        if evaluator.evaluate(&sue, &target) {
            return sue.number;
        }
    }

    panic!("No matching Sue was found!");
}
