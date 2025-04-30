use crate::sequencer::Sequencer;

pub fn part_2(input: &str) -> usize {
    let sequencer = Sequencer::new(input, 50);

    let sequence = sequencer.run();

    sequence.len()
}
