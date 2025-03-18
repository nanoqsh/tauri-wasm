use {
    crate::core::{InvokeArgs, InvokeOptions},
    serde::Serialize,
    wasm_bindgen::JsValue,
};

pub struct Data<T>(pub T)
where
    T: ?Sized;

impl<T> InvokeArgs for Data<T>
where
    T: Serialize,
{
    fn invoke_args(self) -> Result<JsValue, JsValue> {
        (&self).invoke_args()
    }
}

impl<T> InvokeArgs for &Data<T>
where
    T: Serialize + ?Sized,
{
    fn invoke_args(self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0).map_err(JsValue::from)
    }
}

impl<T> InvokeOptions for Data<T>
where
    T: Serialize,
{
    fn invoke_options(self) -> Result<JsValue, JsValue> {
        (&self).invoke_options()
    }
}

impl<T> InvokeOptions for &Data<T>
where
    T: Serialize + ?Sized,
{
    fn invoke_options(self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0).map_err(JsValue::from)
    }
}
