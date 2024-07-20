use crate::prelude::*;

pub struct CliLoader {}

impl JournalLoaderTrait for CliLoader {
    type ConfigSource = PathBuf;
    type DataDriver = EntryFilePersistence;

    fn load(path: Self::ConfigSource) -> Result<(Self::DataDriver, TopicMap, PathBuf)> {
        let config = crate::mdbook::config::load(&path)?;

        Ok((
            EntryFilePersistence {},
            crate::mdbook::dto::TopicMapDto::try_from(&config)
                .context("loading topics dto")?
                .try_into()
                .context("converting topics dto")?,
            path.parent()
                .with_context(|| format!("invalid path `{}`", path.display()))?
                .join(config.book.src),
        ))
    }

    fn install(path: Self::ConfigSource) -> Result<()> {
        crate::mdbook::config::install(&path)
    }
}
