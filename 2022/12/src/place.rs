use std::fmt;

#[derive(PartialEq)]
pub enum Place {
    Regular { height: u8 },
    Start,
    Destination,
}

impl Place {
    fn elevation(&self) -> u8 {
        match self {
            Self::Start => 0,
            Self::Destination => 25,
            Self::Regular { height } => *height,
        }
    }
}

impl From<char> for Place {
    fn from(character: char) -> Self {
        match character {
            'S' => Self::Start,
            'E' => Self::Destination,
            other => Self::Regular {
                height: other as u8 - b'a',
            },
        }
    }
}

impl fmt::Debug for Place {
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
