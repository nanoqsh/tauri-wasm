mod core;
mod ext;
#[cfg(feature = "serde")]
mod serde;

pub mod invoke {
    pub use crate::core::{IntoStringValue, InvokeArgs, InvokeOptions};
}

pub use crate::{
    core::{Error, invoke, invoke_with_args, invoke_with_options},
    ext::is_tauri,
};

#[cfg(feature = "serde")]
pub use crate::serde::Data;
