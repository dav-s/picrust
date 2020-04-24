mod board;
mod orientation;
mod selection;
mod slice;
mod state;
use crate::board::Board;

fn main() {
    let b = Board::new(
        vec![vec![1, 1, 1], vec![1], vec![4], vec![2], vec![3]],
        vec![vec![1, 1], vec![1], vec![1, 1, 1], vec![3], vec![2, 2]],
    );
    println!("{}", b);
}
