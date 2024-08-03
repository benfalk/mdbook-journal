use super::prelude::*;
use crate::prelude::*;

pub struct NaivePreprocessor<T>
where
    T: JournalLoaderTrait,
{
    journal: Journal<T>,
}

impl<T> NaivePreprocessor<T>
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

impl<T> Preprocessor for NaivePreprocessor<T>
where
    T: JournalLoaderTrait,
{
    fn name(&self) -> &str {
        "Naive Journal Preprocessor"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let journal = &self.journal;
        let writing = &mut book;

        for topic in journal.each_topic() {
            let mut entries = journal.entries_for_topic(&topic.name())?;
            entries.sort_by(|a, b| b.created_at().cmp(a.created_at()));
            let book = topic_chapter(topic, &entries);
            writing.push_item(book);
        }

        Ok(book)
    }
}

fn topic_chapter(topic: &Topic, entries: &[Entry]) -> BookItem {
    let sub_items = entries.iter().map(|e| entry_chapter(topic, e)).collect();

    BookItem::Chapter(Chapter {
        sub_items,
        name: topic.name().to_owned(),
        ..Default::default()
    })
}

fn entry_chapter(topic: &Topic, entry: &Entry) -> BookItem {
    let name = match entry.meta_value(&"title") {
        Some(MetaValue::String(title)) => title.to_owned(),
        _ => String::from("Untitled"),
    };
    let content = entry.content().to_owned();

    BookItem::Chapter(Chapter {
        name,
        content,
        path: topic.virtual_path(entry).ok(),
        source_path: entry.file_location().cloned(),
        ..Default::default()
    })
}
