use crate::prelude::*;

pub fn run(root: &Path, query: &Query) -> Result<Vec<(PathBuf, String)>> {
    match query {
        Query::ForTopic(topic) => private::entries(&root.join(topic.source_root())),
        Query::AllEntries => private::entries(root),
    }
}

mod private {
    use super::*;

    pub fn entries(path: &Path) -> Result<Vec<(PathBuf, String)>> {
        walkdir::WalkDir::new(path)
            .into_iter()
            .try_fold(vec![], |mut files, potential_path| {
                let some_path = potential_path?.into_path();
                if some_path.is_file() {
                    let data = std::fs::read_to_string(&some_path)?;
                    files.push((some_path, data));
                }
                Ok(files)
            })
    }
}
