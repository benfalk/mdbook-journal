use crate::prelude::*;

/// Supporting structures
mod private;

use private::*;

#[derive(Debug, Default)]
pub struct DirIndex {
    depth: usize,
    entries: Vec<Arc<Entry>>,
    indices: BTreeMap<String, DirIndex>,
}

impl DirIndex {
    pub fn for_topic(entries: &[Entry], topic: &Topic) -> Result<Self> {
        let mut index = Self::default();
        let parts = &mut TopicParts::new(topic);

        for entry in entries {
            let mut current = &mut index;
            parts.load(entry)?;
            let shared_entry = Arc::new(entry.clone());
            for (idx, segment) in parts.iter_mut().enumerate() {
                current.entries.push(shared_entry.clone());
                if !current.indices.contains_key(segment) {
                    let dir = DirIndex {
                        depth: idx + 1,
                        ..Default::default()
                    };
                    current.indices.insert(segment.clone(), dir);
                }
                current = current.indices.get_mut(segment).unwrap();
            }
            current.entries.push(shared_entry);
        }

        Ok(index)
    }

    pub fn is_leaf(&self) -> bool {
        self.indices.is_empty()
    }

    pub fn children(&self) -> impl Iterator<Item = (&String, &DirIndex)> {
        self.indices.iter()
    }

    pub fn entries(&self) -> impl Iterator<Item = &Entry> {
        self.entries.iter().map(|entry| entry.as_ref())
    }
}

impl std::ops::Index<&str> for DirIndex {
    type Output = Self;

    fn index(&self, index: &str) -> &Self::Output {
        self.indices.get(index).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::support::fixtures::*;
    use crate::support::prelude::*;

    #[rstest]
    fn builds_correctly(entries: Vec<Entry>, topic: Topic) -> Result<()> {
        let index = DirIndex::for_topic(&entries, &topic)?;

        assert_eq!(index["test"]["2024"]["July"].entries().count(), 1);
        assert_eq!(index["test"]["2024"]["July"].children().count(), 0);

        assert_eq!(index["test"]["2024"]["June"].entries().count(), 1);
        assert_eq!(index["test"]["2024"]["June"].children().count(), 0);

        assert_eq!(index["test"]["2024"].entries().count(), 2);
        assert_eq!(index["test"]["2024"].children().count(), 2);

        assert_eq!(index["test"].children().count(), 1);
        assert_eq!(index["test"].entries().count(), 2);

        Ok(())
    }
}
