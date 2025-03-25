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
    static EVENT_CMD: JsString = "plugin:event|emit";
}

#[derive(Serialize)]
struct Emit<'pl, P>
where
    P: ?Sized,
{
    #[serde(with = "serde_wasm_bindgen::preserve")]
    event: JsValue,
    payload: &'pl P,
}

#[inline]
pub async fn emit<E, P>(event: E, payload: &P) -> Result<(), Error>
where
    E: ToStringValue,
    P: Serialize + ?Sized,
{
    let event = event.to_string_value().into();
    let emit = Emit { event, payload };

    let cmd = EVENT_CMD.with(|s| JsValue::from(s));
    let args = serde_wasm_bindgen::to_value(&emit).map_err(|e| Error::Args(JsValue::from(e)))?;

    ext::invoke(&cmd, &args, None)
        .await
        .map_err(Error::Invoke)?;

    Ok(())
}
