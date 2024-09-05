use crate::prelude::*;

mod entry;
mod loader;
mod persistence;
mod topic;

pub use entry::*;
pub use loader::*;
pub use persistence::*;
pub use topic::*;

pub struct Journal<LOADER>
where
    LOADER: JournalLoaderTrait,
{
    /// directory realative to the mdbook `SUMMARY.md`
    source_root: PathBuf,
    /// All of the topics tracked by journal
    topics: TopicMap,
    /// Responsible for saving and loading entries
    persistence: LOADER::DataDriver,
}

impl<LOADER> Journal<LOADER>
where
    LOADER: JournalLoaderTrait,
{
    pub fn install(config: LOADER::ConfigSource) -> Result<()> {
        LOADER::install(config)
    }

    pub fn load(config: LOADER::ConfigSource) -> Result<Self> {
        let (persistence, topics, source_root) = LOADER::load(config)?;

        Ok(Self {
            source_root,
            persistence,
            topics,
        })
    }

    pub fn with_topic<T>(&self, topic: &T) -> Result<&Topic>
    where
        T: AsRef<str>,
    {
        self.topics
            .find(topic)
            .with_context(|| format!("Topic Not Found [{}]", topic.as_ref()))
    }

    pub fn each_topic(&self) -> impl Iterator<Item = &Topic> {
        self.topics.iter()
    }

    pub fn persist_entry(&self, entry: &Entry) -> Result<PathBuf> {
        let topic = self.with_topic(&entry.topic_name())?;
        let file_location = self.source_root.join(topic.source_path(entry)?);
        let data = &self.persistence.serialize(entry)?;
        self.persistence.persist(&file_location, data)?;
        Ok(file_location)
    }

    pub fn fetch_entry(&self, path: &Path) -> Result<Entry> {
        self.persistence.fetch(path)
    }

    pub fn entries_for_topic<T>(&self, topic: &T) -> Result<Vec<Entry>>
    where
        T: AsRef<str>,
    {
        let mut entries = self
            .persistence
            .query(&Query::ForTopic(self.with_topic(topic)?))?;
        self.hydrate_virtual_paths(&mut entries)?;
        Ok(entries)
    }

    pub fn all_entries(&self) -> Result<Vec<Entry>> {
        let mut entries = self.persistence.query(&Query::AllEntries)?;
        self.hydrate_virtual_paths(&mut entries)?;
        Ok(entries)
    }

    // PRIVATE METHODS

    fn hydrate_virtual_paths(&self, entries: &mut [Entry]) -> Result<()> {
        for entry in entries {
            let path = self.with_topic(&entry.topic_name())?.virtual_path(entry)?;
            entry.virtual_path = Some(path);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use crate::support::prelude::*;
    use pretty_assertions::assert_eq;

    #[rstest]
    fn full_generation() -> Result<()> {
        let journal: Journal<MockJournalLoaderTrait> = Journal {
            persistence: FilePersistence::new("/tmp/mdbook-journal-test"),
            source_root: "/tmp/mdbook-journal-test".into(),
            topics: TopicMap::default().insert(
                Topic::builder("code-blog")
                    .add_variable(Variable::new("title").required())
                    .build(),
            )?,
        };

        let topic = journal.with_topic(&"code-blog")?;
        assert_eq!("code-blog", topic.name());

        let mut adapter = MockEntryGenerationTrait::new();

        adapter
            .expect_created_at()
            .returning(|| Ok(Utc.with_ymd_and_hms(2024, 10, 19, 16, 20, 0).unwrap()));

        adapter
            .expect_collect_value()
            .withf(|var| var.key() == "title")
            .returning(|_| Ok(Some(MetaValue::String("Test Entry".to_owned()))));

        let entry = topic.generate_entry(&adapter)?;

        assert_eq!(entry.topic_name(), "code-blog");
        assert_eq!(entry.created_at().year(), 2024);
        assert_eq!(entry.created_at().month(), 10);
        assert_eq!(
            entry.meta_value(&"title").unwrap(),
            &MetaValue::String("Test Entry".to_owned())
        );
        assert_eq!(entry.content(), "");

        let file_location = journal.persist_entry(&entry)?;
        let reloaded = journal.fetch_entry(&file_location)?;

        assert_eq!(entry.topic_name(), reloaded.topic_name());
        assert_eq!(entry.created_at(), reloaded.created_at());
        assert_eq!(entry.content(), reloaded.content());
        assert_eq!(entry.meta_value(&"title"), reloaded.meta_value(&"title"));
        assert_eq!(&file_location, reloaded.file_location().unwrap());

        let entries = journal.entries_for_topic(&"code-blog")?;
        assert_eq!(entry.meta_value(&"title"), entries[0].meta_value(&"title"));

        let entries = journal.all_entries()?;
        assert_eq!(entry.meta_value(&"title"), entries[0].meta_value(&"title"));
        Ok(())
    }
}
