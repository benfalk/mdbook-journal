use crate::prelude::*;

use handlebars::Handlebars;

/// Topic Path Mapping
///
/// Understands how to map out where an
/// entry for each topic get's stored.
///
#[derive(Debug, Clone)]
pub struct PathMapping(Handlebars<'static>);

impl TryFrom<&str> for PathMapping {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        let mut registry = Handlebars::new();
        registry.register_template_string("path", value)?;
        Ok(Self(registry))
    }
}

impl PathMapping {
    pub fn map(&self, entry: &Entry) -> Result<PathBuf> {
        let mut data = entry.meta().clone();
        data.insert(
            "CREATED_AT",
            MetaValue::String(entry.created_at().to_string()),
        );
        let path_str = self.0.render("path", &data)?;
        let path_str = entry.created_at().format(&path_str).to_string();
        let mut path: PathBuf = path_str.into();
        path.set_extension("md");
        Ok(path)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::support::fixtures::*;
    use crate::support::prelude::*;

    #[rstest]
    fn mapping_works(entry: Entry) -> Result<()> {
        let mapping = PathMapping::try_from("%Y/%B/{{kebabCase title}}")?;
        let mapped = mapping.map(&entry)?;
        assert_eq!(PathBuf::from("2024/July/test-blog.md"), mapped);
        Ok(())
    }
}
