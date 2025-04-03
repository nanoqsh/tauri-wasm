use {crate::invoke::ToHeaders, wasm_bindgen::JsValue, web_sys::Headers};

impl ToHeaders for Headers {
    #[inline]
    fn to_headers(self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from(self))
    }
}
