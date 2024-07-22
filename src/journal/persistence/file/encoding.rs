use crate::prelude::*;

pub fn encode(entry: &Entry) -> Result<String> {
    use std::fmt::Write;

    let mut data = String::with_capacity(256);
    data.push_str("---\n");
    writeln!(&mut data, "CREATED_AT: {}", entry.created_at().to_rfc3339())?;
    writeln!(&mut data, "TOPIC: {}", entry.topic_name())?;
    data.push_str(&serde_yaml::to_string(entry.meta())?);
    data.push_str("---\n");
    data.push_str(entry.content());
    Ok(data)
}

pub fn decode(string: String) -> Result<Entry> {
    use markdown_it_front_matter::FrontMatter;

    let parser = &mut markdown_it::MarkdownIt::new();
    markdown_it_front_matter::add(parser);
    let header = parser.parse(&string);
    let header = header.children.first().context("empty file")?;

    let front_matter = header
        .cast::<FrontMatter>()
        .context("expecting FrontMatter")?;

    let mut map: serde_yaml::Mapping = serde_yaml::from_str(&front_matter.content)?;

    // fetch created_at
    let created_at = DateTime::parse_from_rfc3339(
        map.remove("CREATED_AT")
            .context("expecting CREATED_AT")?
            .as_str()
            .context("expecting CREATED_AT str")?,
    )
    .context("expecting CREATED_AT valid format")?;
    let created_at = UtcDateTime::from(created_at);

    // fetch topic
    let topic = TopicName::from(
        map.remove("TOPIC")
            .context("expecting TOPIC")?
            .as_str()
            .context("expecting TOPIC str")?,
    );

    // gobbling up the meta
    let mut meta = EntryMeta::default();
    for (key, val) in map {
        if let MetaValue::String(key) = key {
            meta.insert(key, val);
        }
    }

    let (_, end) = header
        .srcmap
        .context("expecting header offser")?
        .get_byte_offsets();
    let content = String::from(&string[(end + 1)..]);

    Ok(Entry::builder(topic)
        .created_at(created_at)
        .content(content)
        .add_meta(meta)
        .build())
}
