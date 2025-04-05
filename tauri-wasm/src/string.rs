use {js_sys::JsString, wasm_bindgen::JsValue};

/// A value that can be represented as a JS string.
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
