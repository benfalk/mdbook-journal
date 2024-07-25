use super::*;
use crate::prelude::*;

static DEFAULT_MAPPING: Lazy<PathMapping> = Lazy::new(|| {
    PathMapping::try_from("%Y/%B/%d-%H-%M-%S-{{kebabCase title}}")
        .context("creating default topic mapping")
        .unwrap()
});

// Helper that allows dynamic overrides when
// constructing a `Topic`
//
#[derive(Debug)]
pub struct TopicBuilder {
    name: String,
    virtual_root: PathBuf,
    source_root: PathBuf,
    variables: VariableMap,
    path_mapping: Option<PathMapping>,
    template: Template,
}

impl TopicBuilder {
    pub(super) fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        let name = name.into();
        Self {
            virtual_root: name.clone().into(),
            source_root: name.clone().into(),
            variables: VariableMap::default(),
            path_mapping: None,
            template: Template::default(),
            name,
        }
    }

    pub fn with_source_root<S>(self, source_root: S) -> Self
    where
        S: Into<PathBuf>,
    {
        Self {
            source_root: source_root.into(),
            ..self
        }
    }

    pub fn with_virtual_root<S>(self, virtual_root: S) -> Self
    where
        S: Into<PathBuf>,
    {
        Self {
            virtual_root: virtual_root.into(),
            ..self
        }
    }

    pub fn with_path_mapping<M>(mut self, mapping: M) -> Result<Self>
    where
        M: AsRef<str>,
    {
        self.path_mapping = Some(mapping.as_ref().try_into()?);
        Ok(self)
    }

    pub fn with_template<T>(mut self, template: T) -> Result<Self>
    where
        T: AsRef<str>,
    {
        self.template = template.as_ref().try_into()?;
        Ok(self)
    }

    pub fn add_variable(mut self, var: Variable) -> Self {
        self.variables.insert(var);
        self
    }

    pub fn build(self) -> Topic {
        Topic {
            name: self.name.into(),
            source_root: self.source_root,
            virtual_root: self.virtual_root,
            variables: self.variables,
            path_mapping: self.path_mapping.unwrap_or_else(|| DEFAULT_MAPPING.clone()),
            template: self.template,
        }
    }
}
