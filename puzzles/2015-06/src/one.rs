#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Light {
    Off,
    On,
}

impl Default for Light {
    fn default() -> Self {
        Self::Off
    }
}
