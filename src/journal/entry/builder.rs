use crate::prelude::*;

#[derive(Debug)]
pub struct EntryBuilder {
    entry: Entry,
}

impl AsRef<Entry> for EntryBuilder {
    fn as_ref(&self) -> &Entry {
        &self.entry
    }
}

impl EntryBuilder {
    pub(super) fn new<S>(topic: S) -> EntryBuilder
    where
        S: Into<TopicName>,
    {
        Self {
            entry: Entry {
                topic: topic.into(),
                content: String::new(),
                meta: EntryMeta::default(),
                created_at: Utc::now(),
                file_loc: None,
            },
        }
    }

    pub fn created_at<D>(mut self, created_at: D) -> Self
    where
        D: Into<UtcDateTime>,
    {
        self.entry.created_at = created_at.into();
        self
    }

    pub fn content<S>(mut self, content: S) -> Self
    where
        S: Into<String>,
    {
        self.entry.content = content.into();
        self
    }

    pub fn file_name<N>(mut self, file_name: N) -> Self
    where
        N: Into<PathBuf>,
    {
        let filename: PathBuf = file_name.into();
        self.entry.file_loc = filename.file_name().map(Into::into);
        self
    }

    pub fn add_meta(mut self, meta: EntryMeta) -> Self {
        self.entry.meta = meta;
        self
    }

    pub fn add_meta_value<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<MetaValue>,
    {
        self.entry.meta.insert(key, value);
        self
    }

    pub fn build(self) -> Entry {
        self.entry
    }
}
