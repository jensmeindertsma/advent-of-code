use crate::common::generate_sequence;

pub fn part_2(input: &str) -> usize {
    let input = input.trim().to_owned();
    (0..50)
        .fold(input, |sequence, _| generate_sequence(sequence))
        .len()
}
