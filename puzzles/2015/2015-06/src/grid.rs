use crate::instruction::{Instruction, Task};

pub struct Grid<I> {
    inner: Vec<Vec<I>>,
}

impl<I> Grid<I> {
    pub fn new(size: usize, initial_value: I) -> Self
    where
        I: Clone + Copy,
    {
        Self {
            inner: vec![vec![initial_value; size]; size],
        }
    }

    pub fn process(
        &mut self,
        instructions: impl Iterator<Item = Instruction>,
        mut handler: impl FnMut(&Task, &mut I),
    ) {
        for instruction in instructions {
            for column in self
                .inner
                .iter_mut()
                .take(instruction.to.x + 1)
                .skip(instruction.from.x)
            {
                for item in column
                    .iter_mut()
                    .take(instruction.to.y + 1)
                    .skip(instruction.from.y)
                {
                    handler(&instruction.task, item)
                }
            }
        }
    }

    pub fn columns(&self) -> impl Iterator<Item = &Vec<I>> {
        self.inner.iter()
    }
}
