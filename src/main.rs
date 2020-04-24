enum SquareState {
    Unknown,
    Filled,
    None,
    OutOfBounds,
}

impl std::fmt::Display for SquareState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Unknown     => "?",
            Self::Filled      => "■",
            Self::None        => " ",
            Self::OutOfBounds => "X"
        })
    }
}

fn main() {
    println!("Hello, world!");
    println!("{}", SquareState::Filled);
}
