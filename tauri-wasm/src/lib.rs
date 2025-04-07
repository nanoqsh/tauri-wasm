#![warn(clippy::missing_inline_in_public_items)]
#![cfg_attr(all(doc, not(doctest)), doc = include_str!("../README.md"))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

mod error;
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub mod event;
mod ext;
#[cfg(feature = "headers")]
#[cfg_attr(docsrs, doc(cfg(feature = "headers")))]
mod headers;
pub mod invoke;
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
mod serde;
mod string;

pub use crate::{error::Error, ext::is_tauri, invoke::api::invoke, string::ToStringValue};

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub use crate::{event::api::emit, serde::args};
