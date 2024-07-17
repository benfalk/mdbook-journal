pub use crate::prelude::*;

pub type DefaultValue = Arc<str>;
pub type KeyName = Arc<str>;

#[derive(Debug)]
pub struct Variable {
    key: KeyName,
    required: bool,
    default: Option<DefaultValue>,
}

#[derive(Debug, Default)]
pub struct VariableMap {
    data: BTreeMap<KeyName, Variable>,
}

impl Variable {
    pub fn new<S>(key: S) -> Self
    where
        S: Into<KeyName>,
    {
        Self {
            key: key.into(),
            required: false,
            default: None,
        }
    }

    pub fn required(self) -> Self {
        Self {
            required: true,
            ..self
        }
    }

    pub fn default<S>(self, value: S) -> Self
    where
        S: Into<DefaultValue>,
    {
        Self {
            default: Some(value.into()),
            ..self
        }
    }

    pub fn is_required(&self) -> bool {
        self.required
    }

    pub fn key(&self) -> &str {
        self.key.as_ref()
    }

    pub fn default_value(&self) -> Option<MetaValue> {
        self.default
            .as_ref()
            .map(ToString::to_string)
            .map(MetaValue::String)
    }
}

impl VariableMap {
    pub fn insert(&mut self, var: Variable) {
        self.data.insert(var.key.clone(), var);
    }

    pub fn get<K>(&self, key: &K) -> Option<&Variable>
    where
        K: AsRef<str>,
    {
        self.data.get(key.as_ref())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Variable> {
        self.data.values()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
