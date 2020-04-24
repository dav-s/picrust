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
    index: i32,
}

struct Slice {
    selection: Selection,
    states: Vec<GridState>,
}

impl Slice {
    fn get(&self, index: i32) -> GridState {
        if 0 <= index && (index as usize) < self.states.len() {
            self.states[index as usize]
        } else {
            GridState::OutOfBounds
        }
    }

    fn can_fit(&self, start: i32, length: i32) -> bool {
        !(self.get(start - 1) == GridState::Filled
            || (start..start + length)
                .any(|i| (self.get(i) == GridState::None || self.get(i) == GridState::OutOfBounds))
            || self.get(start + length) == GridState::Filled)
    }

    fn finalize(&self) -> Self {
        Self {
            selection: self.selection,
            states: self
                .states
                .iter()
                .map(|s| match s {
                    GridState::Filled => GridState::Filled,
                    _ => GridState::None,
                })
                .collect(),
        }
    }

    fn place_segment(&self, start: i32, length: i32) -> Self {
        Self {
            selection: self.selection,
            states: self
                .states
                .iter()
                .cloned()
                .enumerate()
                .map(|(i, s)| {
                    let i = i as i32;
                    if i == start - 1 {
                        GridState::None
                    } else if start <= i && i < start + length {
                        GridState::Filled
                    } else if i == start + length {
                        GridState::None
                    } else {
                        s
                    }
                })
                .collect(),
        }
    }

    fn get_segments(&self) -> Vec<i32> {
        let mut segments = Vec::new();
        let mut i: i32 = 0;
        while (i as usize) < self.states.len() {
            let mut segment: i32 = 0;
            while self.get(i) == GridState::Filled {
                i += 1;
                segment += 1;
            }
            if segment > 0 {
                segments.push(segment);
            }
            i += 1;
        }
        if segments.len() > 0 {
            segments
        } else {
            vec![0]
        }
    }

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
        (0..a.states.len())
            .filter_map(|i| {
                if a.states[i] != b.states[i] {
                    Some(Selection {
                        orientation: a.selection.orientation.invert(),
                        index: i as i32,
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
    rows: Vec<Vec<i32>>,
    columns: Vec<Vec<i32>>,
}

impl Board {
    fn new(rows: Vec<Vec<i32>>, columns: Vec<Vec<i32>>) -> Self {
        let n = rows.len();
        let m = columns.len();
        Self {
            states: vec![vec![GridState::Unknown; m]; n],
            rows: rows,
            columns: columns,
        }
    }

    fn get_segments(&self, selection: Selection) -> Vec<i32> {
        match selection.orientation {
            Orientation::Row => self.rows[selection.index as usize].clone(),
            Orientation::Column => self.columns[selection.index as usize].clone(),
        }
    }

    fn extract_slice(&self, selection: Selection) -> Slice {
        Slice {
            selection: selection,
            states: match selection.orientation {
                Orientation::Row => self.states[selection.index as usize].clone(),
                Orientation::Column => (0..self.rows.len())
                    .map(|j| self.states[j][selection.index as usize])
                    .collect(),
            },
        }
    }

    fn apply_slice(&mut self, slice: Slice) {
        let i = slice.selection.index as usize;
        match slice.selection.orientation {
            Orientation::Row => {
                for j in 0..self.columns.len() {
                    self.states[i][j] = slice.states[j];
                }
            }

            Orientation::Column => {
                for j in 0..self.rows.len() {
                    self.states[j][i] = slice.states[j];
                }
            }
        }
    }

    fn slice_is_completed(&self, slice: Slice) -> bool {
        slice
            .get_segments()
            .iter()
            .zip(self.get_segments(slice.selection).iter())
            .filter(|(a, b)| a != b)
            .count()
            == 0
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
