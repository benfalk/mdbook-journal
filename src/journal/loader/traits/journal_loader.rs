use crate::prelude::*;

#[cfg_attr(test, automock(
    type ConfigSource=String;
    type DataDriver=EntryFilePersistence;
))]
pub trait JournalLoaderTrait {
    type ConfigSource;
    type DataDriver: EntryPersistenceTrait;

    fn load(config_source: Self::ConfigSource) -> Result<(Self::DataDriver, TopicMap, PathBuf)>;

    fn install(config_source: Self::ConfigSource) -> Result<()>;
}
