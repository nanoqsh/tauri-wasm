use {
    crate::{
        ext,
        invoke::{Error, Options, ToStringValue},
    },
    js_sys::JsString,
    serde::Serialize,
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
/// tauri_wasm::emit("file-selected", "/path/to/file").await?;
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
/// tauri_wasm::emit("file-selected", &message).await?;
/// # Ok(())
/// # }
/// ```
///
/// To trigger an event to a listener registered by a specific target
/// you can use the [`emit_to`] function.
#[inline]
pub async fn emit<E, P>(event: E, payload: &P) -> Result<(), Error>
where
    E: ToStringValue,
    P: Serialize + ?Sized,
{
    let event = event.to_string_value();

    let payload = serde_wasm_bindgen::to_value(&payload).map_err(|e| Error(JsValue::from(e)))?;

    invoke_emit(None, event.as_ref(), &payload)
        .await
        .map_err(Error)?;

    Ok(())
}

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
/// use tauri_wasm::EventTarget;
///
/// let target = EventTarget::from("editor");
/// tauri_wasm::emit_to(target, "file-selected", "/path/to/file").await?;
/// # Ok(())
/// # }
/// ```
#[inline]
pub async fn emit_to<S, E, P>(target: EventTarget<S>, event: E, payload: &P) -> Result<(), Error>
where
    S: ToStringValue,
    E: ToStringValue,
    P: Serialize + ?Sized,
{
    let target = target.map(|s| s.to_string_value());
    let target = target.as_ref().map(|s| s.as_ref());
    let event = event.to_string_value();

    let payload = serde_wasm_bindgen::to_value(&payload).map_err(|e| Error(JsValue::from(e)))?;

    invoke_emit(Some(target), event.as_ref(), &payload)
        .await
        .map_err(Error)?;

    Ok(())
}

#[inline]
fn invoke_emit(
    target: Option<EventTarget<&JsValue>>,
    event: &JsValue,
    payload: &JsValue,
) -> JsFuture {
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
    JsFuture::from(ext::invoke(&cmd, &args, Options::empty()))
}

/// An argument of event target for the [`emit_to`] function.
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
