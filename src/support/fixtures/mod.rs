use super::prelude::*;
use crate::prelude::*;

#[fixture]
pub fn entry() -> Entry {
    let date = Utc.with_ymd_and_hms(2024, 7, 15, 16, 20, 0).unwrap();

    Entry::builder("test")
        .add_meta_value("title", "Test Blog")
        .created_at(date)
        .build()
}
