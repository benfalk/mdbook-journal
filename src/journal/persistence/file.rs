use crate::prelude::*;

mod encoding;
mod query;

#[derive(Debug)]
pub struct FilePersistence {
    root: PathBuf,
}

impl FilePersistence {
    pub fn new<P>(root: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self { root: root.into() }
    }
}

impl PersistenceTrait for FilePersistence {
    type Serialized = String;

    fn serialize(&self, entry: &Entry) -> Result<Self::Serialized> {
        encoding::encode(entry)
    }

    fn deserialize(&self, string: Self::Serialized) -> Result<Entry> {
        encoding::decode(string)
    }

    fn load(&self, path: &Path) -> Result<(PathBuf, Self::Serialized)> {
        Ok((path.into(), std::fs::read_to_string(path)?))
    }

    fn persist(&self, path: &Path, data: &Self::Serialized) -> Result<()> {
        let directory = path
            .parent()
            .with_context(|| format!("preparing directories for {path:?}"))?;
        std::fs::create_dir_all(directory)
            .with_context(|| format!("writing directorys for {path:?}"))?;
        std::fs::write(path, data).with_context(|| format!("saving {path:?}"))?;
        Ok(())
    }

    fn execute(&self, query: &Query) -> Result<Vec<(PathBuf, Self::Serialized)>> {
        query::run(&self.root, query)
    }
}
