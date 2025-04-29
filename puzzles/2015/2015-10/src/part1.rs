use crate::sequence::generate_sequence;

pub fn part_1(input: &str) -> usize {
    let input = input.trim().to_owned();
    (0..40)
        .fold(input, |sequence, _| generate_sequence(sequence))
        .len()
}
