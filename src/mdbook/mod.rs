//! # `mdbook` Related Interfaces
//!
use crate::prelude::*;

mod prelude {
    pub(super) use ::mdbook::{
        book::{Book, BookItem, Chapter, SectionNumber},
        Config,
    };
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

/// All preprocessor systems for the mdBook system
/// are kept in this module.  These are reposible
/// for updating a `mdBook` with `Journal` data in
/// some way.
///
pub mod preprocessor;

/// Traits that serve to enhance the mdbook public
/// types with additional functionality.
///
pub mod traits;
