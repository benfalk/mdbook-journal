#![allow(dead_code, unused_imports)]

#[cfg(test)]
mod support;

mod journal;

pub use journal::*;

mod prelude {
    pub use super::*;

    pub use anyhow::{bail, Context, Result};
    pub use chrono::{DateTime, Utc};
    pub use convert_case::{Case, Casing};

    pub use std::collections::BTreeMap;
    pub use std::path::PathBuf;
    pub use std::sync::Arc;

    #[cfg(test)]
    pub use mockall::automock;

    pub type UtcDateTime = DateTime<Utc>;
}
