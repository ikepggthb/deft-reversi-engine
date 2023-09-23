
pub mod board;
pub mod ai;
mod bit;
mod eval;
mod learn;
// ---

pub use board::*;
pub use ai::*;
// use eval::*;
// use learn::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
