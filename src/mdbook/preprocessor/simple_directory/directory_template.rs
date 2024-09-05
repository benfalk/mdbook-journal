use crate::prelude::*;
use handlebars::Handlebars;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct DirectoryTemplate(Handlebars<'static>);

impl Default for DirectoryTemplate {
    fn default() -> Self {
        DEFAULT_TEMPLATE.clone()
    }
}

pub static DEFAULT_TEMPLATE: Lazy<DirectoryTemplate> = Lazy::new(|| {
    let tmpl = include_str!("./index.hbs");
    DirectoryTemplate::try_from(tmpl)
        .with_context(|| format!("processing template:\n\n{tmpl}"))
        .unwrap()
});

impl TryFrom<&str> for DirectoryTemplate {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        let mut registry = Handlebars::new();
        registry.register_template_string("tpl", value)?;
        Ok(Self(registry))
    }
}

impl DirectoryTemplate {
    pub fn generate_content<T>(&self, data: &T) -> Result<String>
    where
        T: Serialize,
    {
        Ok(self.0.render("tpl", data)?)
    }
}
