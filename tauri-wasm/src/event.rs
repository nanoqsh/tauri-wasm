use {
    crate::{
        core::{Error, ToStringValue},
        ext,
    },
    js_sys::JsString,
    serde::Serialize,
    wasm_bindgen::prelude::*,
};

#[rustfmt::skip]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(thread_local_v2, static_string)]
    static EMIT: JsString = "plugin:event|emit";

     #[wasm_bindgen(thread_local_v2, static_string)]
    static EMIT_TO: JsString = "plugin:event|emit_to";
}

#[inline]
pub async fn emit<E, P>(event: E, payload: &P) -> Result<(), Error>
where
    E: ToStringValue,
    P: Serialize + ?Sized,
{
    #[derive(Serialize)]
    struct Emit<'pl, P>
    where
        P: ?Sized,
    {
        #[serde(with = "serde_wasm_bindgen::preserve")]
        event: JsValue,
        payload: &'pl P,
    }

    let event = event.to_string_value().as_ref().clone();
    let emit = Emit { event, payload };

    let cmd = EMIT.with(|s| JsValue::from(s));
    let args = serde_wasm_bindgen::to_value(&emit).map_err(|e| Error::Args(JsValue::from(e)))?;

    ext::invoke(&cmd, &args, None)
        .await
        .map_err(Error::Invoke)?;

    Ok(())
}

#[inline]
pub async fn emit_to<E, P>(target: EventTarget, event: E, payload: &P) -> Result<(), Error>
where
    E: ToStringValue,
    P: Serialize + ?Sized,
{
    #[derive(Serialize)]
    struct Target {
        #[serde(with = "serde_wasm_bindgen::preserve")]
        kind: JsValue,
        #[serde(with = "serde_wasm_bindgen::preserve")]
        label: JsString,
    }

    #[derive(Serialize)]
    struct EmitTo<'pl, P>
    where
        P: ?Sized,
    {
        target: Target,
        #[serde(with = "serde_wasm_bindgen::preserve")]
        event: JsValue,
        payload: &'pl P,
    }

    let event = event.to_string_value().as_ref().clone();
    let emit = EmitTo {
        target: Target {
            kind: JsValue::from(target.kind),
            label: target.label,
        },
        event,
        payload,
    };

    let cmd = EMIT_TO.with(|s| JsValue::from(s));
    let args = serde_wasm_bindgen::to_value(&emit).map_err(|e| Error::Args(JsValue::from(e)))?;

    ext::invoke(&cmd, &args, None)
        .await
        .map_err(Error::Invoke)?;

    Ok(())
}

#[derive(Clone)]
pub struct EventTarget {
    pub kind: EventKind,
    pub label: JsString,
}

impl EventTarget {
    #[inline]
    pub fn from_str(s: &str) -> Self {
        Self::from_js_string(JsString::from(s))
    }

    #[inline]
    pub fn from_js_string(label: JsString) -> Self {
        let kind = EventKind::AnyLabel;
        Self { kind, label }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum EventKind {
    Any = "Any",
    AnyLabel = "AnyLabel",
    App = "App",
    Window = "Window",
    Webview = "Webview",
    WebviewWindow = "WebviewWindow",
}
