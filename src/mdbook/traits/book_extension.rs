use super::*;

pub trait BookExtension {
    fn max_section_number(&self) -> Option<SectionNumber>;
}

impl BookExtension for Book {
    fn max_section_number(&self) -> Option<SectionNumber> {
        self.sections
            .iter()
            .max_by(|left, right| {
                let left = &root_num(left);
                let right = &root_num(right);
                left.cmp(right)
            })
            .and_then(|bookitem| bookitem.section_number().cloned())
    }
}

fn root_num(book: &BookItem) -> u32 {
    book.section_number()
        .and_then(|n| n.root_value())
        .unwrap_or_default()
}
