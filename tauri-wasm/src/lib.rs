#![doc = include_str!("../README.md")]
#![warn(clippy::missing_inline_in_public_items)]

mod error;
#[cfg(feature = "serde")]
mod event;
mod ext;
#[cfg(feature = "headers")]
mod headers;
pub mod invoke;
#[cfg(feature = "serde")]
mod serde;

pub use crate::{error::Error, ext::is_tauri, invoke::api::invoke};

#[cfg(feature = "serde")]
pub use crate::{
    event::{EventTarget, emit, emit_to},
    serde::args,
};
