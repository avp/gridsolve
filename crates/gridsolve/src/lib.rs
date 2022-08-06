mod constraint;
mod puzzle;
mod rule;
mod solver;

pub use puzzle::{Puzzle, PuzzleError};
pub use solver::{solve, Solution};
