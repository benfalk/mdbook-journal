use crate::prelude::*;

#[derive(Debug)]
pub(crate) struct DirMapper<'a> {
    topic: &'a Topic,
}

impl<'a> DirMapper<'a> {
    pub(super) fn new(topic: &'a Topic) -> Self {
        Self { topic }
    }

    pub fn map(&self, entry: &Entry) -> Result<PathBuf> {
        let year_month = entry.created_at().format("%Y/%B").to_string();
        Ok(year_month.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::support::prelude::*;
    use chrono::{TimeZone, Utc};

    #[rstest]
    fn map_works() {
        let date = Utc.with_ymd_and_hms(2024, 7, 15, 16, 20, 0).unwrap();
        let entry = Entry::builder("test").created_at(date).build();
        let topic = Topic::builder("test").build();
        let mapper = DirMapper::new(&topic);
        let path = mapper.map(&entry).unwrap();
        assert_eq!(path, PathBuf::from("2024/July"))
    }
}
