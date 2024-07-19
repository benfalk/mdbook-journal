use crate::prelude::*;

#[cfg_attr(test, automock(type Serialized=String;))]
pub trait EntryPersistenceTrait {
    type Serialized;

    fn persist(&self, path: &Path, data: &Self::Serialized) -> Result<()>;

    fn load(&self, path: &Path) -> Result<Self::Serialized>;

    fn serialize(&self, entry: &Entry) -> Result<Self::Serialized>;

    fn deserialize(&self, data: Self::Serialized) -> Result<Entry>;
}
