use {
    crate::ext,
    js_sys::{ArrayBuffer, JsString, Uint8Array},
    std::{error, fmt},
    wasm_bindgen::prelude::*,
};

/// Sends a message to the backend.
///
/// # Example
///
/// ```
/// # async fn invoke() {
/// use gloo::console;
///
/// if let Ok(message) = tauri_wasm::invoke("connect").await {
///     console::log!("connected to backend: {message}");
/// }
/// # }
/// ```
pub async fn invoke<C>(cmd: C) -> Result<JsValue, Error>
where
    C: IntoStringValue,
{
    let cmd = cmd.into_string_value();
    let args = JsValue::NULL;
    let opts = JsValue::NULL;
    ext::invoke(cmd, args, opts).await.map_err(Error::Invoke)
}

/// Sends a message with arguments to the backend.
///
/// # Example
///
#[cfg_attr(feature = "serde", doc = "```")]
#[cfg_attr(not(feature = "serde"), doc = "```ignore")]
/// # async fn invoke() {
/// use {gloo::console, serde::Serialize, tauri_wasm::Data};
///
/// #[derive(Serialize)]
/// struct User<'s> {
///     name: &'s str,
///     pass: &'s str,
/// }
///
/// let user = User {
///     name: "anon",
///     pass: "p@$$w0rD",
/// };
///
/// if let Ok(message) = tauri_wasm::invoke_with_args("login", Data(user)).await {
///     console::log!("logged on backend: {message}");
/// }
/// # }
/// ```
pub async fn invoke_with_args<C, A>(cmd: C, args: A) -> Result<JsValue, Error>
where
    C: IntoStringValue,
    A: InvokeArgs,
{
    let cmd = cmd.into_string_value();
    let args = args.invoke_args().map_err(Error::Args)?;
    let opts = JsValue::NULL;
    ext::invoke(cmd, args, opts).await.map_err(Error::Invoke)
}

/// Sends a message with arguments and options to the backend.
///
/// # Example
///
#[cfg_attr(feature = "serde", doc = "```")]
#[cfg_attr(not(feature = "serde"), doc = "```ignore")]
/// # async fn invoke() {
/// use {gloo::console, serde::Serialize, tauri_wasm::Data};
///
/// #[derive(Serialize)]
/// struct Options {
///     secret: u32,
/// }
///
/// let opts = Options {
///     secret: 37,
/// };
///
/// if let Ok(message) = tauri_wasm::invoke_with_options("send", &[], Data(opts)).await {
///     console::log!("received from backend: {message}");
/// }
/// # }
/// ```
pub async fn invoke_with_options<C, A, O>(cmd: C, args: A, opts: O) -> Result<JsValue, Error>
where
    C: IntoStringValue,
    A: InvokeArgs,
    O: InvokeOptions,
{
    let cmd = cmd.into_string_value();
    let args = args.invoke_args().map_err(Error::Args)?;
    let opts = opts.invoke_options().map_err(Error::Options)?;
    ext::invoke(cmd, args, opts).await.map_err(Error::Invoke)
}

pub trait IntoStringValue {
    fn into_string_value(self) -> JsValue;
}

impl IntoStringValue for JsString {
    fn into_string_value(self) -> JsValue {
        JsValue::from(self)
    }
}

impl IntoStringValue for &JsString {
    fn into_string_value(self) -> JsValue {
        JsValue::from(self)
    }
}

impl IntoStringValue for String {
    fn into_string_value(self) -> JsValue {
        JsValue::from(self)
    }
}

impl IntoStringValue for &String {
    fn into_string_value(self) -> JsValue {
        JsValue::from(self)
    }
}

impl IntoStringValue for &str {
    fn into_string_value(self) -> JsValue {
        JsValue::from(self)
    }
}

pub trait InvokeArgs {
    fn invoke_args(self) -> Result<JsValue, JsValue>;
}

impl InvokeArgs for ArrayBuffer {
    fn invoke_args(self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from(self))
    }
}

impl InvokeArgs for &ArrayBuffer {
    fn invoke_args(self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from(self))
    }
}

impl InvokeArgs for Uint8Array {
    fn invoke_args(self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from(self))
    }
}

impl InvokeArgs for &Uint8Array {
    fn invoke_args(self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from(self))
    }
}

impl InvokeArgs for &[u8] {
    fn invoke_args(self) -> Result<JsValue, JsValue> {
        Uint8Array::from(self).invoke_args()
    }
}

impl<const N: usize> InvokeArgs for &[u8; N] {
    fn invoke_args(self) -> Result<JsValue, JsValue> {
        self.as_slice().invoke_args()
    }
}

pub trait InvokeOptions {
    fn invoke_options(self) -> Result<JsValue, JsValue>;
}

pub struct Untype(pub JsValue);

impl IntoStringValue for Untype {
    fn into_string_value(self) -> JsValue {
        self.0
    }
}

impl InvokeArgs for Untype {
    fn invoke_args(self) -> Result<JsValue, JsValue> {
        Ok(self.0)
    }
}

impl InvokeOptions for Untype {
    fn invoke_options(self) -> Result<JsValue, JsValue> {
        Ok(self.0)
    }
}

#[derive(Debug)]
pub enum Error {
    Invoke(JsValue),
    Args(JsValue),
    Options(JsValue),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        ext::to_string(self.as_ref()).fmt(f)
    }
}

impl error::Error for Error {}

impl From<Error> for JsValue {
    fn from(e: Error) -> Self {
        let (Error::Invoke(js) | Error::Args(js) | Error::Options(js)) = e;
        js
    }
}

impl AsRef<JsValue> for Error {
    fn as_ref(&self) -> &JsValue {
        let (Self::Invoke(js) | Self::Args(js) | Self::Options(js)) = self;
        js
    }
}
