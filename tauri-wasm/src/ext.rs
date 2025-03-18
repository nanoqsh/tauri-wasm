use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/core.js")]
extern "C" {
    /// Checks whether Tauri environment is detected.
    pub fn is_tauri() -> bool;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], catch)]
    pub(crate) async fn invoke(
        cmd: JsValue,
        args: JsValue,
        opts: JsValue,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = String)]
    pub(crate) fn to_string(value: &JsValue) -> String;
}
