#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum GridState {
    Unknown,
    Filled,
    None,
    OutOfBounds,
}

impl GridState {
    fn to_char(&self) -> char {
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Orientation {
    Row,
    Column,
}

impl Orientation {
    fn invert(&self) -> Orientation {
        match self {
            Self::Row => Self::Column,
            Self::Column => Self::Row,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Selection {
    orientation: Orientation,
    index: usize,
}

struct Slice {
    selection: Selection,
    states: Vec<GridState>,
}

impl Slice {
    fn merge(a: Option<Self>, b: Self) -> Option<Self> {
        let a = a?;
        Some(Self {
            selection: a.selection,
            states: a
                .states
                .into_iter()
                .zip(b.states.into_iter())
                .map(|(sa, sb)| if sa == sb { sa } else { GridState::Unknown })
                .collect(),
        })
    }

    fn get_updates(a: Self, b: Self) -> Vec<Selection> {
        (1..a.states.len())
            .filter_map(|i| {
                if a.states[i] != b.states[i] {
                    Some(Selection {
                        orientation: a.selection.orientation.invert(),
                        index: i,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

struct Board {
    states: Vec<Vec<GridState>>,
    rows: Vec<Vec<usize>>,
    columns: Vec<Vec<usize>>,
}

impl Board {
    fn new(rows: Vec<Vec<usize>>, columns: Vec<Vec<usize>>) -> Self {
        let n = rows.len();
        let m = columns.len();
        Self {
            states: vec![vec![GridState::Unknown; m]; n],
            rows: rows,
            columns: columns,
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.states.iter().try_for_each(|row| {
            row.iter()
                .try_for_each(|state| write!(f, "{} ", state))
                .and_then(|_| writeln!(f))
        })
    }
}

fn main() {
    let b = Board::new(
        vec![vec![1, 1, 1], vec![1], vec![4], vec![2], vec![3]],
        vec![vec![1, 1], vec![1], vec![1, 1, 1], vec![3], vec![2, 2]],
    );
    println!("{}", b);
}
