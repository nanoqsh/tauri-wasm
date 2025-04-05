use {
    crate::ext,
    std::{error, fmt},
    wasm_bindgen::JsValue,
};

#[derive(Debug)]
pub struct Error(pub(crate) JsValue);

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        ext::to_string(&self.0).fmt(f)
    }
}

impl error::Error for Error {}

impl From<Error> for JsValue {
    #[inline]
    fn from(e: Error) -> Self {
        e.0
    }
}
