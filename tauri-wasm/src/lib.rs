#![doc = include_str!("../README.md")]
#![warn(clippy::missing_inline_in_public_items)]

mod core;
#[cfg(feature = "serde")]
mod event;
mod ext;
#[cfg(feature = "headers")]
mod headers;
#[cfg(feature = "serde")]
mod serde;

pub use crate::{
    core::{
        Error, Options, ToArgs, ToHeaders, ToStringValue, invoke, invoke_with_args,
        invoke_with_options,
    },
    ext::is_tauri,
};

#[cfg(feature = "serde")]
pub use crate::{
    event::{EventTarget, emit, emit_to},
    serde::Data,
};
