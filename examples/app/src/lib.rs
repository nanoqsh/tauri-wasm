use {gloo::console, wasm_bindgen::prelude::*};

async fn hello() {
    use {serde::Serialize, tauri_wasm::Data};

    #[derive(Serialize)]
    struct Send {
        message: &'static str,
    }

    let s = Send { message: "front!" };

    match tauri_wasm::invoke_with_args("hello", Data(s)).await {
        Ok(message) => console::log!("message from frontend received", message),
        Err(e) => console::error!("failed to receive a message", e),
    }
}

#[wasm_bindgen]
pub async fn close() {
    if let Err(e) = tauri_wasm::invoke("close").await {
        console::error!("failed to close the application", e);
    }
}

#[wasm_bindgen(start)]
async fn start() {
    if tauri_wasm::is_tauri() {
        console::log!("tauri was detected!");
    } else {
        console::warn!("tauri was not detected!");
    }

    hello().await;
}
