mod builder;
mod meta;

use crate::prelude::*;

pub use builder::*;
pub use meta::*;

/// Journal Entry
///
/// Represents a single markdown file created from a `Topic`
///
#[derive(Debug, Clone, Serialize)]
pub struct Entry {
    /// Topic identifier to which this entry belongs
    #[serde(serialize_with = "serialize_topic_name")]
    topic: TopicName,
    /// Time this entry was created
    #[serde(serialize_with = "serialize_created_at")]
    created_at: UtcDateTime,
    /// Location where this Entry is persisted to disk
    pub(crate) file_loc: Option<PathBuf>,
    pub(crate) virtual_path: Option<PathBuf>,
    /// Additional data found in the entries front-matter
    meta: EntryMeta,
    /// The contents of the markdown file except
    /// for the front-matter
    content: String,
}

impl Entry {
    pub(crate) fn builder<S>(topic: S) -> EntryBuilder
    where
        S: Into<TopicName>,
    {
        EntryBuilder::new(topic)
    }

    pub fn created_at(&self) -> &UtcDateTime {
        &self.created_at
    }

    pub fn meta(&self) -> &EntryMeta {
        &self.meta
    }

    pub fn meta_value<K>(&self, key: &K) -> Option<&MetaValue>
    where
        K: AsRef<str>,
    {
        self.meta.get(key)
    }

    pub fn topic_name(&self) -> &str {
        self.topic.as_ref()
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn file_location(&self) -> Option<&PathBuf> {
        self.file_loc.as_ref()
    }
}

fn serialize_topic_name<S>(name: &TopicName, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(name.as_ref())
}

fn serialize_created_at<S>(
    date: &UtcDateTime,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let data = &date.to_rfc3339();
    serializer.serialize_str(data)
}
