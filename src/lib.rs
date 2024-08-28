mod utils;

use rand::rngs::OsRng;
use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn next(lo: u32, hi: u32) -> u32 {
    let mut rng = OsRng; // Using OsRng for high-quality entropy
    let random_number: u32 = rng.gen(); // Generate a random u32 number

    random_number % (hi - lo) + lo
}
