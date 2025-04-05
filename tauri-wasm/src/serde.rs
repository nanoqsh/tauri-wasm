use {
    crate::{
        error::Error,
        invoke::{Options, ToArgs},
    },
    serde::{Serialize, Serializer as _, ser},
    serde_wasm_bindgen::Serializer,
    std::collections::HashMap,
    wasm_bindgen::JsValue,
};

/// Arbitrary serializable data for
/// [`with_args`](crate::invoke::Invoke::with_args) function.
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
    pub fn from_map<K, V>(map: &HashMap<K, V>) -> Result<Self, Error>
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        use ser::SerializeMap;

        let error = |e| Error(JsValue::from(e));

        let ser = Serializer::new();
        let mut s = ser.serialize_map(Some(map.len())).map_err(error)?;

        for (key, val) in map {
            s.serialize_entry(key.as_ref(), val.as_ref())
                .map_err(error)?;
        }

        let headers = s.end().map_err(error)?;
        Ok(Self { headers })
    }

    #[inline]
    pub fn from_record<'val, I>(fields: I) -> Result<Self, Error>
    where
        I: IntoIterator<IntoIter: ExactSizeIterator, Item = (&'static str, &'val str)>,
    {
        use ser::SerializeStruct;

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
