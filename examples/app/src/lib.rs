use {gloo::console, wasm_bindgen::prelude::*};

#[wasm_bindgen(start)]
fn start() {
    if tauri_wasm::is_tauri() {
        console::log!("tauri was detected!");
    } else {
        console::warn!("tauri was not detected!");
    }
}
