//! # Development Support
//!
//! This provides internal testing and development
//! support.  All functionality here is only available
//! via `#[cfg(test)]`.
pub mod fixtures;

pub mod prelude {
    pub use chrono::{Datelike, TimeZone};
    pub use mockall::{mock, predicate::*};
    pub use rstest::{fixture, rstest};
}
