use crate::prelude::*;

#[cfg_attr(test, automock(
    type ConfigSource=String;
    type DataDriver=FilePersistence;
))]
pub trait JournalLoaderTrait {
    type ConfigSource;
    type DataDriver: PersistenceTrait;

    fn load(config_source: Self::ConfigSource) -> Result<(Self::DataDriver, TopicMap, PathBuf)>;

    fn install(config_source: Self::ConfigSource) -> Result<()>;
}
