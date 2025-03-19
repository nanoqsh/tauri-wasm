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
#[inline]
pub async fn invoke<C>(cmd: C) -> Result<JsValue, Error>
where
    C: ToStringValue,
{
    let cmd = cmd.to_string_value();
    let args = JsValue::NULL;
    let opts = JsValue::NULL;
    ext::invoke(cmd.as_ref(), args.as_ref(), opts.as_ref())
        .await
        .map_err(Error::Invoke)
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
/// struct User<'str> {
///     name: &'str str,
///     pass: &'str str,
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
#[inline]
pub async fn invoke_with_args<C, A>(cmd: C, args: A) -> Result<JsValue, Error>
where
    C: ToStringValue,
    A: ToArgs,
{
    let cmd = cmd.to_string_value();
    let args = args.to_args().map_err(Error::Args)?;
    let opts = JsValue::NULL;
    ext::invoke(cmd.as_ref(), args.as_ref(), opts.as_ref())
        .await
        .map_err(Error::Invoke)
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
#[inline]
pub async fn invoke_with_options<C, A, O>(cmd: C, args: A, opts: O) -> Result<JsValue, Error>
where
    C: ToStringValue,
    A: ToArgs,
    O: ToOptions,
{
    let cmd = cmd.to_string_value();
    let args = args.to_args().map_err(Error::Args)?;
    let opts = opts.to_options().map_err(Error::Options)?;
    ext::invoke(cmd.as_ref(), args.as_ref(), opts.as_ref())
        .await
        .map_err(Error::Invoke)
}

pub trait ToStringValue {
    type JsValue: AsRef<JsValue>;
    fn to_string_value(self) -> Self::JsValue;
}

impl ToStringValue for JsString {
    type JsValue = JsValue;

    #[inline]
    fn to_string_value(self) -> Self::JsValue {
        JsValue::from(self)
    }
}

impl<'rf> ToStringValue for &'rf JsString {
    type JsValue = &'rf JsValue;

    #[inline]
    fn to_string_value(self) -> Self::JsValue {
        self
    }
}

impl ToStringValue for String {
    type JsValue = JsValue;

    #[inline]
    fn to_string_value(self) -> Self::JsValue {
        (&self).to_string_value()
    }
}

impl ToStringValue for &String {
    type JsValue = JsValue;

    #[inline]
    fn to_string_value(self) -> Self::JsValue {
        JsValue::from(self)
    }
}

impl ToStringValue for &str {
    type JsValue = JsValue;

    #[inline]
    fn to_string_value(self) -> Self::JsValue {
        JsValue::from(self)
    }
}

pub trait ToArgs {
    type JsValue: AsRef<JsValue>;
    fn to_args(self) -> Result<Self::JsValue, JsValue>;
}

impl ToArgs for ArrayBuffer {
    type JsValue = JsValue;

    #[inline]
    fn to_args(self) -> Result<Self::JsValue, JsValue> {
        Ok(JsValue::from(self))
    }
}

impl<'rf> ToArgs for &'rf ArrayBuffer {
    type JsValue = &'rf JsValue;

    #[inline]
    fn to_args(self) -> Result<Self::JsValue, JsValue> {
        Ok(self)
    }
}

impl ToArgs for Uint8Array {
    type JsValue = JsValue;

    #[inline]
    fn to_args(self) -> Result<Self::JsValue, JsValue> {
        Ok(JsValue::from(self))
    }
}

impl<'rf> ToArgs for &'rf Uint8Array {
    type JsValue = &'rf JsValue;

    #[inline]
    fn to_args(self) -> Result<Self::JsValue, JsValue> {
        Ok(self)
    }
}

impl ToArgs for &[u8] {
    type JsValue = JsValue;

    #[inline]
    fn to_args(self) -> Result<Self::JsValue, JsValue> {
        Uint8Array::from(self).to_args()
    }
}

impl<const N: usize> ToArgs for &[u8; N] {
    type JsValue = JsValue;

    #[inline]
    fn to_args(self) -> Result<Self::JsValue, JsValue> {
        self.as_slice().to_args()
    }
}

pub trait ToOptions {
    fn to_options(self) -> Result<JsValue, JsValue>;
}

#[derive(Debug)]
pub enum Error {
    Invoke(JsValue),
    Args(JsValue),
    Options(JsValue),
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        ext::to_string(self.as_ref()).fmt(f)
    }
}

impl error::Error for Error {}

impl From<Error> for JsValue {
    #[inline]
    fn from(e: Error) -> Self {
        let (Error::Invoke(js) | Error::Args(js) | Error::Options(js)) = e;
        js
    }
}

impl AsRef<JsValue> for Error {
    #[inline]
    fn as_ref(&self) -> &JsValue {
        let (Self::Invoke(js) | Self::Args(js) | Self::Options(js)) = self;
        js
    }
}
