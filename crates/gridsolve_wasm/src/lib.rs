use gridsolve::{solve, Puzzle};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PuzzleWrapper(Puzzle);

#[wasm_bindgen]
pub fn solve_puzzle(input: &str) -> Result<String, JsValue> {
    let puzzle = Puzzle::parse(input).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let solution = solve(&puzzle).unwrap();
    Ok(serde_json::to_string(&solution).unwrap())
}
