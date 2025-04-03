use {
    crate::invoke::{Error, Options, ToArgs},
    serde::{Serialize, Serializer as _, ser::SerializeStruct},
    serde_wasm_bindgen::Serializer,
    wasm_bindgen::JsValue,
};

/// Arbitrary serializable data for [`with_args`](crate::invoke::Invoke::with_args) function.
#[inline]
pub fn args<T>(args: &T) -> Result<impl ToArgs, Error>
where
    T: Serialize + ?Sized,
{
    struct Data(JsValue);

    impl ToArgs for Data {
        type JsValue = JsValue;

        fn to_args(self) -> Self::JsValue {
            self.0
        }
    }

    let data = serde_wasm_bindgen::to_value(args).map_err(|e| Error(JsValue::from(e)))?;
    Ok(Data(data))
}

impl Options {
    #[inline]
    pub fn from_record<'val, I>(fields: I) -> Result<Self, Error>
    where
        I: IntoIterator<IntoIter: ExactSizeIterator, Item = (&'static str, &'val str)>,
    {
        let fields = fields.into_iter();
        let error = |e| Error(JsValue::from(e));

        let ser = Serializer::new();
        let mut s = ser
            .serialize_struct("Record", fields.len())
            .map_err(error)?;

        for (key, val) in fields {
            s.serialize_field(key, val).map_err(error)?;
        }

        let headers = s.end().map_err(error)?;
        Ok(Self { headers })
    }
}
