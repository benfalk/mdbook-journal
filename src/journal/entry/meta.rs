pub use crate::prelude::*;

pub use serde_yaml::Value as MetaValue;

#[derive(Debug, Default, Serialize)]
#[serde(transparent)]
pub struct EntryMeta {
    data: BTreeMap<String, MetaValue>,
}

/// Entry Meta-Data
///
/// Collection of all non-reserved data found from
/// the front-matter of an entry.
///
impl EntryMeta {
    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<MetaValue>,
    {
        self.data.insert(key.into(), value.into());
    }

    pub fn get<K>(&self, key: &K) -> Option<&MetaValue>
    where
        K: AsRef<str>,
    {
        self.data.get(key.as_ref())
    }
}
