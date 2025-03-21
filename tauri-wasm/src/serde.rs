use {
    crate::core::{Error, Options, ToArgs},
    serde::{Serialize, Serializer as _, ser::SerializeStruct},
    serde_wasm_bindgen::Serializer,
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

    #[inline]
    fn to_args(self) -> Result<Self::JsValue, JsValue> {
        (&self).to_args()
    }
}

impl<T> ToArgs for &Data<T>
where
    T: Serialize + ?Sized,
{
    type JsValue = JsValue;

    #[inline]
    fn to_args(self) -> Result<Self::JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0).map_err(JsValue::from)
    }
}

impl Options {
    #[inline]
    pub fn from_record<'val, I>(fields: I) -> Result<Self, Error>
    where
        I: IntoIterator<IntoIter: ExactSizeIterator, Item = (&'static str, &'val str)>,
    {
        let fields = fields.into_iter();
        let error = |e| Error::Headers(JsValue::from(e));

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
