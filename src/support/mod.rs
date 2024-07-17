//! # Development Support
//!
//! This provides internal testing and development
//! support.  All functionality here is only available
//! via `#[cfg(test)]`.

pub mod prelude {
    pub use mockall::{mock, predicate::*};
    pub use rstest::{fixture, rstest};
}
