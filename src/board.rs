use crate::orientation::Orientation;
use crate::selection::Selection;
use crate::slice::Slice;
use crate::state::GridState;

pub struct Board {
    states: Vec<Vec<GridState>>,
    rows: Vec<Vec<i32>>,
    columns: Vec<Vec<i32>>,
}

impl Board {
    pub fn new(rows: Vec<Vec<i32>>, columns: Vec<Vec<i32>>) -> Self {
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
