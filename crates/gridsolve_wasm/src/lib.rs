use gridsolve::{Puzzle, PuzzleError};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PuzzleWrapper(Puzzle);

#[wasm_bindgen]
pub fn puzzle(input: &str) -> Result<PuzzleWrapper, JsValue> {
    Puzzle::parse(input)
        .map_err(|e| JsValue::from_str(&e.to_string()))
        .map(PuzzleWrapper)
}
