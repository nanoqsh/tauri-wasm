use {gloo::console, serde::Serialize, wasm_bindgen::prelude::*};

async fn hello() -> Result<(), JsError> {
    use serde::Serialize;

    #[derive(Serialize)]
    struct Send {
        message: &'static str,
    }

    let s = Send { message: "ping" };

    let args = tauri_wasm::args(&s)?;
    let message = tauri_wasm::invoke("hello").with_args(args).await?;
    console::log!("message from backend received", &message);

    if message == "ping" {
        Ok(())
    } else {
        Err(JsError::new("wrong backend message"))
    }
}

async fn headers() -> Result<(), JsError> {
    use tauri_wasm::Options;

    let opts = Options::from_record([
        ("app-0", "fi"), //
        ("app-1", "hi"), //
        ("app-2", "lo"), //
    ])?;

    let message = tauri_wasm::invoke("headers").with_options(opts).await?;
    console::log!("message from backend received", &message);

    if message == "fi.hi.lo" {
        Ok(())
    } else {
        Err(JsError::new("wrong backend message"))
    }
}

async fn event() -> Result<(), JsError> {
    use tauri_wasm::EventTarget;

    let target = EventTarget::from("app");
    tauri_wasm::emit_to(target, "test-event", "payload").await?;
    Ok(())
}

#[wasm_bindgen]
pub async fn close() {
    if let Err(e) = tauri_wasm::invoke("close").await {
        console::error!("failed to close the application", e);
    }
}

#[wasm_bindgen(start)]
async fn start() {
    if !tauri_wasm::is_tauri() {
        console::error!("tauri was not detected!");
        return;
    }

    console::log!("tauri was detected!");

    if let Err(e) = hello().await {
        console::error!("failed to call hello", e);
        return;
    }

    if let Err(e) = headers().await {
        console::error!("failed to call headers", e);
        return;
    }

    if let Err(e) = event().await {
        console::error!("failed to call event", e);
    }
}
