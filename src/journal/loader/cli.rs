use crate::prelude::*;

pub struct CliLoader {}

impl JournalLoaderTrait for CliLoader {
    type ConfigSource = PathBuf;
    type DataDriver = FilePersistence;

    fn load(path: Self::ConfigSource) -> Result<(Self::DataDriver, TopicMap, PathBuf)> {
        let config = crate::mdbook::config::load(&path)?;
        let path_root = path
            .parent()
            .with_context(|| format!("invalid path `{}`", path.display()))?
            .join(&config.book.src);
        let topics = crate::mdbook::dto::TopicMapDto::try_from(&config)
            .context("loading topics dto")?
            .try_into()
            .context("converting topics dto")?;
        let persistence = FilePersistence::new(path_root.clone());

        Ok((persistence, topics, path_root))
    }

    fn install(path: Self::ConfigSource) -> Result<()> {
        crate::mdbook::config::install(&path)
    }
}
