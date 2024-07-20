use super::prelude::*;
use crate::prelude::*;

use ::toml_edit::{DocumentMut, Item, Table};

pub fn install(toml: &Path) -> Result<()> {
    private::edit_config(toml, |_config| {
        // Presently nothing to do... Letting the
        // config follow through ensures it is saved
        Ok(())
    })
}

pub fn load(toml: &Path) -> Result<Config> {
    Config::from_disk(toml)
}

mod private {
    use super::*;

    pub fn edit_config<F>(path: &Path, mut config: F) -> Result<()>
    where
        F: FnMut(&mut Item) -> Result<()>,
    {
        let mut doc = from_disk(path)?;
        let settings = preprocessor(&mut doc)?;
        config(settings)?;
        to_disk(path, &doc)?;
        Ok(())
    }

    fn from_disk(path: &Path) -> Result<DocumentMut> {
        std::fs::read_to_string(path)
            .with_context(|| format!("cannot read configuration file `{}`", path.display()))?
            .parse::<_>()
            .with_context(|| format!("invalid TOML format `{}`", path.display()))
    }

    fn to_disk(path: &Path, doc: &DocumentMut) -> Result<()> {
        use std::io::Write;
        let new_toml = doc.to_string();
        let mut file = std::fs::File::create(path)
            .with_context(|| format!("cannot open config file `{}`", path.display()))?;
        file.write_all(new_toml.as_bytes())
            .with_context(|| format!("cannot write config file `{}`", path.display()))?;
        Ok(())
    }

    fn preprocessor(doc: &mut DocumentMut) -> Result<&mut Item> {
        let doc = doc.as_table_mut();
        let empty_table = Item::Table(Table::default());
        let item = doc.entry("preprocessor").or_insert(empty_table.clone());
        let item = item
            .as_table_mut()
            .context("preprocessor not a table")?
            .entry("journal")
            .or_insert(empty_table);
        item["command"] = ::toml_edit::value("mdbook-journal");
        Ok(item)
    }
}
