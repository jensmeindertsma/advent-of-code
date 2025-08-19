mod circuit;
mod instruction;

mod part_one;
mod part_two;

pub use part_one::part_one;
pub use part_two::part_two;

#[test]
fn one() {
    use circuit::Circuit;
    use instruction::Instruction;

    let input = indoc::indoc! {"
        123 -> x
        456 -> y
        x AND y -> d
        x OR y -> e
        x LSHIFT 2 -> f
        y RSHIFT 2 -> g
        NOT x -> h
        NOT y -> i
    "};

    let instructions: Vec<_> = input
        .lines()
        .map(|line| Instruction::parse(line).unwrap())
        .collect();

    let mut circuit = Circuit::new();

    for instruction in instructions {
        circuit.set(instruction.output_wire, instruction.gate);
    }

    assert_eq!(circuit.signal("d"), 72);
    assert_eq!(circuit.signal("e"), 507);
    assert_eq!(circuit.signal("f"), 492);
    assert_eq!(circuit.signal("g"), 114);
    assert_eq!(circuit.signal("h"), 65412);
    assert_eq!(circuit.signal("i"), 65079);
    assert_eq!(circuit.signal("x"), 123);
    assert_eq!(circuit.signal("y"), 456);

    assert_eq!(part_one(include_str!("../input.txt")), 46065);
}

#[test]
fn two() {
    assert_eq!(part_two(include_str!("../input.txt")), 14134);
}
