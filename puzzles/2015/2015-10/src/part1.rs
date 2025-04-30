use crate::sequencer::Sequencer;

pub fn part_1(input: &str) -> usize {
    let sequencer = Sequencer::new(input, 40);

    let sequence = sequencer.run();

    sequence.len()
}
