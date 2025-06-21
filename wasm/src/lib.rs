extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn multiply(a: i8, b: i8) -> i8 {
    a * b
}
