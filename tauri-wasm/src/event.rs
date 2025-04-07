//! Types of tauri [event system].
//!
//! [event system]: https://v2.tauri.app/develop/calling-rust/#event-system

use {
    crate::{error::Error, ext, invoke::Options, string::ToStringValue},
    js_sys::{JsString, Promise},
    serde::Serialize,
    std::{
        pin::Pin,
        task::{Context, Poll},
    },
    wasm_bindgen::prelude::*,
    wasm_bindgen_futures::JsFuture,
};

#[rustfmt::skip]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(thread_local_v2, static_string)]
    static EMIT: JsString = "plugin:event|emit";

     #[wasm_bindgen(thread_local_v2, static_string)]
    static EMIT_TO: JsString = "plugin:event|emit_to";
}

pub(crate) mod api {
    use super::*;

    /// Sends an [event] to the backend.
    ///
    /// [event]: https://v2.tauri.app/develop/calling-rust/#event-system
    ///
    /// # Example
    ///
    /// Send an event with string payload.
    ///
    #[cfg_attr(feature = "serde", doc = "```")]
    #[cfg_attr(not(feature = "serde"), doc = "```ignore")]
    /// # async fn e() -> Result<(), tauri_wasm::Error> {
    /// tauri_wasm::emit("file-selected", "/path/to/file")?.await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// You can send any [serializable](Serialize) payload.
    ///
    #[cfg_attr(feature = "serde", doc = "```")]
    #[cfg_attr(not(feature = "serde"), doc = "```ignore")]
    /// # async fn e() -> Result<(), tauri_wasm::Error> {
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct Message {
    ///     key: &'static str,
    ///     data: u32,
    /// }
    ///
    /// let message = Message {
    ///     key: "secret",
    ///     data: 37,
    /// };
    ///
    /// tauri_wasm::emit("file-selected", &message)?.await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// To trigger an event to a listener registered by a specific target
    /// you can use the [`to`](Emit::to) function.
    ///
    /// # Capabilities
    ///
    /// Note that in order to emit events, the Tauri framework
    /// requires the corresponding capabilities to be enabled.
    /// For example, let's say our application and its window
    /// are named "app". Then your `Tauri.toml` config should
    /// include something like:
    ///
    /// ```toml
    /// [app]
    /// # app configs..
    ///
    /// [[app.security.capabilities]]
    /// identifier = "default"
    /// windows = ["app"]
    /// permissions = ["core:event:default"]
    /// ```
    #[inline]
    pub fn emit<E, P>(event: E, payload: &P) -> Result<Emit<E::Js>, Error>
    where
        E: ToStringValue,
        P: Serialize + ?Sized,
    {
        let event = event.to_string_value();
        let payload =
            serde_wasm_bindgen::to_value(&payload).map_err(|e| Error(JsValue::from(e)))?;
        let target = None;

        Ok(Emit {
            event,
            payload,
            target,
        })
    }
}

/// A type used to configure an [emit](api::emit) operation.
pub struct Emit<E, T = JsValue> {
    event: E,
    payload: JsValue,
    target: Option<EventTarget<T>>,
}

impl<E> Emit<E> {
    /// Sends an [event] to a listener registered by a specific target.
    ///
    /// [event]: https://v2.tauri.app/develop/calling-rust/#event-system
    ///
    /// # Example
    ///
    /// Send an event with string payload to the target with "editor" label.
    ///
    #[cfg_attr(feature = "serde", doc = "```")]
    #[cfg_attr(not(feature = "serde"), doc = "```ignore")]
    /// # async fn e() -> Result<(), tauri_wasm::Error> {
    /// use tauri_wasm::event::EventTarget;
    ///
    /// let target = EventTarget::from("editor");
    /// tauri_wasm::emit("file-selected", "/path/to/file")?.to(target).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn to<S>(self, target: EventTarget<S>) -> Emit<E, S::Js>
    where
        S: ToStringValue,
    {
        let event = self.event;
        let payload = self.payload;
        let target = Some(target.map(|s| s.to_string_value()));

        Emit {
            event,
            payload,
            target,
        }
    }
}

/// Represents the future of an [emit](api::emit) operation.
pub struct EmitFuture(JsFuture);

impl EmitFuture {
    /// Returns the inner future.
    #[inline]
    pub fn into_future(self) -> JsFuture {
        self.0
    }
}

impl Future for EmitFuture {
    type Output = Result<JsValue, Error>;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.get_mut();
        Pin::new(&mut me.0).poll(cx).map_err(Error)
    }
}

impl<E, T> IntoFuture for Emit<E, T>
where
    E: AsRef<JsValue>,
    T: AsRef<JsValue>,
{
    type Output = Result<JsValue, Error>;
    type IntoFuture = EmitFuture;

    #[inline]
    fn into_future(self) -> Self::IntoFuture {
        let target = self.target.as_ref().map(|s| s.as_ref().map(|s| s.as_ref()));
        let promise = invoke_emit(target, self.event.as_ref(), &self.payload);
        EmitFuture(JsFuture::from(promise))
    }
}

fn invoke_emit(
    target: Option<EventTarget<&JsValue>>,
    event: &JsValue,
    payload: &JsValue,
) -> Promise {
    let cmd = if target.is_none() { &EMIT } else { &EMIT_TO };

    let (kind, label) = match target {
        None => (0, &JsValue::UNDEFINED),
        Some(target) => match target {
            EventTarget::Any => (1, &JsValue::UNDEFINED),
            EventTarget::AnyLabel(s) => (2, s),
            EventTarget::App => (3, &JsValue::UNDEFINED),
            EventTarget::Window(s) => (4, s),
            EventTarget::Webview(s) => (5, s),
            EventTarget::WebviewWindow(s) => (6, s),
        },
    };

    let cmd = cmd.with(|s| JsValue::from(s));
    let args = ext::eargs(event, payload, kind, label);
    ext::invoke(&cmd, &args, Options::empty())
}

/// An argument of event target for the [`to`](Emit::to) function.
pub enum EventTarget<S> {
    Any,
    AnyLabel(S),
    App,
    Window(S),
    Webview(S),
    WebviewWindow(S),
}

impl<S> EventTarget<S> {
    #[inline]
    pub fn from_string(s: S) -> Self {
        Self::AnyLabel(s)
    }

    #[inline]
    pub fn as_ref(&self) -> EventTarget<&S> {
        match self {
            Self::Any => EventTarget::Any,
            Self::AnyLabel(s) => EventTarget::AnyLabel(s),
            Self::App => EventTarget::App,
            Self::Window(s) => EventTarget::Window(s),
            Self::Webview(s) => EventTarget::Webview(s),
            Self::WebviewWindow(s) => EventTarget::WebviewWindow(s),
        }
    }

    #[inline]
    pub fn map<F, T>(self, f: F) -> EventTarget<T>
    where
        F: FnOnce(S) -> T,
    {
        match self {
            Self::Any => EventTarget::Any,
            Self::AnyLabel(s) => EventTarget::AnyLabel(f(s)),
            Self::App => EventTarget::App,
            Self::Window(s) => EventTarget::Window(f(s)),
            Self::Webview(s) => EventTarget::Webview(f(s)),
            Self::WebviewWindow(s) => EventTarget::WebviewWindow(f(s)),
        }
    }
}

impl From<&str> for EventTarget<JsString> {
    #[inline]
    fn from(s: &str) -> Self {
        Self::from_string(JsString::from(s))
    }
}
