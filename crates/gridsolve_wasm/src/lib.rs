use gridsolve::{solve, Puzzle};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
#[wasm_bindgen]
pub struct PuzzleError {
    error: String,
}

impl PuzzleError {
    fn from_str(str: String) -> JsValue {
        JsValue::from_serde(&PuzzleError { error: str }).unwrap()
    }
}

#[wasm_bindgen]
pub fn solve_puzzle(input: &str) -> Result<String, JsValue> {
    let puzzle = Puzzle::parse(input).map_err(|e| PuzzleError::from_str(e.to_string()))?;
    let solution = solve(&puzzle).unwrap();
    Ok(serde_json::to_string(&solution).unwrap())
}
