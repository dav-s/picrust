#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Orientation {
    Row,
    Column,
}

impl Orientation {
    pub fn invert(&self) -> Orientation {
        match self {
            Self::Row => Self::Column,
            Self::Column => Self::Row,
        }
    }
}
