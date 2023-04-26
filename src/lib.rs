use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));

}

#[wasm_bindgen]
pub fn add( n:u32 , m:u32) -> u32 {
    n + m
}