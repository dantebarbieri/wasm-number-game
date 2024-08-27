mod utils;

use wasm_bindgen::prelude::*;
use rand::rngs::OsRng;
use rand::Rng;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let mut rng = OsRng; // Using OsRng for high-quality entropy
    let random_number: u32 = rng.gen(); // Generate a random u32 number

    let greeting = format!("Hello, wasm-number-game! Your random number is: {}", random_number);
    alert(&greeting);
}
