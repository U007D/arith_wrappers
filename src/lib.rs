#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
// Safety-critical application lints
#![deny(
    bare_trait_objects,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used,
    clippy::pedantic
)]
// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]
#![allow(clippy::match_bool)]

// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing license files and more
//#![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
//#![deny(warnings)]

mod consts;
mod error;
mod wrapping;
pub use error::{Error, Result};
pub use wrapping::WrappingWrapper;
