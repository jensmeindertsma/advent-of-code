mod circuit;
mod instruction;
mod part1;
mod part2;

pub use part1::part_1;
pub use part2::part_2;

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use super::{circuit::Circuit, instruction::Instruction, part_1};

        let mut circuit = Circuit::build(
            "
            123 -> x
            456 -> y
            x AND y -> d
            x OR y -> e
            x LSHIFT 2 -> f
            y RSHIFT 2 -> g
            NOT x -> h
            NOT y -> i
            "
            .trim()
            .lines()
            .map(|line| Instruction::try_from(line.trim()).unwrap()),
        );

        assert_eq!(circuit.signal("d"), 72);
        assert_eq!(circuit.signal("e"), 507);
        assert_eq!(circuit.signal("f"), 492);
        assert_eq!(circuit.signal("g"), 114);
        assert_eq!(circuit.signal("h"), 65412);
        assert_eq!(circuit.signal("i"), 65079);
        assert_eq!(circuit.signal("x"), 123);
        assert_eq!(circuit.signal("y"), 456);

        assert_eq!(part_1(INPUT), 46065);
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        assert_eq!(part_2(INPUT), 14134)
    }
}
