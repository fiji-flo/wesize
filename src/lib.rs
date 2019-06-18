extern crate wasm_bindgen;
extern crate resize;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn wesize(w1: usize, h1: usize, w2: usize, h2: usize, input: &[u8], output: &mut [u8]) {
    resize::resize(w1, h1, w2, h2, resize::Pixel::RGBA, resize::Type::Lanczos3, input, output)
}