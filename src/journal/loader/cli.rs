use anyhow::Ok;

use crate::prelude::*;

pub struct CliLoader {}

impl JournalLoaderTrait for CliLoader {
    type ConfigSource = PathBuf;
    type DataDriver = EntryFilePersistence;

    fn load(_: Self::ConfigSource) -> Result<(Self::DataDriver, TopicMap, PathBuf)> {
        Ok((
            EntryFilePersistence {},
            TopicMap::default().insert(
                Topic::builder("code-blog")
                    .add_variable(Variable::new("title").required())
                    .build(),
            )?,
            "/tmp/mdbook-journal-test-cli/".into(),
        ))
    }

    fn install(_: Self::ConfigSource) -> Result<()> {
        todo!()
    }
}
