#![warn(clippy::missing_inline_in_public_items)]

mod core;
mod ext;
#[cfg(feature = "headers")]
mod headers;
#[cfg(feature = "serde")]
mod serde;

pub mod invoke {
    pub use crate::core::{Error, Options, ToArgs, ToHeaders, ToStringValue};
}

pub use crate::{
    core::{invoke, invoke_with_args, invoke_with_options},
    ext::is_tauri,
};

#[cfg(feature = "serde")]
pub use crate::serde::Data;
