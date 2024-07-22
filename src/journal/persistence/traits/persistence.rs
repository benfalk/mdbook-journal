use crate::prelude::*;

#[derive(Debug)]
pub enum Query<'a> {
    /// Find all for a provided topic name
    ForTopic(&'a Topic),
    /// Fetch all entries
    AllEntries,
}

pub trait PersistenceTrait {
    type Serialized;

    fn persist(&self, path: &Path, data: &Self::Serialized) -> Result<()>;

    fn load(&self, path: &Path) -> Result<(PathBuf, Self::Serialized)>;

    fn serialize(&self, entry: &Entry) -> Result<Self::Serialized>;

    fn deserialize(&self, data: Self::Serialized) -> Result<Entry>;

    fn execute(&self, query: &Query) -> Result<Vec<(PathBuf, Self::Serialized)>>;

    fn fetch(&self, path: &Path) -> Result<Entry> {
        let (file_path, data) = self.load(path)?;
        let mut entry = self.deserialize(data)?;
        entry.file_loc = Some(file_path);
        Ok(entry)
    }

    fn query(&self, query: &Query) -> Result<Vec<Entry>> {
        self.execute(query)?
            .into_iter()
            .try_fold(vec![], |mut items, (path, data)| {
                let mut entry = self.deserialize(data)?;
                entry.file_loc = Some(path);
                items.push(entry);
                Ok(items)
            })
    }
}
