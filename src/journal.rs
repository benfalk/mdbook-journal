mod entry;
mod topic;

use crate::prelude::*;

pub use entry::*;
pub use topic::*;

pub struct Journal {
    /// directory realative to the mdbook `SUMMARY.md`
    source_root: PathBuf,
    /// All of the topics tracked by journal
    topics: TopicMap,
}

impl Journal {
    pub fn with_topic<T>(&self, topic: &T) -> Result<&Topic>
    where
        T: AsRef<str>,
    {
        self.topics
            .find(topic)
            .with_context(|| format!("Topic Not Found [{}]", topic.as_ref()))
    }

    pub fn persist(&self, entry: &Entry) -> Result<PathBuf> {
        let topic = self.with_topic(&entry.topic_name())?;

        let file_location = self
            .source_root
            .join(topic.source_root())
            .join(topic.dir_mapper().map(entry)?)
            .join(topic.filename_mapper().map(entry)?);

        // TODO: persist; maybe with an adapter trait
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use crate::support::prelude::*;

    #[rstest]
    fn full_generation() -> Result<()> {
        let journal = Journal {
            source_root: "/tmp/mdbook-journal-test".into(),
            topics: TopicMap::default().insert(
                Topic::builder("code-blog")
                    .add_variable(Variable::new("title").required())
                    .build(),
            )?,
        };

        let topic = journal.with_topic(&"code-blog")?;
        assert_eq!("code-blog", topic.name());

        // TODO: let test_generation = TestEntryGeneration::new();
        // TODO: let entry = topic.generate_entry(test_generation);
        // TODO: let file_location = journal.persist(&entry)?;
        // TODO: let reloaded = journal.load(&file_location)?;

        Ok(())
    }
}
