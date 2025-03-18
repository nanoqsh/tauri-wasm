use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/core.js")]
extern "C" {
    /// Checks whether Tauri environment is detected.
    pub fn is_tauri() -> bool;
}
