mod computer;

mod part_one;
mod part_two;

pub use part_one::part_one;
pub use part_two::part_two;

#[test]
fn one() {
    use computer::Computer;

    assert_eq!(
        Computer::with_program("1,0,0,0,99").run().state,
        vec![2, 0, 0, 0, 99]
    );

    assert_eq!(
        Computer::with_program("2,3,0,3,99").run().state,
        vec![2, 3, 0, 6, 99]
    );

    assert_eq!(
        Computer::with_program("2,4,4,5,99,0").run().state,
        vec![2, 4, 4, 5, 99, 9801]
    );

    assert_eq!(
        Computer::with_program("1,1,1,4,99,5,6,0,99").run().state,
        vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
    );

    assert_eq!(part_one(include_str!("../input.txt")), 7210630);
}

#[test]
fn two() {
    assert_eq!(part_two(include_str!("../input.txt")), 3892);
}
