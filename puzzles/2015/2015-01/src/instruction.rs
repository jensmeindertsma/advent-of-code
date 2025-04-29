pub enum Instruction {
    Ascend,
    Descend,
}

impl TryFrom<char> for Instruction {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Self::Ascend),
            ')' => Ok(Self::Descend),
            other => Err(other),
        }
    }
}
