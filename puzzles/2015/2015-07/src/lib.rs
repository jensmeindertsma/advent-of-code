mod circuit;
mod instruction;
mod part_one;
mod part_two;

pub use part_one::part_one;
pub use part_two::part_two;

#[test]
fn one() {
    use crate::{
        circuit::{Circuit, Wire},
        instruction::{Instruction, parsing},
    };
    use indoc::indoc;

    let input = indoc! {"
        123 -> x
        456 -> y
        x AND y -> d
        x OR y -> e
        x LSHIFT 2 -> f
        y RSHIFT 2 -> g
        NOT x -> h
        NOT y -> i
    "};

    let instructions: Vec<Instruction<'_>> = input
        .lines()
        .map(parsing::instruction)
        .map(|result| result.expect("every instruction should be valid"))
        .collect();

    let mut circuit = Circuit::new();

    for instruction in instructions {
        circuit.assemble(instruction);
    }

    assert_eq!(circuit.get_signal(Wire("d")), 72);
    assert_eq!(circuit.get_signal(Wire("e")), 507);
    assert_eq!(circuit.get_signal(Wire("f")), 492);
    assert_eq!(circuit.get_signal(Wire("g")), 114);
    assert_eq!(circuit.get_signal(Wire("h")), 65_412);
    assert_eq!(circuit.get_signal(Wire("i")), 65_079);
    assert_eq!(circuit.get_signal(Wire("x")), 123);
    assert_eq!(circuit.get_signal(Wire("y")), 456);

    assert_eq!(part_one(include_str!("../input.txt")), 46_065);
}

#[test]
fn two() {
    assert_eq!(part_two(include_str!("../input.txt")), 14_134);
}
