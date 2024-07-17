use super::{Topic, Variable, VariableMap};
use std::path::PathBuf;

// Helper that allows dynamic overrides when
// constructing a `Topic`
//
#[derive(Debug)]
pub struct TopicBuilder {
    name: String,
    virtual_root: PathBuf,
    source_root: PathBuf,
    variables: VariableMap,
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
        }
    }
}
