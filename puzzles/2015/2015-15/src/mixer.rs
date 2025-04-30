pub struct Mixer {
    total: usize,
    elements: usize,
    mix: Vec<usize>,
    state: State,
}

enum State {
    Ready,
    Running,
    Done,
}

impl Mixer {
    pub fn new(total: usize, elements: usize) -> Self {
        if total == 0 {
            panic!("Please provide a larger total to the mixer!")
        }

        if elements == 0 {
            panic!("The mixer cannot run for zero elements!")
        }

        // Allocates vector with `elements` length and 0usize values.
        let mix = vec![0; elements];

        Self {
            total,
            elements,
            mix,
            state: State::Ready,
        }
    }

    pub fn mix(&mut self) -> Option<&Vec<usize>> {
        match self.state {
            State::Done => None,
            State::Ready => {
                self.state = State::Running;

                // Set mix to something like `[0, 0, 0, 100]`
                if self.elements > 0 {
                    self.mix[self.elements - 1] = self.total;
                }

                Some(&self.mix)
            }
            State::Running => {
                // Walks right to left finding the first non-zero element
                let mut index = self.elements - 1;
                while index > 0 && self.mix[index] == 0 {
                    index -= 1;
                }

                // If only the left-most element has a non-zero value,
                // we've reached a [100, 0, 0, 0] state which means we're done
                if index == 0 {
                    self.state = State::Done;
                    return Some(&self.mix);
                }

                // The first non-zero element is decremented by one and the
                // element to the left is incremented by one
                self.mix[index] -= 1;
                self.mix[index - 1] += 1;

                // This part is the hardest to understand but the most crucial:
                // see the following list
                // [0, 0, 5]
                // [0, 1, 4] -> right-most non-zero -= 1, next to the left += 1
                // [0, 2, 3] -> again
                // [0, 3, 2] -> again
                // [0, 4, 1] -> again
                // [0, 5, 0] ** would produce `[1, 4, 0]` next, skipping over all
                //              the compositions where 1 is at the start (because
                //              the next one after would be `[2, 3, 0]` and we want
                //              `[1, 0, 4]`). Therefore we must detect when the mix
                //              has become `[1, 4, 0]` after the "shifting" code above
                //              completed. In that case `element_index` will be `1` and
                //              `self.mix[1] = 4`. We add 4 to `remainder` and set
                //              `self.mix = 0`. Lastly it sets the last element to the total
                //              "remainder".
                // [1, 0, 4]
                // [1, 1, 3]
                // [1, 2, 2]
                // [1, 3, 1]
                // [1, 4, 0]
                // [2, 0, 3]
                //
                // Basically when a "shift" of a value of 1 to the left results in
                // a zero value to the right of the "shifting" elements, the values
                // to the right of the "shifting" elements must be summed up, all the
                // values wiped to 0, and the last element value set to the sum. This
                // then prevents skipping over compositions.
                let mut remainder = 0;
                for element_index in index..self.elements {
                    remainder += self.mix[element_index];
                    self.mix[element_index] = 0;
                }

                self.mix[self.elements - 1] = remainder;

                Some(&self.mix)
            }
        }
    }
}
