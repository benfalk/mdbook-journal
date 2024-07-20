//
// Generate a MD
//
use mdbook_journal::{cli_entry, CliLoader, Journal};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let journal = Journal::<CliLoader>::load(PathBuf::default())?;
    let topic = journal.with_topic(&"code-blog")?;
    let entry = &topic.generate_entry(cli_entry::std_io())?;
    let path = journal.persist_entry(entry)?;
    println!("{path:?}");
    Ok(())
}
