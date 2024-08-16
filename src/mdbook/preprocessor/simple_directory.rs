use super::prelude::*;
use crate::prelude::*;

use crate::index::DirIndex;
use crate::mdbook::traits::*;

pub struct SimpleDirPreprocessor<T>
where
    T: JournalLoaderTrait,
{
    journal: Journal<T>,
}

impl<T> SimpleDirPreprocessor<T>
where
    T: JournalLoaderTrait,
{
    pub fn new<J>(journal: J) -> Self
    where
        J: Into<Journal<T>>,
    {
        Self {
            journal: journal.into(),
        }
    }
}

impl<T> Preprocessor for SimpleDirPreprocessor<T>
where
    T: JournalLoaderTrait,
{
    fn name(&self) -> &str {
        "Simple Directory Preprocessor"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let journal = &self.journal;
        let writing = &mut book;
        let mut section = writing
            .max_section_number()
            .and_then(|s| s.root())
            .unwrap_or_default();

        for topic in journal.each_topic() {
            let mut entries = journal.entries_for_topic(&topic.name())?;
            entries.sort_by(|a, b| b.created_at().cmp(a.created_at()));
            section.increment();
            let chapter = topic_chapter(topic, &entries, section.clone())?;
            writing.push_item(chapter);
        }

        Ok(book)
    }
}

fn topic_chapter(topic: &Topic, entries: &[Entry], section: SectionNumber) -> Result<BookItem> {
    let index = DirIndex::for_topic(entries, topic)?;
    let parents = vec![topic.name().to_owned()];
    let mut path = topic.virtual_root().clone();
    path.push("README.md");
    let sub_items = if index.is_empty() {
        vec![]
    } else {
        build_sub_items(
            topic,
            &index[topic.name()],
            parents,
            section.advance_level(),
        )
    };

    Ok(BookItem::Chapter(Chapter {
        sub_items,
        name: topic.name().to_owned(),
        path: Some(path),
        number: Some(section),
        ..Default::default()
    }))
}

fn build_sub_items(
    topic: &Topic,
    index: &DirIndex,
    parents: Vec<String>,
    mut section: SectionNumber,
) -> Vec<BookItem> {
    if index.is_leaf() {
        index
            .entries()
            .map(|entry| {
                section.increment();
                entry_chapter(topic, entry, parents.clone(), section.clone())
            })
            .collect()
    } else {
        index
            .children()
            .map(|(name, dir)| {
                let name = name.to_owned();
                let mut new_parents = parents.clone();
                new_parents.push(name.clone());
                let mut path: PathBuf = new_parents.join("/").into();
                section.increment();
                path.push("README.md");
                BookItem::Chapter(Chapter {
                    sub_items: build_sub_items(topic, dir, new_parents, section.advance_level()),
                    parent_names: parents.clone(),
                    number: Some(section.clone()),
                    name,
                    path: Some(path),
                    ..Default::default()
                })
            })
            .collect()
    }
}

fn entry_chapter(
    topic: &Topic,
    entry: &Entry,
    parents: Vec<String>,
    section: SectionNumber,
) -> BookItem {
    let name = match entry.meta_value(&"title") {
        Some(MetaValue::String(title)) => title.to_owned(),
        _ => String::from("Untitled"),
    };
    let content = entry.content().to_owned();

    BookItem::Chapter(Chapter {
        name,
        content,
        number: Some(section),
        path: topic.virtual_path(entry).ok(),
        source_path: entry.file_location().cloned(),
        parent_names: parents,
        ..Default::default()
    })
}
