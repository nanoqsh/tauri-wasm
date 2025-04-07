<div align="center">
    <h1>tauri-wasm</h1>
    <p>
        The <a href="https://github.com/tauri-apps/tauri">tauri</a> framework library for pure rust frontend with <a href="https://github.com/rustwasm/wasm-bindgen">wasm-bindgen</a>
    </p>
    <p>
        <a href="https://crates.io/crates/tauri-wasm"><img src="https://img.shields.io/crates/v/tauri-wasm.svg"></img></a>
        <a href="https://docs.rs/tauri-wasm"><img src="https://docs.rs/tauri-wasm/badge.svg"></img></a>
        <a href="https://github.com/nanoqsh/tauri-wasm/actions"><img src="https://github.com/nanoqsh/tauri-wasm/workflows/ci/badge.svg"></img></a>
    </p>
</div>

## About

Interact with a Tauri backend using the pure Rust library.
You don't need NPM or any other JavaScript tools to build a frontend, use Cargo instead.

This crate is designed for Tauri version 2.0 and above.

## Getting Started

Let's write the frontend part:

```rust
use {
    gloo::console,
    wasm_bindgen::prelude::*,
};

#[wasm_bindgen]
pub async fn start() -> Result<(), JsError> {
    // first, check if we are running in a Tauri environment
    if !tauri_wasm::is_tauri() {
        console::error!("tauri was not detected!");
        return;
    }

    // invoke the "hello" command on the backend
    let message = tauri_wasm::invoke("hello").await?;

    // log the response from the backend
    console::log!("message: ", message);

    Ok(())
}
```

When `wasm_bindgen` attribute exports our `start` function to the JS runtime, we can call it from frontend.

On the backend part let's write this:

```rust
// define the "hello" command
#[tauri::command]
fn hello() -> String {
    println!("frontend invoked hello");
    "message from backend".to_owned()
}

// configure the backend
pub fn run() {
    tauri::Builder::default()
        // register the "hello" command
        .invoke_handler(tauri::generate_handler![hello])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

For more details, see the [example](https://github.com/nanoqsh/tauri-wasm/tree/main/examples) in the repository.
