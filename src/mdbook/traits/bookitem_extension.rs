use super::*;

pub trait BookItemExtension {
    fn section_number(&self) -> Option<&SectionNumber>;
}

impl BookItemExtension for BookItem {
    fn section_number(&self) -> Option<&SectionNumber> {
        match self {
            Self::Chapter(Chapter {
                number: Some(section),
                ..
            }) => Some(section),
            _ => None,
        }
    }
}
