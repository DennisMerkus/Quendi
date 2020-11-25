use wasm_bindgen::prelude::*;

extern crate quenya;

#[wasm_bindgen]
pub fn annotate(text: &str) -> String {
    String::from(quenya::annotate(text))
}
