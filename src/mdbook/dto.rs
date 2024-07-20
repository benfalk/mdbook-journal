use super::prelude::*;
use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct TopicMapDto {
    pub data: BTreeMap<String, TopicDto>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TopicDto {
    pub virtual_root: Option<PathBuf>,
    pub source_root: Option<PathBuf>,
    pub variables: VariableMapDto,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct VariableMapDto {
    pub data: BTreeMap<String, VariableDto>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct VariableDto {
    pub required: bool,
    pub default: Option<String>,
}

impl TryFrom<&Config> for TopicMapDto {
    type Error = anyhow::Error;

    fn try_from(config: &Config) -> Result<Self> {
        if let Some(value) = config.get("preprocessor.journal.topics") {
            Ok(Self::deserialize(value.clone())?)
        } else {
            Ok(Default::default())
        }
    }
}

impl TryFrom<TopicMapDto> for TopicMap {
    type Error = anyhow::Error;

    fn try_from(value: TopicMapDto) -> Result<Self> {
        value
            .data
            .into_iter()
            .try_fold(Self::default(), |map, key_val| {
                map.insert(Topic::from(key_val))
            })
            .context("folding for TopicMap")
    }
}

impl From<(String, TopicDto)> for Topic {
    fn from((name, topic): (String, TopicDto)) -> Self {
        let mut builder = Topic::builder(name);

        if let Some(path) = topic.source_root {
            builder = builder.with_source_root(path);
        }

        if let Some(path) = topic.virtual_root {
            builder = builder.with_virtual_root(path);
        }

        for key_val in topic.variables.data.into_iter() {
            builder = builder.add_variable(Variable::from(key_val));
        }

        builder.build()
    }
}

impl From<(String, VariableDto)> for Variable {
    fn from((name, data): (String, VariableDto)) -> Self {
        let mut var = Variable::new(name);

        if data.required {
            var = var.required();
        }

        if let Some(value) = data.default {
            var = var.default(value);
        }

        var
    }
}
