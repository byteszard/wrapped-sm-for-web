use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sm3_digest(plain: &str) -> String {
    yarism::sm3::digest(plain)
}