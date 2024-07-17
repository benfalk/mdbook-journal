use std::path::PathBuf;

use once_cell::sync::{Lazy, OnceCell};
use serde_yaml::Value;

use super::Topic;

use crate::prelude::*;

static DEFAULT_TITLE: Lazy<Value> = Lazy::new(|| Value::String("Untitled".to_owned()));

#[derive(Debug)]
pub struct FilenameMapper<'a> {
    topic: &'a Topic,
}

impl<'a> FilenameMapper<'a> {
    pub(super) fn new(topic: &'a Topic) -> Self {
        Self { topic }
    }

    pub fn map(&self, entry: &Entry) -> Result<PathBuf> {
        let mut filename = entry.created_at().format("%d-%H-%M-%S ").to_string();
        filename.push_str(
            entry
                .meta_value(&"title")
                .unwrap_or_else(|| &DEFAULT_TITLE)
                .as_str()
                .unwrap_or("Untitled"),
        );
        let mut filename: PathBuf = filename.from_case(Case::Title).to_case(Case::Kebab).into();
        filename.set_extension("md");
        Ok(filename)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::support::prelude::*;
    use chrono::{TimeZone, Utc};

    #[rstest]
    fn map_works() {
        let date = Utc.with_ymd_and_hms(2024, 7, 1, 16, 20, 0).unwrap();
        let entry = Entry::builder("test")
            .created_at(date)
            .add_meta_value("title", "Code Times")
            .build();
        let topic = Topic::builder("test").build();
        let mapper = FilenameMapper::new(&topic);
        let path = mapper.map(&entry).unwrap();
        assert_eq!(path, PathBuf::from("01-16-20-00-code-times.md"))
    }
}
