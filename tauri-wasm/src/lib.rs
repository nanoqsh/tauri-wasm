#![warn(clippy::missing_inline_in_public_items)]
#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

mod error;
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
mod event;
mod ext;
#[cfg(feature = "headers")]
#[cfg_attr(docsrs, doc(cfg(feature = "headers")))]
mod headers;
pub mod invoke;
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
mod serde;

pub use crate::{error::Error, ext::is_tauri, invoke::api::invoke};

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub use crate::{
    event::{EventTarget, emit, emit_to},
    serde::args,
};
