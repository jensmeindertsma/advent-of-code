use std::fmt;

#[derive(PartialEq)]
pub enum Position {
    Regular { height: u8 },
    Start,
    Destination,
}

impl From<char> for Position {
    fn from(character: char) -> Self {
        match character {
            'S' => Self::Start,
            'E' => Self::Destination,
            other => Self::Regular {
                height: other as u8 - 97,
            },
        }
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Start => 'S'.to_string(),
                Self::Destination => 'E'.to_string(),
                Self::Regular { height } => height.to_string(),
            }
        )
    }
}
