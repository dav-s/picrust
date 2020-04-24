use crate::orientation::Orientation;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Selection {
    pub orientation: Orientation,
    pub index: i32,
}
