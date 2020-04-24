#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum GridState {
    Unknown,
    Filled,
    None,
    OutOfBounds,
}

impl GridState {
    pub fn to_char(&self) -> char {
        match self {
            Self::Unknown => '?',
            Self::Filled => '■',
            Self::None => ' ',
            Self::OutOfBounds => 'X',
        }
    }
}

impl std::fmt::Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
