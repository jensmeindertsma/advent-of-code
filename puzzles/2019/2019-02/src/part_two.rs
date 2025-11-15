use crate::computer::Computer;

pub fn part_two(input: &str) -> usize {
    let computer = Computer::with_program(input);

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut computer = computer.clone();

            computer.set(1, noun);
            computer.set(2, verb);

            if computer.run().state[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    unreachable!()
}
