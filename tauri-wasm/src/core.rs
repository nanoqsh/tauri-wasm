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
/// # async fn e() -> Result<(), tauri_wasm::invoke::Error> {
/// use gloo::console;
///
/// let message = tauri_wasm::invoke("connect").await?;
/// console::log!("connected to backend", message);
/// # Ok(())
/// # }
/// ```
#[inline]
pub async fn invoke<C>(cmd: C) -> Result<JsValue, Error>
where
    C: ToStringValue,
{
    let cmd = cmd.to_string_value();
    let args = JsValue::NULL;

    ext::invoke(cmd.as_ref(), args.as_ref(), None)
        .await
        .map_err(Error::Invoke)
}

/// Sends a message with arguments to the backend.
///
/// # Example
///
#[cfg_attr(feature = "serde", doc = "```")]
#[cfg_attr(not(feature = "serde"), doc = "```ignore")]
/// # async fn e() -> Result<(), tauri_wasm::invoke::Error> {
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
/// let message = tauri_wasm::invoke_with_args("login", Data(user)).await?;
/// console::log!("logged on backend", message);
/// # Ok(())
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

    ext::invoke(cmd.as_ref(), args.as_ref(), None)
        .await
        .map_err(Error::Invoke)
}

/// Sends a message with arguments and options to the backend.
///
/// # Example
///
#[cfg_attr(feature = "serde", doc = "```")]
#[cfg_attr(not(feature = "serde"), doc = "```ignore")]
/// # async fn e() -> Result<(), tauri_wasm::invoke::Error> {
/// use {gloo::console, tauri_wasm::invoke::Options};
///
/// let opts = Options::from_record([("secret", "37")])?;
/// let message = tauri_wasm::invoke_with_options("send", &[], opts).await?;
/// console::log!("received from backend", message);
/// # Ok(())
/// # }
/// ```
#[inline]
pub async fn invoke_with_options<C, A>(cmd: C, args: A, opts: Options) -> Result<JsValue, Error>
where
    C: ToStringValue,
    A: ToArgs,
{
    let cmd = cmd.to_string_value();
    let args = args.to_args().map_err(Error::Args)?;

    ext::invoke(cmd.as_ref(), args.as_ref(), Some(opts))
        .await
        .map_err(Error::Invoke)
}

pub trait ToStringValue {
    type JsValue: AsRef<JsValue> + Into<JsValue>;
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

#[wasm_bindgen]
pub struct Options {
    pub(crate) headers: JsValue,
}

#[wasm_bindgen]
impl Options {
    #[inline]
    #[wasm_bindgen(getter)]
    pub fn headers(self) -> JsValue {
        self.headers
    }
}

pub trait ToHeaders {
    fn to_headers(self) -> Result<JsValue, JsValue>;

    #[inline]
    fn to_options(self) -> Result<Options, Error>
    where
        Self: Sized,
    {
        let headers = self.to_headers().map_err(Error::Headers)?;
        Ok(Options { headers })
    }
}

#[derive(Debug)]
pub enum Error {
    Invoke(JsValue),
    Args(JsValue),
    Headers(JsValue),
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
        let (Error::Invoke(js) | Error::Args(js) | Error::Headers(js)) = e;
        js
    }
}

impl AsRef<JsValue> for Error {
    #[inline]
    fn as_ref(&self) -> &JsValue {
        let (Self::Invoke(js) | Self::Args(js) | Self::Headers(js)) = self;
        js
    }
}
