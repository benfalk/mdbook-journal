use crate::prelude::*;

use handlebars::Handlebars;

#[derive(Debug)]
pub struct Template(Handlebars<'static>);

impl Default for Template {
    fn default() -> Self {
        Self::try_from("").unwrap()
    }
}

impl TryFrom<&str> for Template {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        let mut registry = Handlebars::new();
        registry.register_template_string("template", value)?;
        Ok(Self(registry))
    }
}

impl Template {
    pub fn generate_content(&self, entry: &Entry) -> Result<String> {
        let mut data = entry.meta().clone();
        data.insert(
            "CREATED_AT",
            MetaValue::String(entry.created_at().to_string()),
        );
        Ok(self.0.render("template", &data)?)
    }
}
