use {
    crate::core::{ToArgs, ToOptions},
    serde::Serialize,
    wasm_bindgen::JsValue,
};

pub struct Data<T>(pub T)
where
    T: ?Sized;

impl<T> ToArgs for Data<T>
where
    T: Serialize,
{
    type JsValue = JsValue;
    fn to_args(self) -> Result<Self::JsValue, JsValue> {
        (&self).to_args()
    }
}

impl<'rf, T> ToArgs for &'rf Data<T>
where
    T: Serialize + ?Sized,
{
    type JsValue = JsValue;
    fn to_args(self) -> Result<Self::JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0).map_err(JsValue::from)
    }
}

impl<T> ToOptions for Data<T>
where
    T: Serialize,
{
    fn to_options(self) -> Result<JsValue, JsValue> {
        (&self).to_options()
    }
}

impl<T> ToOptions for &Data<T>
where
    T: Serialize + ?Sized,
{
    fn to_options(self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0).map_err(JsValue::from)
    }
}
