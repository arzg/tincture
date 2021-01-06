//! A crate for converting colors between different color spaces.

#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]

mod oklab;
mod rgb;

pub use oklab::Oklab;
pub use rgb::Rgb;
