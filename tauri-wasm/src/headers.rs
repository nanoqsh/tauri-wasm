use {
    crate::{Error, invoke::IntoHeaders},
    wasm_bindgen::JsValue,
    web_sys::Headers,
};

impl IntoHeaders for Headers {
    #[inline]
    fn into_headers(self) -> Result<JsValue, Error> {
        Ok(JsValue::from(self))
    }
}
