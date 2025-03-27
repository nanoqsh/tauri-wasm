use {crate::core::Options, wasm_bindgen::prelude::*};

#[wasm_bindgen(module = "/core.js")]
extern "C" {
    /// Checks whether Tauri environment is detected.
    ///
    /// # Example
    ///
    /// ```
    /// # fn e() {
    /// use gloo::console;
    ///
    /// if tauri_wasm::is_tauri() {
    ///     console::log!("tauri was detected!");
    /// } else {
    ///     console::error!("tauri was not detected!");
    /// }
    /// # }
    /// ```
    pub fn is_tauri() -> bool;

    pub(crate) fn eargs(event: &JsValue, payload: &JsValue, k: u32, l: &JsValue) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], catch)]
    pub(crate) async fn invoke(
        cmd: &JsValue,
        args: &JsValue,
        opts: Option<Options>,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = String)]
    pub(crate) fn to_string(value: &JsValue) -> String;
}
