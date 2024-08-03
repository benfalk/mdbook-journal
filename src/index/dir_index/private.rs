use super::*;

#[derive(Default)]
struct Parts {
    len: usize,
    buffers: Vec<String>,
}

impl Parts {
    fn load(&mut self, path: PathBuf) -> Result<()> {
        self.clear();
        self.prepare_size(path.components().count());
        for (idx, part) in path.components().enumerate() {
            let str_part = part
                .as_os_str()
                .to_str()
                .with_context(|| format!("converting segment for index [{}]", path.display()))?;
            let buffer = &mut self.buffers[idx];
            buffer.push_str(str_part);
        }
        Ok(())
    }

    fn clear(&mut self) {
        self.len = 0;
        self.buffers.iter_mut().for_each(String::clear);
    }

    fn prepare_size(&mut self, size: usize) {
        let current = self.buffers.len();
        if current < size {
            for _ in current..size {
                self.buffers.push(String::with_capacity(128));
            }
        }
        self.len = size;
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut String> {
        self.buffers[0..(self.len)].iter_mut()
    }

    fn iter(&self) -> impl Iterator<Item = &String> {
        self.buffers[0..(self.len)].iter()
    }
}

pub struct TopicParts<'a> {
    topic: &'a Topic,
    parts: Parts,
}

impl<'a> TopicParts<'a> {
    pub fn new(topic: &'a Topic) -> Self {
        Self {
            topic,
            parts: Default::default(),
        }
    }

    pub fn load(&mut self, entry: &Entry) -> Result<()> {
        let mut path = self.topic.virtual_path(entry)?;
        path.pop();
        self.parts.load(path)?;
        Ok(())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut String> {
        self.parts.iter_mut()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::support::fixtures::*;
    use crate::support::prelude::*;

    #[rstest]
    fn topic_parts_iter_functions_correctly(topic: Topic, entry: Entry) -> Result<()> {
        let mut parts = TopicParts::new(&topic);
        parts.load(&entry)?;
        let segments: Vec<String> = parts.iter_mut().map(|p| p.clone()).collect();
        assert_eq!(&vec!["test", "2024", "July"], &segments[..]);
        Ok(())
    }
}
