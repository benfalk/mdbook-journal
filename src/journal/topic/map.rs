use super::TopicName;
use crate::prelude::*;

use std::collections::BTreeMap;

/// Journal Topic Map
///
/// Simple collection of topics indexed by name
///
#[derive(Debug, Default)]
pub struct TopicMap {
    map: BTreeMap<TopicName, Topic>,
}

impl TopicMap {
    pub(crate) fn insert(mut self, topic: Topic) -> Result<Self> {
        if self.map.contains_key(&topic.name) {
            bail!("Topic with key {} already taken!", &topic.name);
        }
        self.map.insert(topic.name.clone(), topic);
        Ok(self)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &Topic> {
        self.map.values()
    }

    pub(crate) fn find<S>(&self, name: &S) -> Option<&Topic>
    where
        S: AsRef<str>,
    {
        self.map.get(name.as_ref())
    }

    pub(crate) fn find_mut<S>(&mut self, name: &S) -> Option<&mut Topic>
    where
        S: AsRef<str>,
    {
        self.map.get_mut(name.as_ref())
    }
}
