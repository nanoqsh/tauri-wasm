//! Types of tauri [commands].
//!
//! [commands]: https://v2.tauri.app/develop/calling-rust/#commands

use {
    crate::{error::Error, ext, string::ToStringValue},
    js_sys::{ArrayBuffer, Uint8Array},
    std::{
        pin::Pin,
        task::{Context, Poll},
    },
    wasm_bindgen::prelude::*,
    wasm_bindgen_futures::JsFuture,
};

pub(crate) mod api {
    use super::*;

    /// Invokes a [command] on the backend.
    ///
    /// [command]: https://v2.tauri.app/develop/calling-rust/#commands
    ///
    /// This function returns a future-like object that
    /// can be extended with additional properties.
    /// See [`with_args`](Invoke::with_args) and
    /// [`with_options`](Invoke::with_options) for details.
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
}

/// A type used to configure an [invoke](api::invoke) operation.
pub struct Invoke<C, A = JsValue> {
    cmd: C,
    args: A,
    opts: Options,
}

impl<C, A> Invoke<C, A> {
    /// Invokes a [command] with arguments on the backend.
    ///
    /// [command]: https://v2.tauri.app/develop/calling-rust/#commands
    ///
    /// # Passing a serializable type
    ///
    /// To send a custom serializable type as arguments,
    /// use the helper [`args`](crate::args) function.
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
    ///
    /// # Passing a JS object
    ///
    /// Thanks to the `wasm_bindgen` attribute
    /// you can convert your type into a JS value.
    /// To pass the value as arguments implement the [`ToArgs`] trait.
    ///
    /// ```
    /// # async fn e() -> Result<(), tauri_wasm::Error> {
    /// use {
    ///     gloo::console,
    ///     tauri_wasm::invoke::ToArgs,
    ///     wasm_bindgen::prelude::*,
    /// };
    ///
    /// #[wasm_bindgen(getter_with_clone)]
    /// struct User {
    ///     name: String,
    ///     pass: String,
    /// }
    ///
    /// impl ToArgs for User {
    ///     type Js = JsValue;
    ///
    ///     fn to_args(self) -> Self::Js {
    ///         // wasm_bindgen attribute implements
    ///         // convertion into JS value
    ///         JsValue::from(self)
    ///     }
    /// }
    ///
    /// let user = User {
    ///     name: "anon".to_owned(),
    ///     pass: "p@$$w0rD".to_owned(),
    /// };
    ///
    /// let message = tauri_wasm::invoke("login").with_args(user).await?;
    /// console::log!("logged on backend", message);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn with_args<T>(self, args: T) -> Invoke<C, T::Js>
    where
        T: ToArgs,
    {
        let cmd = self.cmd;
        let args = args.to_args();
        let opts = self.opts;
        Invoke { cmd, args, opts }
    }

    /// Invokes a [command] with options on the backend.
    ///
    /// [command]: https://v2.tauri.app/develop/calling-rust/#commands
    ///
    /// # Example
    ///
    #[cfg_attr(feature = "serde", doc = "```")]
    #[cfg_attr(not(feature = "serde"), doc = "```ignore")]
    /// # async fn e() -> Result<(), tauri_wasm::Error> {
    /// use {gloo::console, tauri_wasm::invoke::Options};
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

/// Represents the future of an [invoke](api::invoke) operation.
pub struct InvokeFuture(JsFuture);

impl InvokeFuture {
    /// Returns the inner future.
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

/// Types that can be represented as arguments.
pub trait ToArgs {
    type Js: AsRef<JsValue>;
    fn to_args(self) -> Self::Js;
}

impl ToArgs for ArrayBuffer {
    type Js = JsValue;

    #[inline]
    fn to_args(self) -> Self::Js {
        JsValue::from(self)
    }
}

impl<'arr> ToArgs for &'arr ArrayBuffer {
    type Js = &'arr JsValue;

    #[inline]
    fn to_args(self) -> Self::Js {
        self
    }
}

impl ToArgs for Uint8Array {
    type Js = JsValue;

    #[inline]
    fn to_args(self) -> Self::Js {
        JsValue::from(self)
    }
}

impl<'arr> ToArgs for &'arr Uint8Array {
    type Js = &'arr JsValue;

    #[inline]
    fn to_args(self) -> Self::Js {
        self
    }
}

impl ToArgs for &[u8] {
    type Js = JsValue;

    #[inline]
    fn to_args(self) -> Self::Js {
        Uint8Array::from(self).to_args()
    }
}

impl<const N: usize> ToArgs for &[u8; N] {
    type Js = JsValue;

    #[inline]
    fn to_args(self) -> Self::Js {
        self.as_slice().to_args()
    }
}

/// Invoke options.
///
/// To pass options to an invoke call, use the
/// [`with_options`](Invoke::with_options) method.
///
/// You can create options from
/// [headers](IntoHeaders::into_options).
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
    /// Returns options headers.
    #[inline]
    #[wasm_bindgen(getter)]
    pub fn headers(self) -> JsValue {
        self.headers
    }
}

/// Types that can be converted into headers.
pub trait IntoHeaders {
    /// Converts the value into headers.
    fn into_headers(self) -> Result<JsValue, Error>;

    /// Converts the value into [options](Options).
    #[inline]
    fn into_options(self) -> Result<Options, Error>
    where
        Self: Sized,
    {
        let headers = self.into_headers()?;
        Ok(Options { headers })
    }
}
