use crate::{instruction::InstructionKind, one, two, Instruction};

pub struct Grid<L> {
    lights: Vec<Vec<L>>,
}

impl<L> Grid<L>
where
    L: Clone,
{
    pub fn with_size(size: usize, initial_state: L) -> Self {
        Self {
            lights: vec![vec![initial_state; size]; size],
        }
    }
}

/*
    [        x      x      x
        y [(0,0), (1,0), (2,0)]
        y [(0,1), (1,1), (2,1)]
        y [(0,2), (1,2), (2,2)]
    ]
    -> [0][2] gives you `(2,0)` so you need to do [y][x]

    [        y      y      y
        x [(0,0), (1,0), (2,0)]
        x [(0,1), (1,1), (2,1)]
        x [(0,2), (1,2), (2,2)]
    ]
    -> you can do [x][y] but `.flatten()` won't work here!
*/

impl<L> Grid<L> {
    pub fn iter(&self) -> GridIterator<'_, L> {
        GridIterator {
            iterator: self.lights.iter().flatten(),
        }
    }
}

impl Grid<one::Light> {
    pub fn lit(&self) -> usize {
        self.iter()
            .filter(|light| **light == one::Light::On)
            .count()
    }

    pub fn instruct(&mut self, instruction: Instruction) {
        use one::Light;

        for row in instruction.from.y..=instruction.to.y {
            for column in instruction.from.x..=instruction.to.x {
                let light = &mut self.lights[row][column];
                match instruction.kind {
                    InstructionKind::TurnOff => *light = Light::Off,
                    InstructionKind::TurnOn => *light = Light::On,
                    InstructionKind::Toggle => {
                        *light = match light {
                            Light::Off => Light::On,
                            Light::On => Light::Off,
                        }
                    }
                }
            }
        }
    }
}

impl Grid<two::Light> {
    pub fn brightness(&self) -> usize {
        self.lights
            .iter()
            .flatten()
            .map(|light| light.brightness)
            .sum()
    }

    pub fn instruct(&mut self, instruction: Instruction) {
        for row in instruction.from.y..=instruction.to.y {
            for column in instruction.from.x..=instruction.to.x {
                let light = &mut self.lights[row][column];
                match instruction.kind {
                    InstructionKind::TurnOff => {
                        light.brightness = light.brightness.saturating_sub(1)
                    }
                    InstructionKind::TurnOn => light.brightness += 1,
                    InstructionKind::Toggle => light.brightness += 2,
                }
            }
        }
    }
}

pub struct GridIterator<'a, L> {
    iterator: std::iter::Flatten<std::slice::Iter<'a, Vec<L>>>,
}

impl<'a, L> Iterator for GridIterator<'a, L> {
    type Item = &'a L;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}
