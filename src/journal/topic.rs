use crate::prelude::*;

mod builder;
mod map;
mod path_mapping;
mod template;
mod traits;
mod variables;

use crate::mdbook::preprocessor::DirectoryTemplate;
use path_mapping::PathMapping;
use template::Template;

pub mod cli_entry;
pub mod json_entry;

pub use builder::*;
pub use map::*;
pub use traits::*;
pub use variables::*;

pub type TopicName = Arc<str>;

/// Journal Topic
///
/// A topic represents a collection of similar entries
/// in a journal.
///
#[derive(Debug)]
pub struct Topic {
    /// unique string that identifies this topic
    name: TopicName,
    /// root location in the generated mdbook where
    /// this topic builds generated entries
    virtual_root: PathBuf,
    /// location relative to the mdbook `SUMMARY.md`
    /// where topic entries are saved on the disk.
    source_root: PathBuf,
    /// describes data that will be collected for
    /// a freshly created `Entry`
    variables: VariableMap,
    /// contains the logic for mapping an [Entry] to
    /// a specific file
    path_mapping: PathMapping,
    /// template used for the initial content when
    /// creating a new [Entry]
    content_template: Template,
    /// template used for a directory that contains
    /// entries only
    leaf_template: DirectoryTemplate,
}

impl Topic {
    pub(crate) fn builder<S>(name: S) -> TopicBuilder
    where
        S: Into<String>,
    {
        TopicBuilder::new(name)
    }

    pub fn virtual_path(&self, entry: &Entry) -> Result<PathBuf> {
        Ok(self.virtual_root.join(self.path_mapping.map(entry)?))
    }

    pub fn source_path(&self, entry: &Entry) -> Result<PathBuf> {
        Ok(self.source_root.join(self.path_mapping.map(entry)?))
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn generate_entry(&self, adapter: &dyn EntryGenerationTrait) -> Result<Entry> {
        let mut entry = Entry::builder(self.name());
        entry = entry.created_at(adapter.created_at()?);

        for var in self.variables.iter() {
            match adapter.collect_value(var)? {
                Some(value) => {
                    entry = entry.add_meta_value(var.key(), value);
                }
                None if var.is_required() => {
                    if let Some(value) = var.default_value() {
                        entry = entry.add_meta_value(var.key(), value);
                    } else {
                        bail!("{} is required", var.key())
                    }
                }
                None => continue,
            }
        }

        let content = self.content_template.generate_content(entry.as_ref())?;
        let entry = entry.content(content);
        Ok(entry.build())
    }

    pub fn directory_template(&self) -> &DirectoryTemplate {
        &self.leaf_template
    }

    pub fn source_root(&self) -> &PathBuf {
        &self.source_root
    }

    pub fn virtual_root(&self) -> &PathBuf {
        &self.virtual_root
    }
}
