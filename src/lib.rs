#![crate_type = "lib"]
#![doc = include_str!("../README.md")]

#[doc(inline)]
pub use crate::translator::{TranslateOptions, Translator};
#[doc(inline)]
pub use crate::translator::errors::TranslatorErrors;

pub mod translator;