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

#[fixture]
pub fn entries() -> Vec<Entry> {
    vec![
        Entry::builder("test")
            .add_meta_value("title", "June Blog")
            .created_at(Utc.with_ymd_and_hms(2024, 6, 15, 16, 20, 0).unwrap())
            .build(),
        Entry::builder("test")
            .add_meta_value("title", "July Blog")
            .created_at(Utc.with_ymd_and_hms(2024, 7, 15, 16, 20, 0).unwrap())
            .build(),
    ]
}

#[fixture]
pub fn topic() -> Topic {
    Topic::builder("test")
        .add_variable(Variable::new("title"))
        .build()
}
