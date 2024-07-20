//! # `mdbook` Related Interfaces
//!
use crate::prelude::*;

mod prelude {
    pub(super) use ::mdbook::{Config, MDBook};
}

/// Util functions for working with the `book.toml`
/// configuration file.  The path for all functions
/// is expected to resolve to the actual file itself
/// and `NOT` the directory it is contained in.
///
pub mod config;

/// Util structures designed to more closely match
/// the data structures of serialized data that
/// serve to covert foreign data.
///
pub mod dto;
