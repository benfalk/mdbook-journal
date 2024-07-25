use crate::prelude::*;

// This is the interface a topic needs in order
// to generate the required `Meta` for an `Entry`.
//
#[cfg_attr(test, automock)]
pub trait EntryGenerationTrait {
    fn created_at(&self) -> Result<UtcDateTime> {
        Ok(Utc::now())
    }

    fn collect_value(&self, variable: &Variable) -> Result<Option<MetaValue>>;
}
