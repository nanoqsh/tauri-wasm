use {
    crate::ext,
    js_sys::{ArrayBuffer, JsString, Uint8Array},
    std::{
        error, fmt,
        pin::Pin,
        task::{Context, Poll},
    },
    wasm_bindgen::prelude::*,
    wasm_bindgen_futures::JsFuture,
};

/// Sends a message to the backend.
///
/// # Example
///
/// ```
/// # async fn e() -> Result<(), tauri_wasm::Error> {
/// use gloo::console;
///
/// let message = tauri_wasm::invoke("connect").await?;
/// console::log!("connected to backend", message);
/// # Ok(())
/// # }
/// ```
#[inline]
pub fn invoke<C>(cmd: C) -> Invoke<C::Js>
where
    C: ToStringValue,
{
    let cmd = cmd.to_string_value();
    let args = JsValue::UNDEFINED;
    let opts = Options::empty();
    Invoke { cmd, args, opts }
}

pub struct Invoke<C, A = JsValue> {
    cmd: C,
    args: A,
    opts: Options,
}

impl<C> Invoke<C> {
    /// Sends a message with arguments to the backend.
    ///
    /// # Example
    ///
    #[cfg_attr(feature = "serde", doc = "```")]
    #[cfg_attr(not(feature = "serde"), doc = "```ignore")]
    /// # async fn e() -> Result<(), tauri_wasm::Error> {
    /// use {gloo::console, serde::Serialize};
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
    /// let args = tauri_wasm::args(&user)?;
    /// let message = tauri_wasm::invoke("login").with_args(args).await?;
    /// console::log!("logged on backend", message);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn with_args<A>(self, args: A) -> Invoke<C, A::JsValue>
    where
        A: ToArgs,
    {
        let cmd = self.cmd;
        let args = args.to_args();
        let opts = self.opts;
        Invoke { cmd, args, opts }
    }
}

impl<C, A> Invoke<C, A> {
    /// Sends a message with options to the backend.
    ///
    /// # Example
    ///
    #[cfg_attr(feature = "serde", doc = "```")]
    #[cfg_attr(not(feature = "serde"), doc = "```ignore")]
    /// # async fn e() -> Result<(), tauri_wasm::Error> {
    /// use {gloo::console, tauri_wasm::Options};
    ///
    /// let opts = Options::from_record([
    ///     ("secret", "2"),
    ///     ("data", "3"),
    /// ])?;
    ///
    /// let message = tauri_wasm::invoke("send").with_options(opts).await?;
    /// console::log!("received from backend", message);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn with_options(self, opts: Options) -> Self {
        Self { opts, ..self }
    }
}

pub struct InvokeFuture(JsFuture);

impl InvokeFuture {
    #[inline]
    pub fn into_future(self) -> JsFuture {
        self.0
    }
}

impl Future for InvokeFuture {
    type Output = Result<JsValue, Error>;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.get_mut();
        Pin::new(&mut me.0).poll(cx).map_err(Error)
    }
}

impl<C, A> IntoFuture for Invoke<C, A>
where
    C: AsRef<JsValue>,
    A: AsRef<JsValue>,
{
    type Output = Result<JsValue, Error>;
    type IntoFuture = InvokeFuture;

    #[inline]
    fn into_future(self) -> Self::IntoFuture {
        let promise = ext::invoke(self.cmd.as_ref(), self.args.as_ref(), self.opts);
        InvokeFuture(JsFuture::from(promise))
    }
}

pub trait ToStringValue {
    type Js: AsRef<JsValue>;
    fn to_string_value(self) -> Self::Js;
}

impl ToStringValue for JsString {
    type Js = JsValue;

    #[inline]
    fn to_string_value(self) -> Self::Js {
        JsValue::from(self)
    }
}

impl<'str> ToStringValue for &'str JsString {
    type Js = &'str JsValue;

    #[inline]
    fn to_string_value(self) -> Self::Js {
        self
    }
}

impl ToStringValue for String {
    type Js = JsValue;

    #[inline]
    fn to_string_value(self) -> Self::Js {
        (&self).to_string_value()
    }
}

impl ToStringValue for &String {
    type Js = JsValue;

    #[inline]
    fn to_string_value(self) -> Self::Js {
        JsValue::from(self)
    }
}

impl ToStringValue for &str {
    type Js = JsValue;

    #[inline]
    fn to_string_value(self) -> Self::Js {
        JsValue::from(self)
    }
}

pub trait ToArgs {
    type JsValue: AsRef<JsValue>;
    fn to_args(self) -> Self::JsValue;
}

impl ToArgs for ArrayBuffer {
    type JsValue = JsValue;

    #[inline]
    fn to_args(self) -> Self::JsValue {
        JsValue::from(self)
    }
}

impl<'arr> ToArgs for &'arr ArrayBuffer {
    type JsValue = &'arr JsValue;

    #[inline]
    fn to_args(self) -> Self::JsValue {
        self
    }
}

impl ToArgs for Uint8Array {
    type JsValue = JsValue;

    #[inline]
    fn to_args(self) -> Self::JsValue {
        JsValue::from(self)
    }
}

impl<'arr> ToArgs for &'arr Uint8Array {
    type JsValue = &'arr JsValue;

    #[inline]
    fn to_args(self) -> Self::JsValue {
        self
    }
}

impl ToArgs for &[u8] {
    type JsValue = JsValue;

    #[inline]
    fn to_args(self) -> Self::JsValue {
        Uint8Array::from(self).to_args()
    }
}

impl<const N: usize> ToArgs for &[u8; N] {
    type JsValue = JsValue;

    #[inline]
    fn to_args(self) -> Self::JsValue {
        self.as_slice().to_args()
    }
}

#[wasm_bindgen]
pub struct Options {
    pub(crate) headers: JsValue,
}

impl Options {
    pub(crate) const fn empty() -> Self {
        let headers = JsValue::UNDEFINED;
        Self { headers }
    }
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
        let headers = self.to_headers().map_err(Error)?;
        Ok(Options { headers })
    }
}

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
