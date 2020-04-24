use crate::selection::Selection;
use crate::state::GridState;

pub struct Slice {
    pub selection: Selection,
    pub states: Vec<GridState>,
}

impl Slice {
    pub fn get(&self, index: i32) -> GridState {
        if 0 <= index && (index as usize) < self.states.len() {
            self.states[index as usize]
        } else {
            GridState::OutOfBounds
        }
    }

    pub fn can_fit(&self, start: i32, length: i32) -> bool {
        !(self.get(start - 1) == GridState::Filled
            || (start..start + length)
                .any(|i| (self.get(i) == GridState::None || self.get(i) == GridState::OutOfBounds))
            || self.get(start + length) == GridState::Filled)
    }

    pub fn finalize(&self) -> Self {
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

    pub fn place_segment(&self, start: i32, length: i32) -> Self {
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

    pub fn get_segments(&self) -> Vec<i32> {
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

    pub fn merge(a: Option<Self>, b: Self) -> Option<Self> {
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

    pub fn get_updates(a: Self, b: Self) -> Vec<Selection> {
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
