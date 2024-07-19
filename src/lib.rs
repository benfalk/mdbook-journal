#![allow(dead_code, unused_imports)]

mod journal;

#[cfg(test)]
mod support;

pub use journal::*;

mod prelude {
    pub(crate) use super::*;

    pub(crate) use anyhow::{bail, Context, Result};
    pub(crate) use chrono::{DateTime, Utc};
    pub(crate) use convert_case::{Case, Casing};
    pub(crate) use once_cell::sync::{Lazy, OnceCell};
    pub(crate) use serde::{Deserialize, Serialize};

    pub(crate) use std::collections::BTreeMap;
    pub(crate) use std::path::{Path, PathBuf};
    pub(crate) use std::sync::Arc;

    #[cfg(test)]
    pub(crate) use mockall::automock;

    pub type UtcDateTime = DateTime<Utc>;
}
