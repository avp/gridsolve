macro_rules! info {
    ($($ex:expr),*$(,)?) => {};
}
macro_rules! error {
    ($($ex:expr),*$(,)?) => {};
}
pub(crate) use error;
pub(crate) use info;

mod constraint;
mod puzzle;
mod rule;
mod solver;

pub use puzzle::{Puzzle, PuzzleError};
pub use solver::{solve, Solution};
